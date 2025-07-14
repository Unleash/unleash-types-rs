#[cfg(feature = "hashes")]
use base64::Engine;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::{cmp::Ordering, collections::BTreeMap};
#[cfg(feature = "openapi")]
use utoipa::{IntoParams, ToSchema};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
#[cfg(feature = "hashes")]
use xxhash_rust::xxh3::xxh3_128;

use crate::{Deduplicate, Merge, Upsert};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "openapi", derive(ToSchema, IntoParams))]
#[serde(rename_all = "camelCase")]
pub struct Query {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
#[cfg_attr(feature = "openapi", into_params(style = Form, parameter_in = Query))]
#[serde(rename_all = "camelCase")]
pub struct Context {
    #[serde(default)]
    #[serde(
        deserialize_with = "stringify_numbers_and_booleans",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_id: Option<String>,
    #[serde(default)]
    #[serde(
        deserialize_with = "stringify_numbers_and_booleans",
        skip_serializing_if = "Option::is_none"
    )]
    pub session_id: Option<String>,
    #[serde(default)]
    #[serde(
        deserialize_with = "stringify_numbers_and_booleans",
        skip_serializing_if = "Option::is_none"
    )]
    pub environment: Option<String>,
    #[serde(default)]
    #[serde(
        deserialize_with = "stringify_numbers_and_booleans",
        skip_serializing_if = "Option::is_none"
    )]
    pub app_name: Option<String>,
    #[serde(default)]
    #[serde(
        deserialize_with = "stringify_numbers_and_booleans",
        skip_serializing_if = "Option::is_none"
    )]
    pub current_time: Option<String>,
    #[serde(default)]
    #[serde(
        deserialize_with = "stringify_numbers_and_booleans",
        skip_serializing_if = "Option::is_none"
    )]
    pub remote_address: Option<String>,
    #[serde(default)]
    #[serde(
        deserialize_with = "stringify_numbers_and_booleans_remove_nulls_and_non_strings",
        serialize_with = "optional_ordered_map",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "openapi", param(style = Form, explode = false, value_type = Object))]
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
// to understand the intent of the executed code clearly and there's no reason to fail.
// This also maps numbers + booleans to strings, and disregards other types without failing
fn stringify_numbers_and_booleans_remove_nulls_and_non_strings<'de, D>(
    deserializer: D,
) -> Result<Option<HashMap<String, String>>, D::Error>
where
    D: Deserializer<'de>,
{
    let props: Option<HashMap<String, Option<Value>>> = Option::deserialize(deserializer)?;
    Ok(props.map(|props| {
        props
            .into_iter()
            .filter_map(|(k, v)| match v {
                Some(Value::String(s)) => {
                    if s.is_empty() {
                        None
                    } else {
                        Some((k, s))
                    }
                }
                Some(Value::Number(n)) => Some((k, n.to_string())),
                Some(Value::Bool(b)) => Some((k, b.to_string())),
                _ => None,
            })
            .collect()
    }))
}

// Allowing a looser deserialization for the contexts properties to match Unleash behavior
fn stringify_numbers_and_booleans<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let prop: Option<Value> = Option::deserialize(deserializer)?;
    Ok(match prop {
        Some(Value::String(s)) => {
            if s.is_empty() {
                None
            } else {
                Some(s)
            }
        }
        Some(Value::Number(n)) => Some(n.to_string()),
        Some(Value::Bool(b)) => Some(b.to_string()),
        _ => None,
    })
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<Vec<Constraint>>,
    #[serde(
        serialize_with = "optional_ordered_map",
        skip_serializing_if = "Option::is_none"
    )]
    pub parameters: Option<HashMap<String, String>>,
    #[serde(serialize_with = "serialize_option_vec")]
    pub variants: Option<Vec<StrategyVariant>>,
}

fn serialize_option_vec<S, T>(value: &Option<Vec<T>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    match value {
        Some(ref v) => v.serialize(serializer),
        None => Vec::<T>::new().serialize(serializer),
    }
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
        Some(self.cmp(other))
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_type: Option<WeightType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stickiness: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Payload>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<Vec<Override>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct StrategyVariant {
    pub name: String,
    pub weight: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Payload>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stickiness: Option<String>,
}

impl PartialOrd for Variant {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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
        Some(self.cmp(other))
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct FeatureDependency {
    pub feature: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variants: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Default)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct ClientFeature {
    pub name: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub feature_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seen_at: Option<DateTime<Utc>>,
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stale: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impression_data: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategies: Option<Vec<Strategy>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variants: Option<Vec<Variant>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<FeatureDependency>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub etag: Option<String>,
    pub revision_id: Option<usize>,
    pub query_hash: Option<String>,
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
            meta: other.meta.or(self.meta),
        }
    }
}

impl Upsert for ClientFeatures {
    fn upsert(self, other: Self) -> Self {
        let mut features = self.features.upsert(other.features);
        features.sort();
        let segments = match (self.segments, other.segments) {
            (Some(s), Some(mut o)) => {
                o.extend(s);
                Some(o.deduplicate())
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
            meta: other.meta.or(self.meta),
        }
    }
}

impl PartialOrd for ClientFeature {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<Segment>>,
    pub query: Option<Query>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "kebab-case")]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub enum DeltaEvent {
    /// Event for a feature update.
    FeatureUpdated {
        #[serde(rename = "eventId")]
        event_id: u32,
        feature: ClientFeature,
    },
    /// Event for a feature removal.
    #[serde(rename_all = "camelCase")]
    FeatureRemoved {
        event_id: u32,
        feature_name: String,
        project: String,
    },
    /// Event for a segment update.
    SegmentUpdated {
        #[serde(rename = "eventId")]
        event_id: u32,
        segment: Segment,
    },
    /// Event for a segment removal.
    #[serde(rename_all = "camelCase")]
    SegmentRemoved { event_id: u32, segment_id: i32 },
    /// Hydration event for features and segments.
    Hydration {
        #[serde(rename = "eventId")]
        event_id: u32,
        features: Vec<ClientFeature>,
        segments: Vec<Segment>,
    },
}

impl DeltaEvent {
    pub fn get_event_id(&self) -> u32 {
        match self {
            DeltaEvent::FeatureUpdated { event_id, .. }
            | DeltaEvent::FeatureRemoved { event_id, .. }
            | DeltaEvent::SegmentUpdated { event_id, .. }
            | DeltaEvent::SegmentRemoved { event_id, .. }
            | DeltaEvent::Hydration { event_id, .. } => *event_id,
        }
    }
}

/// Schema for delta updates of feature configurations.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct ClientFeaturesDelta {
    /// A list of delta events.
    pub events: Vec<DeltaEvent>,
}

impl ClientFeatures {
    /// Modifies the current ClientFeatures instance by applying the events.
    pub fn apply_delta(&mut self, delta: &ClientFeaturesDelta) {
        self.apply_delta_events(delta);
    }

    /// Returns a new ClientFeatures instance with the events applied.
    pub fn create_from_delta(delta: &ClientFeaturesDelta) -> ClientFeatures {
        let mut client_features = ClientFeatures::default();
        client_features.apply_delta_events(delta);
        client_features
    }

    fn apply_delta_events(&mut self, delta: &ClientFeaturesDelta) {
        let segments = &mut self.segments;
        let features = &mut self.features;
        for event in &delta.events {
            match event {
                DeltaEvent::FeatureUpdated { feature, .. } => {
                    if let Some(existing) = features.iter_mut().find(|f| f.name == feature.name) {
                        *existing = feature.clone();
                    } else {
                        features.push(feature.clone());
                    }
                }
                DeltaEvent::FeatureRemoved { feature_name, .. } => {
                    features.retain(|f| f.name != *feature_name);
                }
                DeltaEvent::SegmentUpdated { segment, .. } => {
                    let segments_list = segments.get_or_insert_with(Vec::new);
                    if let Some(existing) = segments_list.iter_mut().find(|s| s.id == segment.id) {
                        *existing = segment.clone();
                    } else {
                        segments_list.push(segment.clone());
                    }
                }
                DeltaEvent::SegmentRemoved { segment_id, .. } => {
                    if let Some(segments_list) = segments {
                        segments_list.retain(|s| s.id != *segment_id);
                    }
                }
                DeltaEvent::Hydration {
                    features: new_features,
                    segments: new_segments,
                    ..
                } => {
                    *features = new_features.clone();
                    *segments = Some(new_segments.clone());
                }
            }
        }

        features.sort();
    }
}

impl Default for ClientFeatures {
    fn default() -> Self {
        Self {
            version: 2,
            features: vec![],
            segments: None,
            query: None,
            meta: None,
        }
    }
}

impl From<ClientFeaturesDelta> for ClientFeatures {
    fn from(value: ClientFeaturesDelta) -> Self {
        ClientFeatures::create_from_delta(&value)
    }
}

impl From<&ClientFeaturesDelta> for ClientFeatures {
    fn from(value: &ClientFeaturesDelta) -> Self {
        ClientFeatures::create_from_delta(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        client_features::{ClientFeature, ClientFeaturesDelta},
        Merge, Upsert,
    };
    use serde_json::{from_reader, to_string};
    use serde_qs::Config;
    use std::{fs::File, io::BufReader, path::PathBuf};

    use super::{ClientFeatures, Constraint, DeltaEvent, Operator, Segment, Strategy};
    use crate::client_features::Context;
    use test_case::test_case;

    #[derive(Debug)]
    pub enum EdgeError {
        SomethingWentWrong,
    }
    #[test]
    pub fn can_deserialize_numbers_to_strings() {
        let json = serde_json::json!({
            "context": {
                "userId": 123123,
                "sessionId": false,
                "environment": {
                    "aKey": "aValue",
                },
                "appName": "name",
                "currentTime": null,
                "properties": {
                    "someValue": 123,
                    "otherValue": null,
                    "anotherValue": {
                        "someKey": 123,
                    },
                    "boolProp": true,
                }
            },
        });
        let context: Context = serde_json::from_value(json["context"].clone()).unwrap();
        assert_eq!(context.user_id.unwrap(), "123123");
        assert_eq!(context.session_id.unwrap(), "false");
        assert_eq!(context.app_name.unwrap(), "name");
        assert!(context.current_time.is_none());
        assert!(context.environment.is_none());
        assert!(context.remote_address.is_none());
        assert_eq!(
            context
                .properties
                .clone()
                .unwrap()
                .get("someValue")
                .unwrap(),
            "123"
        );
        assert_eq!(
            context.properties.clone().unwrap().get("boolProp").unwrap(),
            "true"
        );
        assert!(!context
            .properties
            .clone()
            .unwrap()
            .contains_key("otherValue"));
        assert!(!context
            .properties
            .clone()
            .unwrap()
            .contains_key("anotherValue"));
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
            .map_err(|_| EdgeError::SomethingWentWrong)
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
            meta: None,
        };

        let client_features_two = ClientFeatures {
            version: 2,
            features: vec![ClientFeature {
                name: "feature3".into(),
                ..ClientFeature::default()
            }],
            segments: None,
            query: None,
            meta: None,
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
            meta: None,
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
                variants: None,
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
                variants: None,
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

    #[test]
    pub fn can_parse_properties_map_from_get_query_string() {
        let config = Config::new(5, false);
        let query_string =
            "userId=123123&properties[email]=test@test.com&properties%5BcompanyId%5D=bricks&properties%5Bhello%5D=world";
        let context: Context = config
            .deserialize_str(query_string)
            .expect("Could not parse query string");
        assert_eq!(context.user_id, Some("123123".to_string()));
        let prop_map = context.properties.unwrap();
        assert_eq!(prop_map.len(), 3);
        assert!(prop_map.contains_key("companyId"));
        assert!(prop_map.contains_key("hello"));
        assert!(prop_map.contains_key("email"));
    }

    #[test_case("./examples/delta_base.json".into(), "./examples/delta_patch.json".into(); "Base and delta")]
    pub fn can_take_delta_updates(base: PathBuf, delta: PathBuf) {
        let base_delta: ClientFeaturesDelta = from_reader(read_file(base).unwrap()).unwrap();
        let mut features = ClientFeatures {
            version: 2,
            features: vec![],
            segments: None,
            query: None,
            meta: None,
        };
        features.apply_delta(&base_delta);
        assert_eq!(features.features.len(), 3);
        let delta: ClientFeaturesDelta = from_reader(read_file(delta).unwrap()).unwrap();
        features.apply_delta(&delta);
        assert_eq!(features.features.len(), 2);
    }

    #[test_case("./examples/delta_base.json".into(), "./examples/delta_patch.json".into(); "Base and delta")]
    pub fn validate_delta_updates(base_path: PathBuf, delta_path: PathBuf) {
        let base_delta: ClientFeaturesDelta = from_reader(read_file(base_path).unwrap()).unwrap();

        let mut updated_features = ClientFeatures::create_from_delta(&base_delta);
        let expected_feature_count = base_delta
            .events
            .iter()
            .filter(|event| matches!(event, DeltaEvent::FeatureUpdated { .. }))
            .count();
        assert_eq!(updated_features.features.len(), expected_feature_count);

        let delta_update: ClientFeaturesDelta =
            from_reader(read_file(delta_path).unwrap()).unwrap();
        updated_features.apply_delta(&delta_update);

        let mut sorted_delta_features: Vec<ClientFeature> = delta_update
            .events
            .iter()
            .filter_map(|event| {
                if let DeltaEvent::FeatureUpdated { feature, .. } = event {
                    Some(feature.clone())
                } else {
                    None
                }
            })
            .collect();
        sorted_delta_features.sort();

        let serialized_delta_updates = to_string(&sorted_delta_features).unwrap();
        let serialized_final_features = to_string(&updated_features.features).unwrap();

        assert_eq!(serialized_delta_updates, serialized_final_features);
    }

    #[test]
    pub fn when_strategy_variants_is_none_default_to_empty_vec() {
        let client_features = ClientFeatures {
            version: 2,
            features: vec![ClientFeature {
                name: "feature1".into(),
                strategies: Some(vec![Strategy {
                    name: "default".into(),
                    sort_order: Some(124),
                    segments: None,
                    constraints: None,
                    parameters: None,
                    variants: None,
                }]),
                ..ClientFeature::default()
            }],
            segments: None,
            query: None,
            meta: None,
        };
        let client_features_json = serde_json::to_string(&client_features).unwrap();
        let client_features_parsed: ClientFeatures =
            serde_json::from_str(&client_features_json).unwrap();
        let strategy = client_features_parsed
            .features
            .first()
            .unwrap()
            .strategies
            .as_ref()
            .unwrap()
            .first()
            .unwrap();
        assert_eq!(strategy.variants.as_ref().unwrap().len(), 0);
    }

    #[test]
    pub fn upserting_features_with_segments_overrides_constraints_on_segments_with_same_id_but_keeps_non_overlapping_segments(
    ) {
        let client_features_one = ClientFeatures {
            version: 2,
            features: vec![],
            segments: Some(vec![
                Segment {
                    constraints: vec![Constraint {
                        case_insensitive: false,
                        values: None,
                        context_name: "location".into(),
                        inverted: false,
                        operator: Operator::In,
                        value: Some("places".into()),
                    }],
                    id: 1,
                },
                Segment {
                    constraints: vec![Constraint {
                        case_insensitive: false,
                        values: None,
                        context_name: "hometown".into(),
                        inverted: false,
                        operator: Operator::In,
                        value: Some("somewhere_nice".into()),
                    }],
                    id: 2,
                },
            ]),
            query: None,
            meta: None,
        };
        let client_features_two = ClientFeatures {
            version: 2,
            features: vec![],
            segments: Some(vec![
                Segment {
                    constraints: vec![Constraint {
                        case_insensitive: false,
                        values: None,
                        context_name: "location".into(),
                        inverted: false,
                        operator: Operator::In,
                        value: Some("other-places".into()),
                    }],
                    id: 1,
                },
                Segment {
                    constraints: vec![Constraint {
                        case_insensitive: false,
                        values: None,
                        context_name: "origin".into(),
                        inverted: false,
                        operator: Operator::In,
                        value: Some("africa".into()),
                    }],
                    id: 3,
                },
            ]),
            query: None,
            meta: None,
        };

        let expected = vec![
            Constraint {
                case_insensitive: false,
                values: None,
                context_name: "hometown".into(),
                inverted: false,
                operator: Operator::In,
                value: Some("somewhere_nice".into()),
            },
            Constraint {
                case_insensitive: false,
                values: None,
                context_name: "location".into(),
                inverted: false,
                operator: Operator::In,
                value: Some("other-places".into()),
            },
            Constraint {
                case_insensitive: false,
                values: None,
                context_name: "origin".into(),
                inverted: false,
                operator: Operator::In,
                value: Some("africa".into()),
            },
        ];

        let upserted = client_features_one
            .clone()
            .upsert(client_features_two.clone());
        let mut new_constraints = upserted
            .segments
            .unwrap()
            .iter()
            .flat_map(|segment| segment.constraints.clone())
            .collect::<Vec<Constraint>>();
        new_constraints.sort_by(|a, b| a.context_name.cmp(&b.context_name));

        assert_eq!(new_constraints, expected);
    }

    #[test]
    pub fn when_meta_is_in_client_features_it_is_serialized() {
        let client_features = ClientFeatures {
            version: 2,
            features: vec![],
            segments: None,
            query: None,
            meta: Some(super::Meta {
                etag: Some("123:wqeqwe".into()),
                revision_id: Some(123),
                query_hash: Some("wqeqwe".into()),
            }),
        };
        let serialized = serde_json::to_string(&client_features).unwrap();
        assert!(serialized.contains("meta"));
    }

    #[test_case("./examples/nuno-response.json".into() ; "features with meta tag")]
    pub fn can_parse_meta_from_upstream(path: PathBuf) {
        let features: ClientFeatures = serde_json::from_reader(read_file(path).unwrap()).unwrap();
        assert!(features.meta.is_some());
        let meta = features.meta.unwrap();
        assert_eq!(meta.etag, Some("\"537b2ba0:3726\"".into()));
        assert_eq!(meta.revision_id, Some(3726));
        assert_eq!(meta.query_hash, Some("537b2ba0".into()));
    }
}
