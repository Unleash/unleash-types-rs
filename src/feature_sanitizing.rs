use serde::{de::Error, Deserialize};
use serde_json::Value;
use std::collections::HashSet;

use crate::client_features::{
    ClientFeature, ClientFeatures, ClientFeaturesDelta, DeltaEvent, Meta, Query, Segment,
};

enum ParseFeatureError {
    MissingName,
    InvalidFeature(String),
}

// One way trip - you use this to resolve a potentially partially broken set
// of client features from a serialized format. This will only deal with broken
// features. Bad state in any of the other fields will still cause parse failures.
// Think its unsafe to attempt to repair that state anyway, so imperfect will have to do
// If you're ever tempted to put serialize, clone or hash on this or expose any of the fields
// you're almost certainly doing something wrong
#[derive(Deserialize)]
pub struct WireClientFeatures {
    version: u32,
    features: Vec<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    segments: Option<Vec<Segment>>,
    query: Option<Query>,
    #[serde(skip_serializing_if = "Option::is_none")]
    meta: Option<Meta>,
}

#[derive(Deserialize)]
struct WireClientFeaturesDelta {
    events: Vec<Value>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct WireHydration {
    event_id: u32,
    features: Vec<Value>,
    segments: Vec<Segment>,
}

pub struct SanitizedFeatures {
    pub valid_features: ClientFeatures,
    pub rejected_features: HashSet<String>,
}

pub struct SanitizedClientFeatureDelta {
    pub valid_events: ClientFeaturesDelta,
    pub rejected_features: HashSet<String>,
}

pub struct SanitizedClientFeaturesDelta {
    pub delta_set: ClientFeaturesDelta,
    pub rejected_features: HashSet<String>,
}

impl DeltaEvent {
    fn feature_touched(&self) -> Option<&str> {
        match self {
            DeltaEvent::FeatureUpdated { feature, .. } => Some(&feature.name),
            DeltaEvent::FeatureRemoved {
                feature_name: name, ..
            } => Some(name),
            _ => None,
        }
    }
}

impl TryFrom<WireClientFeatures> for SanitizedFeatures {
    type Error = serde_json::Error;

    fn try_from(wire: WireClientFeatures) -> Result<Self, Self::Error> {
        let mut features = Vec::<ClientFeature>::with_capacity(wire.features.len());
        let mut rejections = HashSet::new();

        // This isn't worth the dependency of itertools nor the complexity of
        // a folding collect approach. Leave it alone, please
        for value in wire.features.into_iter() {
            match parse_feature(value) {
                Ok(feature) => features.push(feature),
                Err(ParseFeatureError::MissingName) => {
                    // If the feature has no name, we reject the entire set.
                    // It's not safe to continue now, and making assumptions here
                    // is dangerous. The caller can make a decision about whether
                    // or not to request a new feature set or fall back to a known good state
                    return Err(serde_json::Error::custom(
                        "Feature has no valid name and parsing cannot recover",
                    ));
                }
                Err(ParseFeatureError::InvalidFeature(name)) => {
                    rejections.insert(name);
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

impl TryFrom<WireClientFeaturesDelta> for SanitizedClientFeaturesDelta {
    type Error = serde_json::Error;

    fn try_from(wire: WireClientFeaturesDelta) -> Result<Self, Self::Error> {
        let mut valid_events = Vec::with_capacity(wire.events.len());
        let mut rejected_features = HashSet::new();

        for raw in wire.events {
            let event = if raw.get("type").and_then(Value::as_str) == Some("hydration") {
                let hydration: WireHydration = serde_json::from_value(raw)?;
                let (features, rejections) = parse_hydration_features(hydration.features)?;

                rejected_features = rejections;

                DeltaEvent::Hydration {
                    event_id: hydration.event_id,
                    features,
                    segments: hydration.segments,
                }
            } else {
                serde_json::from_value::<DeltaEvent>(raw)?
            };

            // An event has overridden a corrupted feature in the original hydration event
            // In this case, we can know that, while the hydration event is corrupted, the
            // override unambiguously repairs the state since it's a full override.
            // This shouldn't happen because we shouldn't get hydration events that are
            // mixed in with deltas. But given the data structure allows it, it's going to happen
            if let Some(feature_name) = event.feature_touched() {
                rejected_features.remove(feature_name);
            }

            valid_events.push(event);
        }
        Ok(SanitizedClientFeaturesDelta {
            delta_set: ClientFeaturesDelta {
                events: valid_events,
            },
            rejected_features,
        })
    }
}

fn parse_hydration_features(
    raw: Vec<Value>,
) -> Result<(Vec<ClientFeature>, HashSet<String>), serde_json::Error> {
    let mut features = Vec::with_capacity(raw.len());
    let mut rejections = HashSet::new();

    for value in raw {
        match parse_feature(value) {
            Ok(feature) => features.push(feature),
            Err(ParseFeatureError::InvalidFeature(name)) => {
                rejections.insert(name);
            }
            Err(ParseFeatureError::MissingName) => {
                return Err(serde_json::Error::custom("feature missing name"));
            }
        }
    }

    Ok((features, rejections))
}

fn parse_feature(value: Value) -> Result<ClientFeature, ParseFeatureError> {
    let name = value
        .get("name")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            // What was passed is so broken that we can't even identify which feature
            // is broken. This is a special error case that the caller needs to know about
            // because it makes recovery impossible
            ParseFeatureError::MissingName
        })?
        .to_owned();

    serde_json::from_value::<ClientFeature>(value)
        .map_err(|_| ParseFeatureError::InvalidFeature(name))
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

        let wire: WireClientFeatures = serde_json::from_str(raw_json).unwrap();
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

        let wire: WireClientFeatures =
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

        let wire: WireClientFeatures =
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

    #[test]
    fn test_hydration_event_containing_a_feature_with_a_missing_name_is_rejected() {
        let raw_json = r#"
        {
            "events": [
                {
                    "eventId": 1,
                    "type": "hydration",
                    "segments": [],
                    "features": [
                        {
                            "enabled": true
                        }
                    ]
                }
            ]
        }
        "#;

        let parsed = serde_json::from_str::<WireClientFeaturesDelta>(&raw_json);
        let maybe_sanitized =
            SanitizedClientFeaturesDelta::try_from(parsed.expect("Failed to parse JSON"));

        assert!(
            maybe_sanitized.is_err(),
            "Expected the feature with a missing name to be rejected"
        );
    }

    #[test]
    fn test_hydration_event_containing_a_bad_feature_with_a_name_is_flagged_as_invalid_but_passes_sanitization(
    ) {
        let raw_json = r#"
        {
            "events": [
                {
                    "eventId": 1,
                    "type": "hydration",
                    "segments": [],
                    "features": [
                        {
                            "name": "test_name",
                            "enabled": "This should be a boolean"
                        }
                    ]
                }
            ]
        }
        "#;

        let parsed = serde_json::from_str::<WireClientFeaturesDelta>(&raw_json)
            .expect("Expected parsing to succeed");

        let maybe_sanitized = SanitizedClientFeaturesDelta::try_from(parsed);

        assert!(
            maybe_sanitized.is_ok(),
            "Expected sanitization to succeed even with a bad feature"
        );
        assert!(
            maybe_sanitized.unwrap().rejected_features.len() == 1,
            "Expected the bad feature to be flagged as invalid"
        );
    }

    #[test]
    fn test_hydration_event_does_not_flag_feature_when_fixed_by_subsequent_event() {
        let raw_json = r#"
        {
            "events": [
                {
                    "eventId": 1,
                    "type": "hydration",
                    "segments": [],
                    "features": [
                        {
                            "name": "test_name",
                            "enabled": "This should be a boolean"
                        }
                    ]
                },
                {
                    "eventId": 2,
                    "type": "hydration",
                    "segments": [],
                    "features": [
                        {
                            "name": "test_name",
                            "enabled": true
                        }
                    ]
                }
            ]
        }
        "#;

        let parsed = serde_json::from_str::<WireClientFeaturesDelta>(&raw_json)
            .expect("Expected parsing to succeed");

        let maybe_sanitized = SanitizedClientFeaturesDelta::try_from(parsed);

        assert!(
            maybe_sanitized.is_ok(),
            "Expected sanitization to succeed even with a bad feature fixed by subsequent event"
        );
        assert!(
            maybe_sanitized.unwrap().rejected_features.is_empty(),
            "Expected no features to be rejected since the bad feature was fixed"
        );
    }

    #[test]
    fn test_a_bad_non_hydration_event_fails_to_parse() {
        let raw_json = r#"
        {
            "events": [
                {
                    "eventId": 1,
                    "type": "feature-updated",
                    "segments": [],
                    "features": [
                        {
                            "name": "test_name",
                            "enabled": "This should be a boolean"
                        }
                    ]
                }
            ]
        }
        "#;

        let maybe_parsed: Result<SanitizedClientFeaturesDelta, _> =
            serde_json::from_str::<WireClientFeaturesDelta>(&raw_json)
                .expect("Rejected by serde")
                .try_into();

        assert!(
            maybe_parsed.is_err(),
            "Expected parsing to fail for a bad non-hydration event"
        );
    }

    #[test_case("delta_base.json")]
    #[test_case("delta_patch.json")]
    fn delta_smoke_tests(path: &str) {
        let raw_json = std::fs::read_to_string(format!("./examples/{}", &path))
            .expect("Failed to read example file");

        SanitizedClientFeaturesDelta::try_from(
            serde_json::from_str::<WireClientFeaturesDelta>(&raw_json).expect("Rejected by serde"),
        )
        .expect("Failed to sanitize client features delta");
    }
}
