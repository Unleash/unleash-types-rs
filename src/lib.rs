use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Query {
    pub tags: Option<Vec<Vec<String>>>,
    pub projects: Option<Vec<String>>,
    pub name_prefix: Option<String>,
    pub environment: Option<String>,
    pub inline_segment_constraints: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Operator {
    NotIn,
    In,
    StrEndWith,
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
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Constraint {
    pub context_name: String,
    pub operator: Operator,
    pub case_insensitive: bool,
    pub inverted: bool,
    pub values: Option<Vec<String>>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WeightType {
    Fix,
    Variable,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct Strategy {
    pub name: String,
    pub sort_order: Option<i32>,
    pub segments: Option<Vec<i32>>,
    pub constraints: Option<Vec<Constraint>>,
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
        self.sort_order.cmp(&other.sort_order)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Override {
    pub context_name: String,
    pub values: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Payload {
    #[serde(rename = "type")]
    pub payload_type: String,
    pub value: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Variant {
    pub name: String,
    pub weight: i32,
    pub weight_type: Option<WeightType>,
    pub stickiness: String,
    pub payload: Option<Payload>,
    pub overrides: Option<Vec<Override>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Segment {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub constraints: Vec<Constraint>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    pub project: String,
    pub strategies: Option<Vec<Strategy>>,
    pub variants: Option<Vec<Variant>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientFeatures {
    pub version: u32,
    pub features: Vec<ClientFeature>,
    pub segments: Option<Vec<Segment>>,
    pub query: Option<Query>,
}
