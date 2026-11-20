use serde::{de::Error, Deserialize};
use serde_json::Value;
use std::collections::HashSet;

use crate::client_features::{ClientFeature, ClientFeatures, Meta, Query, Segment};

// One way trip - you use this to resolve a potentially partially broken set
// of client features from a serialized format. This will only deal with broken
// features. Bad state in any of the other fields will still cause parse failures.
// Think its unsafe to attempt to repair that state anyway, so imperfect will have to do
// If you're ever tempted to put serialize, clone or hash on this or expose any of the fields
// you're almost certainly doing something wrong
#[derive(Deserialize)]
pub struct WireClientsFeatures {
    version: u32,
    features: Vec<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    segments: Option<Vec<Segment>>,
    query: Option<Query>,
    #[serde(skip_serializing_if = "Option::is_none")]
    meta: Option<Meta>,
}

pub struct SanitizedFeatures {
    pub valid_features: ClientFeatures,
    pub rejected_features: HashSet<String>,
}

impl TryFrom<WireClientsFeatures> for SanitizedFeatures {
    type Error = serde_json::Error;

    fn try_from(wire: WireClientsFeatures) -> Result<Self, Self::Error> {
        let mut features = Vec::<ClientFeature>::with_capacity(wire.features.len());
        let mut rejections = HashSet::new();

        // This isn't worth the dependency of itertools nor the complexity of
        // a folding collect approach. Leave it alone, please
        for value in wire.features.into_iter() {
            let name = value
                .get("name")
                .and_then(Value::as_str)
                .ok_or_else(|| {
                    // What was passed is so broken that we can't even identify which features
                    // are borked. We're tapping out here and failing - error is correct here,
                    // the caller can't make use of this now since it can't identify which feature to
                    // cache evict. It either needs to ask for a full new set or rely on it's current
                    // state, neither of which is helped by continuing to parse at this point.
                    serde_json::Error::custom(format!(
                        "Feature has no valid name and parsing cannot recover, raw JSON: {value}"
                    ))
                })?
                .to_owned();

            match serde_json::from_value::<ClientFeature>(value) {
                Ok(feature) => features.push(feature),
                Err(_) => {
                    rejections.insert(name.clone());
                }
            }
        }

        Ok(SanitizedFeatures {
            valid_features: ClientFeatures {
                features,
                version: wire.version,
                segments: wire.segments,
                query: wire.query,
                meta: wire.meta,
            },
            rejected_features: rejections,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_rejects_entire_feature_set_if_a_feature_has_no_name() {
        let raw_json = r#"
        {
            "version": 1,
            "features": [
                {
                    "description": "A feature without a name"
                }
            ],
            "segments": null,
            "query": null,
            "meta": null
        }
        "#;

        let wire: WireClientsFeatures = serde_json::from_str(raw_json).unwrap();
        let snapshot = SanitizedFeatures::try_from(wire);
        assert!(snapshot.is_err());
    }

    #[test]
    fn test_an_unparsable_feature_does_not_reject_and_its_reported_in_invalid_features() {
        let raw_json = r#"
        {
            "version": 1,
            "features": [
                {
                    "name": "broken_feature",
                    "enabled": "broken because this is not a boolean"
                },
                {
                    "name": "healthy_feature",
                    "enabled": true
                }
            ],
            "segments": null,
            "query": null,
            "meta": null
        }
        "#;

        let wire: WireClientsFeatures =
            serde_json::from_str(raw_json).expect("Unparsable JSON, oops you broke the test");
        let snapshot =
            SanitizedFeatures::try_from(wire).expect("Bad snapshot, did you break type contract?");

        assert!(snapshot.rejected_features.contains("broken_feature"));
        assert!(!snapshot.rejected_features.contains("healthy_feature"));

        assert!(snapshot
            .valid_features
            .features
            .iter()
            .any(|f| f.name == "healthy_feature"));
    }

    // dumb and simple - we have examples we know should and do parse against
    // the built in deserializer, so we throw them at snapshot parsing and check
    // that they do the same thing, and they hash to the same result - the hashing
    // check is important because Edge relies on it to produce stable etags
    #[test_case("01-simple-examples.json")]
    #[test_case("08-variants.json")]
    #[test_case("13-constraint-operators.json")]
    #[test_case("14-constraint-semver-operators.json")]
    #[test_case("15-global-constraints.json")]
    #[test_case("16-strategy-variants.json")]
    #[test_case("17-dependent-features.json")]
    #[test_case("demofeatures.json")]
    #[test_case("features_with_variantType.json")]
    #[test_case("nuno-response.json")]
    fn smoke_tests(path: &str) {
        let raw_json = std::fs::read_to_string(format!("./examples/{}", &path))
            .expect("Failed to read example file");

        let parsed =
            serde_json::from_str::<ClientFeatures>(&raw_json).expect("Failed to parse JSON");

        let wire: WireClientsFeatures =
            serde_json::from_str(&raw_json).expect("Failed to parse JSON");

        let snapshot = SanitizedFeatures::try_from(wire).expect("Failed to create snapshot");
        let parsed_and_scrubbed = snapshot.valid_features;

        // We're trying to check that Edge will keep a stable etag here, not that
        // our xx3 hash is correctly upheld. Because Edge doesn't use the xx3 hash
        // implementation and instead goes directly via byte buffer hashing, there's
        // no point in validating against that. Means we also don't actually need a hash
        // byte level equivalence is enough.
        //
        // This is super fragile, and needs to be fixed upstream in Edge,
        // seems to be caused by the Axum rewrite
        let direct_byte_buffer = serde_json::to_vec(&parsed_and_scrubbed)
            .expect("Failed to serialize parsed and scrubbed features");
        let indirect_byte_buffer =
            serde_json::to_vec(&parsed).expect("Failed to serialize snapshot");

        assert_eq!(parsed, parsed_and_scrubbed);
        assert_eq!(direct_byte_buffer, indirect_byte_buffer);
    }
}
