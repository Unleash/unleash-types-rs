#[cfg(feature = "hashes")]
use base64::Engine;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::{cmp::Ordering, collections::BTreeMap};
#[cfg(feature = "openapi")]
use utoipa::{IntoParams, ToSchema};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
#[cfg(feature = "hashes")]
use xxhash_rust::xxh3::xxh3_128;

use crate::{Deduplicate, Merge, Upsert};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(ToSchema, IntoParams))]
#[serde(rename_all = "camelCase")]
pub struct Query {
    pub tags: Option<Vec<Vec<String>>>,
    pub projects: Option<Vec<String>>,
    pub name_prefix: Option<String>,
    pub environment: Option<String>,
    pub inline_segment_constraints: Option<bool>,
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Operator {
    NotIn,
    In,
    StrEndsWith,
    StrStartsWith,
    StrContains,
    NumEq,
    NumGt,
    NumGte,
    NumLt,
    NumLte,
    DateAfter,
    DateBefore,
    SemverEq,
    SemverLt,
    SemverGt,
    Unknown(String),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(ToSchema, IntoParams))]
#[serde(rename_all = "camelCase")]
pub struct Context {
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub environment: Option<String>,
    pub app_name: Option<String>,
    pub current_time: Option<String>,
    pub remote_address: Option<String>,
    #[serde(default)]
    #[serde(
        deserialize_with = "remove_null_properties",
        serialize_with = "optional_ordered_map"
    )]
    pub properties: Option<HashMap<String, String>>,
}

// I know this looks silly but it's also important for two reasons:
// The first is that the client spec tests have a test case that has a context defined like:
// {
//   "properties": {
//      "someValue": null
//    }
// }
// Passing around an Option<HashMap<String, Option<String>>> is awful and unnecessary, we should scrub ingested data
// before trying to execute our logic, so we scrub out those empty values instead, they do nothing useful for us.
// The second reason is that we can't shield the Rust code from consumers using the FFI layers and potentially doing
// exactly the same thing in languages that allow it. They should not do that. But if they do we have enough information
// to understand the intent of the executed code clearly and there's no reason to fail
fn remove_null_properties<'de, D>(
    deserializer: D,
) -> Result<Option<HashMap<String, String>>, D::Error>
where
    D: Deserializer<'de>,
{
    let props: Option<HashMap<String, Option<String>>> = Option::deserialize(deserializer)?;
    Ok(props.map(|props| {
        props
            .into_iter()
            .filter(|x| x.1.is_some())
            .map(|x| (x.0, x.1.unwrap()))
            .collect()
    }))
}

///
/// We need this to ensure that ClientFeatures gets a deterministic serialization.
fn optional_ordered_map<S>(
    value: &Option<HashMap<String, String>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(m) => {
            let ordered: BTreeMap<_, _> = m.iter().collect();
            ordered.serialize(serializer)
        }
        None => serializer.serialize_none(),
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            user_id: None,
            session_id: None,
            environment: None,
            current_time: None,
            app_name: None,
            remote_address: None,
            properties: Some(HashMap::new()),
        }
    }
}

impl<'de> Deserialize<'de> for Operator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "NOT_IN" => Operator::NotIn,
            "IN" => Operator::In,
            "STR_ENDS_WITH" => Operator::StrEndsWith,
            "STR_STARTS_WITH" => Operator::StrStartsWith,
            "STR_CONTAINS" => Operator::StrContains,
            "NUM_EQ" => Operator::NumEq,
            "NUM_GT" => Operator::NumGt,
            "NUM_GTE" => Operator::NumGte,
            "NUM_LT" => Operator::NumLt,
            "NUM_LTE" => Operator::NumLte,
            "DATE_AFTER" => Operator::DateAfter,
            "DATE_BEFORE" => Operator::DateBefore,
            "SEMVER_EQ" => Operator::SemverEq,
            "SEMVER_LT" => Operator::SemverLt,
            "SEMVER_GT" => Operator::SemverGt,
            _ => Operator::Unknown(s),
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct Constraint {
    pub context_name: String,
    pub operator: Operator,
    #[serde(default)]
    pub case_insensitive: bool,
    #[serde(default)]
    pub inverted: bool,
    pub values: Option<Vec<String>>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "camelCase")]
pub enum WeightType {
    Fix,
    Variable,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct Strategy {
    pub name: String,
    pub sort_order: Option<i32>,
    pub segments: Option<Vec<i32>>,
    pub constraints: Option<Vec<Constraint>>,
    #[serde(serialize_with = "optional_ordered_map")]
    pub parameters: Option<HashMap<String, String>>,
}

impl PartialEq for Strategy {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.sort_order == other.sort_order
            && self.segments == other.segments
            && self.constraints == other.constraints
            && self.parameters == other.parameters
    }
}
impl PartialOrd for Strategy {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.sort_order.partial_cmp(&other.sort_order) {
            Some(core::cmp::Ordering::Equal) => self.name.partial_cmp(&other.name),
            Some(s) => Some(s),
            None => self.name.partial_cmp(&other.name),
        }
    }
}
impl Ord for Strategy {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.sort_order.cmp(&other.sort_order) {
            Ordering::Equal => self.name.cmp(&other.name),
            ord => ord,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct Override {
    pub context_name: String,
    pub values: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct Payload {
    #[serde(rename = "type")]
    pub payload_type: String,
    pub value: String,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct Variant {
    pub name: String,
    pub weight: i32,
    pub weight_type: Option<WeightType>,
    pub stickiness: Option<String>,
    pub payload: Option<Payload>,
    pub overrides: Option<Vec<Override>>,
}

impl PartialOrd for Variant {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.name.partial_cmp(&other.name)
    }
}
impl Ord for Variant {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct Segment {
    pub id: i32,
    pub constraints: Vec<Constraint>,
}

impl PartialEq for Segment {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialOrd for Segment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Ord for Segment {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl Hash for Segment {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Default)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct ClientFeature {
    pub name: String,
    #[serde(rename = "type")]
    pub feature_type: Option<String>,
    pub description: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub last_seen_at: Option<DateTime<Utc>>,
    pub enabled: bool,
    pub stale: Option<bool>,
    pub impression_data: Option<bool>,
    pub project: Option<String>,
    pub strategies: Option<Vec<Strategy>>,
    pub variants: Option<Vec<Variant>>,
}

impl Merge for ClientFeatures {
    fn merge(self, other: Self) -> Self {
        let mut features = self.features.merge(other.features);
        features.sort();
        let segments = match (self.segments, other.segments) {
            (Some(mut s), Some(o)) => {
                s.extend(o);
                Some(s.deduplicate())
            }
            (Some(s), None) => Some(s),
            (None, Some(o)) => Some(o),
            (None, None) => None,
        };
        ClientFeatures {
            version: self.version.max(other.version),
            features,
            segments: segments.map(|mut s| {
                s.sort();
                s
            }),
            query: self.query.or(other.query),
        }
    }
}

impl Upsert for ClientFeatures {
    fn upsert(self, other: Self) -> Self {
        let mut features = self.features.upsert(other.features);
        features.sort();
        let segments = match (self.segments, other.segments) {
            (Some(mut s), Some(o)) => {
                s.extend(o);
                Some(s.deduplicate())
            }
            (Some(s), None) => Some(s),
            (None, Some(o)) => Some(o),
            (None, None) => None,
        };
        ClientFeatures {
            version: self.version.max(other.version),
            features,
            segments: segments.map(|mut s| {
                s.sort();
                s
            }),
            query: self.query.or(other.query),
        }
    }
}

impl PartialOrd for ClientFeature {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl Ord for ClientFeature {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialEq for ClientFeature {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Hash for ClientFeature {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct ClientFeatures {
    pub version: u32,
    pub features: Vec<ClientFeature>,
    pub segments: Option<Vec<Segment>>,
    pub query: Option<Query>,
}

#[cfg(feature = "hashes")]
impl ClientFeatures {
    ///
    /// Returns a base64 encoded xx3_128 hash for the json representation of ClientFeatures
    ///
    pub fn xx3_hash(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
            .map(|s| xxh3_128(s.as_bytes()))
            .map(|xxh_hash| base64::prelude::BASE64_URL_SAFE.encode(xxh_hash.to_le_bytes()))
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader, path::PathBuf};

    use crate::{client_features::ClientFeature, Merge, Upsert};

    use super::{ClientFeatures, Constraint, Strategy};
    use test_case::test_case;

    #[derive(Debug)]
    pub enum EdgeError {
        SomethingWentWrong(String),
    }
    #[test]
    pub fn ordering_is_stable_for_constraints() {
        let c1 = Constraint {
            context_name: "acontext".into(),
            operator: super::Operator::DateAfter,
            case_insensitive: true,
            inverted: false,
            values: Some(vec![]),
            value: None,
        };
        let c2 = Constraint {
            context_name: "acontext".into(),
            operator: super::Operator::DateBefore,
            case_insensitive: false,
            inverted: false,
            values: None,
            value: Some("value".into()),
        };
        let c3 = Constraint {
            context_name: "bcontext".into(),
            operator: super::Operator::NotIn,
            case_insensitive: false,
            inverted: false,
            values: None,
            value: None,
        };
        let mut v = vec![c3.clone(), c1.clone(), c2.clone()];
        v.sort();
        assert_eq!(v, vec![c1, c2, c3]);
    }

    fn read_file(path: PathBuf) -> Result<BufReader<File>, EdgeError> {
        File::open(path)
            .map_err(|e| EdgeError::SomethingWentWrong(e.to_string()))
            .map(BufReader::new)
    }

    #[test_case("./examples/features_with_variantType.json".into() ; "features with variantType")]
    #[test_case("./examples/15-global-constraints.json".into(); "global-constraints")]
    pub fn client_features_parsing_is_stable(path: PathBuf) {
        let client_features: ClientFeatures =
            serde_json::from_reader(read_file(path).unwrap()).unwrap();

        let to_string = serde_json::to_string(&client_features).unwrap();
        let reparsed_to_string: ClientFeatures = serde_json::from_str(to_string.as_str()).unwrap();
        assert_eq!(client_features, reparsed_to_string);
    }

    #[cfg(feature = "hashes")]
    #[test_case("./examples/features_with_variantType.json".into() ; "features with variantType")]
    #[cfg(feature = "hashes")]
    #[test_case("./examples/15-global-constraints.json".into(); "global-constraints")]
    pub fn client_features_hashing_is_stable(path: PathBuf) {
        let client_features: ClientFeatures =
            serde_json::from_reader(read_file(path.clone()).unwrap()).unwrap();

        let second_read: ClientFeatures =
            serde_json::from_reader(read_file(path).unwrap()).unwrap();

        let first_hash = client_features.xx3_hash().unwrap();
        let second_hash = client_features.xx3_hash().unwrap();
        assert_eq!(first_hash, second_hash);

        let first_hash_from_second_read = second_read.xx3_hash().unwrap();
        assert_eq!(first_hash, first_hash_from_second_read);
    }

    #[test]
    fn merging_two_client_features_takes_both_feature_sets() {
        let client_features_one = ClientFeatures {
            version: 2,
            features: vec![
                ClientFeature {
                    name: "feature1".into(),
                    ..ClientFeature::default()
                },
                ClientFeature {
                    name: "feature2".into(),
                    ..ClientFeature::default()
                },
            ],
            segments: None,
            query: None,
        };

        let client_features_two = ClientFeatures {
            version: 2,
            features: vec![ClientFeature {
                name: "feature3".into(),
                ..ClientFeature::default()
            }],
            segments: None,
            query: None,
        };

        let merged = client_features_one.merge(client_features_two);
        assert_eq!(merged.features.len(), 3);
    }

    #[test]
    fn upserting_client_features_prioritizes_new_data_but_keeps_uniques() {
        let client_features_one = ClientFeatures {
            version: 2,
            features: vec![
                ClientFeature {
                    name: "feature1".into(),
                    ..ClientFeature::default()
                },
                ClientFeature {
                    name: "feature2".into(),
                    ..ClientFeature::default()
                },
            ],
            segments: None,
            query: None,
        };
        let mut updated_strategies = client_features_one.clone();
        let updated_feature_one_with_strategy = ClientFeature {
            name: "feature1".into(),
            strategies: Some(vec![Strategy {
                name: "default".into(),
                sort_order: Some(124),
                segments: None,
                constraints: None,
                parameters: None,
            }]),
            ..ClientFeature::default()
        };
        let feature_the_third = ClientFeature {
            name: "feature3".into(),
            strategies: Some(vec![Strategy {
                name: "default".into(),
                sort_order: Some(124),
                segments: None,
                constraints: None,
                parameters: None,
            }]),
            ..ClientFeature::default()
        };
        updated_strategies.features = vec![updated_feature_one_with_strategy, feature_the_third];
        let updated_features = client_features_one.upsert(updated_strategies);
        let client_features = updated_features.features;
        assert_eq!(client_features.len(), 3);
        let updated_feature_one = client_features
            .iter()
            .find(|f| f.name == "feature1")
            .unwrap();
        assert_eq!(updated_feature_one.strategies.as_ref().unwrap().len(), 1);
        assert!(client_features.iter().any(|f| f.name == "feature3"));
        assert!(client_features.iter().any(|f| f.name == "feature2"));
    }
}
