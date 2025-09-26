use std::collections::{BTreeMap, HashMap, HashSet};

use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[cfg(feature = "openapi")]
use utoipa::ToSchema;

use crate::{Merge, MergeMut};

#[derive(Debug, Clone, Deserialize, Serialize, Default, Builder)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct ToggleStats {
    #[builder(default = "0")]
    pub no: u32,
    #[builder(default = "0")]
    pub yes: u32,
    #[builder(default = "HashMap::new()")]
    #[serde(default)]
    pub variants: HashMap<String, u32>,
}

impl ToggleStats {
    /// Increments yes count
    fn yes(&mut self) {
        self.yes += 1
    }
    /// Increments no count
    fn no(&mut self) {
        self.no += 1
    }

    /// Use after evaluating a toggle passing in whether or not the toggle was enabled
    pub fn count(&mut self, enabled: bool) {
        if enabled {
            self.yes()
        } else {
            self.no()
        }
    }

    /// Counts occurrence for variant with name.
    /// This method will also count yes for the toggle itself
    /// Use count_disabled()
    pub fn count_variant(&mut self, name: &str) {
        self.increment_variant_count(name);
        self.count(true);
    }

    pub fn variant_disabled(&mut self) {
        self.increment_variant_count("disabled");
        self.count(false);
    }

    /// Incrementing count for var
    fn increment_variant_count(&mut self, name: &str) {
        self.variants
            .entry(name.into())
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Builder)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct MetricBucket {
    pub start: DateTime<Utc>,
    pub stop: DateTime<Utc>,
    pub toggles: HashMap<String, ToggleStats>,
}

pub fn from_bucket_app_name_and_env(
    bucket: MetricBucket,
    app_name: String,
    environment: String,
    metadata: MetricsMetadata,
) -> Vec<ClientMetricsEnv> {
    let timestamp = bucket.start;
    bucket
        .toggles
        .into_iter()
        .map(|(name, stats)| ClientMetricsEnv {
            feature_name: name,
            app_name: app_name.clone(),
            environment: environment.clone(),
            timestamp,
            yes: stats.yes,
            no: stats.no,
            variants: stats.variants,
            metadata: metadata.clone(),
        })
        .collect()
}

#[derive(Debug, Clone, Deserialize, Serialize, Builder)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct ClientMetrics {
    pub app_name: String,
    pub bucket: MetricBucket,
    pub environment: Option<String>,
    pub instance_id: Option<String>,
    pub connection_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_metrics: Option<Vec<ImpactMetric>>,
    #[serde(flatten)]
    pub metadata: MetricsMetadata,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct ClientMetricsEnv {
    pub feature_name: String,
    pub app_name: String,
    pub environment: String,
    pub timestamp: DateTime<Utc>,
    pub yes: u32,
    pub no: u32,
    pub variants: HashMap<String, u32>,
    #[serde(flatten)]
    pub metadata: MetricsMetadata,
}

#[derive(Debug, Clone, Deserialize, Serialize, Builder, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct ConnectVia {
    pub app_name: String,
    pub instance_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Builder)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct ClientApplication {
    pub app_name: String,
    pub connect_via: Option<Vec<ConnectVia>>,
    pub environment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<String>>,
    pub instance_id: Option<String>,
    pub connection_id: Option<String>,
    pub interval: u32,
    pub started: DateTime<Utc>,
    pub strategies: Vec<String>,
    #[serde(flatten)]
    pub metadata: MetricsMetadata,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "lowercase")]
pub enum SdkType {
    Frontend,
    Backend,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Builder)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct MetricsMetadata {
    pub sdk_version: Option<String>,
    pub sdk_type: Option<SdkType>,
    pub yggdrasil_version: Option<String>,
    pub platform_name: Option<String>,
    pub platform_version: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct Bucket {
    #[serde(
        deserialize_with = "deserialize_bucket_le",
        serialize_with = "serialize_bucket_le"
    )]
    pub le: f64,
    pub count: u64,
}

impl PartialEq for Bucket {
    fn eq(&self, other: &Self) -> bool {
        let le_equal = if self.le.is_infinite() && other.le.is_infinite() {
            true
        } else {
            (self.le - other.le).abs() < f64::EPSILON
        };
        le_equal && self.count == other.count
    }
}

impl Eq for Bucket {}

impl PartialOrd for Bucket {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Bucket {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.le.partial_cmp(&other.le).unwrap_or(std::cmp::Ordering::Equal)
    }
}

fn deserialize_bucket_le<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum BucketLe {
        Number(f64),
        String(String),
    }
    
    match BucketLe::deserialize(deserializer)? {
        BucketLe::Number(n) => Ok(n),
        BucketLe::String(s) if s == "+Inf" => Ok(f64::INFINITY),
        BucketLe::String(s) => Err(D::Error::custom(format!("expected '+Inf', got '{}'", s))),
    }
}

fn serialize_bucket_le<S>(le: &f64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    if le.is_infinite() {
        serializer.serialize_str("+Inf")
    } else {
        serializer.serialize_f64(*le)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct BucketMetricSample {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    pub count: u64,
    pub sum: f64,
    pub buckets: Vec<Bucket>,
}

impl PartialEq for BucketMetricSample {
    fn eq(&self, other: &Self) -> bool {
        self.labels == other.labels
            && self.count == other.count
            && (self.sum - other.sum).abs() < f64::EPSILON
            && self.buckets == other.buckets
    }
}

impl Eq for BucketMetricSample {}

impl MergeMut for BucketMetricSample {
    fn merge(&mut self, other: BucketMetricSample) {
        self.count += other.count;
        self.sum += other.sum;
        
        // Merge buckets
        for bucket in other.buckets {
            if let Some(existing) = self.buckets.iter_mut().find(|b| {
                (b.le.is_infinite() && bucket.le.is_infinite())
                    || (b.le - bucket.le).abs() < f64::EPSILON
            }) {
                existing.count += bucket.count;
            } else {
                self.buckets.push(bucket);
            }
        }
        self.buckets.sort();
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Builder)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct NumericMetricSample {
    pub value: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
}

impl PartialEq for NumericMetricSample {
    fn eq(&self, other: &Self) -> bool {
        let values_equal = (self.value - other.value).abs() < f64::EPSILON;

        let labels_equal = self.labels == other.labels;

        values_equal && labels_equal
    }
}

impl Eq for NumericMetricSample {}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(untagged)]
pub enum MetricSample {
    Numeric(NumericMetricSample),
    Bucket(BucketMetricSample),
}

impl PartialEq for MetricSample {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (MetricSample::Numeric(a), MetricSample::Numeric(b)) => a == b,
            (MetricSample::Bucket(a), MetricSample::Bucket(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for MetricSample {}

impl MetricSample {
    pub fn labels(&self) -> &Option<BTreeMap<String, String>> {
        match self {
            MetricSample::Numeric(sample) => &sample.labels,
            MetricSample::Bucket(sample) => &sample.labels,
        }
    }

    pub fn labels_to_key(&self) -> String {
        match self.labels() {
            Some(labels_map) => {
                let mut sorted_entries: Vec<(&String, &String)> = labels_map.iter().collect();
                sorted_entries.sort_unstable_by(|a, b| a.0.cmp(b.0));

                let mut key = String::with_capacity(sorted_entries.len() * 16);

                for (i, (k, v)) in sorted_entries.iter().enumerate() {
                    if i > 0 {
                        key.push(',');
                    }
                    key.push_str(k);
                    key.push(':');
                    key.push_str(v);
                }
                key
            }
            None => String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "lowercase")]
pub enum MetricType {
    Counter,
    Gauge,
    Histogram,
    #[serde(other)]
    Unknown,
}

impl FromStr for MetricType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "counter" => Ok(MetricType::Counter),
            "gauge" => Ok(MetricType::Gauge),
            "histogram" => Ok(MetricType::Histogram),
            _ => Ok(MetricType::Unknown),
        }
    }
}

impl From<&str> for MetricType {
    fn from(s: &str) -> Self {
        s.parse()
            .expect("Failed to parse MetricType, this should never happen")
    }
}

impl std::fmt::Display for MetricType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MetricType::Counter => "counter",
                MetricType::Gauge => "gauge",
                MetricType::Histogram => "histogram",
                MetricType::Unknown => "unknown",
            }
        )
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Builder)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct ImpactMetric {
    pub name: String,
    pub help: String,
    pub r#type: MetricType,
    pub samples: Vec<MetricSample>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct ImpactMetricEnv {
    #[serde(flatten)]
    pub impact_metric: ImpactMetric,
    #[serde(skip)]
    pub app_name: String,
    #[serde(skip)]
    pub environment: String,
}

impl ImpactMetricEnv {
    pub fn new(impact_metric: ImpactMetric, app_name: String, environment: String) -> Self {
        Self {
            impact_metric,
            app_name,
            environment,
        }
    }
}

impl MergeMut for ImpactMetricEnv {
    fn merge(&mut self, other: ImpactMetricEnv) {
        let is_counter = self.impact_metric.r#type == MetricType::Counter;
        let is_histogram = self.impact_metric.r#type == MetricType::Histogram;

        self.impact_metric
            .samples
            .extend(other.impact_metric.samples);
        self.impact_metric
            .samples
            .sort_by(|a, b| a.labels_to_key().cmp(&b.labels_to_key()));

        let old_samples = std::mem::take(&mut self.impact_metric.samples);
        let mut deduped = Vec::with_capacity(old_samples.len());
        let mut iter = old_samples.into_iter();

        if let Some(mut prev) = iter.next() {
            for sample in iter {
                if prev.labels_to_key() == sample.labels_to_key() {
                    if is_counter {
                        if let (MetricSample::Numeric(ref mut p), MetricSample::Numeric(s)) = (&mut prev, sample) {
                            p.value += s.value;
                        }
                    } else if is_histogram {
                        if let (MetricSample::Bucket(ref mut p), MetricSample::Bucket(s)) = (&mut prev, sample) {
                            p.merge(s);
                        }
                    } else {
                        prev = sample;
                    }
                } else {
                    deduped.push(prev);
                    prev = sample;
                }
            }
            deduped.push(prev);
        }

        self.impact_metric.samples = deduped;
    }
}

impl ClientApplication {
    #[cfg(feature = "wall-clock")]
    pub fn new(app_name: &str, interval: u32) -> Self {
        Self {
            app_name: app_name.into(),
            connect_via: Some(vec![]),
            environment: None,
            projects: Some(vec![]),
            instance_id: None,
            connection_id: None,
            interval,
            started: Utc::now(),
            strategies: vec![],
            metadata: MetricsMetadata {
                sdk_version: None,
                sdk_type: None,
                yggdrasil_version: None,
                platform_name: None,
                platform_version: None,
            },
        }
    }

    pub fn add_strategies(&mut self, strategies: Vec<String>) {
        let unique_strats: Vec<String> = self
            .strategies
            .clone()
            .into_iter()
            .chain(strategies)
            .collect::<HashSet<String>>()
            .into_iter()
            .collect();
        self.strategies = unique_strats;
    }

    pub fn connect_via(&self, app_name: &str, instance_id: &str) -> ClientApplication {
        let mut connect_via = self.connect_via.clone().unwrap_or_default();
        connect_via.push(ConnectVia {
            app_name: app_name.into(),
            instance_id: instance_id.into(),
        });
        Self {
            connect_via: Some(connect_via),
            ..self.clone()
        }
    }
}

impl Merge for ClientApplication {
    /// Will keep all set fields from self, overwriting None with Somes from other
    /// Will merge strategies from self and other, deduplicating
    fn merge(self, other: ClientApplication) -> ClientApplication {
        let mut merged_strategies: Vec<String> = self
            .strategies
            .into_iter()
            .chain(other.strategies)
            .collect::<HashSet<String>>()
            .into_iter()
            .collect();
        merged_strategies.sort();
        let merged_connected_via: Option<Vec<ConnectVia>> = self
            .connect_via
            .map(|c| {
                let initial = c.into_iter();
                let other_iter = other.connect_via.clone().unwrap_or_default().into_iter();
                let connect_via: Vec<ConnectVia> = initial.chain(other_iter).collect();
                connect_via
            })
            .or(other.connect_via.clone());

        let merged_projects: Option<Vec<String>> = match (self.projects, other.projects) {
            (Some(self_projects), Some(other_projects)) => {
                let mut projects: Vec<String> = self_projects
                    .into_iter()
                    .chain(other_projects)
                    .collect::<HashSet<String>>()
                    .into_iter()
                    .collect();
                projects.sort();
                Some(projects)
            }
            (Some(projects), None) => Some(projects),
            (None, Some(projects)) => Some(projects),
            (None, None) => None,
        };

        ClientApplication {
            app_name: self.app_name,
            environment: self.environment.or(other.environment),
            projects: merged_projects,
            instance_id: self.instance_id.or(other.instance_id),
            connection_id: self.connection_id.or(other.connection_id),
            interval: self.interval,
            started: self.started,
            strategies: merged_strategies,
            connect_via: merged_connected_via,
            metadata: MetricsMetadata {
                sdk_version: self.metadata.sdk_version.or(other.metadata.sdk_version),
                sdk_type: self.metadata.sdk_type.or(other.metadata.sdk_type),
                yggdrasil_version: self
                    .metadata
                    .yggdrasil_version
                    .or(other.metadata.yggdrasil_version),
                platform_name: self.metadata.platform_name.or(other.metadata.platform_name),
                platform_version: self
                    .metadata
                    .platform_version
                    .or(other.metadata.platform_version),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::*;

    #[test]
    fn client_metrics_with_impact_metrics_serialization() {
        let impact_metrics = vec![ImpactMetric {
            name: "labeled_counter".into(),
            help: "with labels".into(),
            r#type: MetricType::Counter,
            samples: vec![MetricSample::Numeric(NumericMetricSample {
                value: 10.0,
                labels: Some(BTreeMap::from([("foo".into(), "bar".into())])),
            })],
        }];

        let metrics = ClientMetrics {
            app_name: "test-name".into(),
            environment: Some("test-env".into()),
            instance_id: Some("test-instance-id".into()),
            connection_id: Some("test-connection-id".into()),
            impact_metrics: Some(impact_metrics.clone()),
            bucket: MetricBucket {
                start: DateTime::<Utc>::from_timestamp(1000, 0).unwrap(),
                stop: DateTime::<Utc>::from_timestamp(1000, 0).unwrap(),
                toggles: HashMap::new(),
            },
            metadata: MetricsMetadata {
                sdk_version: Some("rust-1.3.0".into()),
                sdk_type: Some(SdkType::Backend),
                yggdrasil_version: None,
                platform_name: Some("rustc".into()),
                platform_version: Some("1.7.9".into()),
            },
        };

        let json_string = serde_json::to_string(&metrics).unwrap();
        let deserialized: ClientMetrics = serde_json::from_str(&json_string).unwrap();

        assert_eq!(deserialized.impact_metrics, Some(impact_metrics));
    }

    #[test]
    pub fn can_increment_counts() {
        let mut stats = ToggleStats::default();
        assert_eq!(stats.yes, 0);
        assert_eq!(stats.no, 0);
        stats.yes();
        stats.no();
        assert_eq!(stats.yes, 1);
        assert_eq!(stats.no, 1);
    }

    #[test]
    pub fn can_increment_variant_count() {
        let mut stats = ToggleStats::default();
        assert!(stats.variants.is_empty());
        stats.increment_variant_count("red");
        stats.increment_variant_count("red");
        let count = stats.variants.get("red").expect("No red key in map");
        assert_eq!(count, &2);
    }

    #[test]
    pub fn counts_correctly_based_on_enabled() {
        let mut stats = ToggleStats::default();
        stats.count(true);
        stats.count(true);
        stats.count(true);
        stats.count(false);
        stats.count(false);
        assert_eq!(stats.yes, 3);
        assert_eq!(stats.no, 2);
    }
    #[test]
    pub fn counting_variant_should_also_increment_yes_no_counters() {
        let mut stats = ToggleStats::default();
        stats.count_variant("red");
        stats.count_variant("green");
        stats.count_variant("green");
        stats.count_variant("green");
        stats.variant_disabled();
        assert_eq!(stats.yes, 4);
        assert_eq!(stats.no, 1);
        let red_count = stats.variants.get("red").unwrap();
        let green_count = stats.variants.get("green").unwrap();
        let disabled_count = stats.variants.get("disabled").unwrap();
        assert_eq!(red_count, &1);
        assert_eq!(green_count, &3);
        assert_eq!(disabled_count, &1);
    }

    #[test]
    fn toggle_states_can_be_deserialized_without_variants() {
        let serialized_metrics = r#"
        {
            "appName": "some-app",
            "instanceId": "some-instance",
            "bucket": {
              "start": "1867-11-07T12:00:00Z",
              "stop": "1934-11-07T12:00:00Z",
              "toggles": {
                "some-feature": {
                  "yes": 1,
                  "no": 0
                }
              }
            }
          }
        "#;
        let metrics: ClientMetrics = serde_json::from_str(serialized_metrics).unwrap();
        assert_eq!(metrics.bucket.toggles.get("some-feature").unwrap().yes, 1);
        assert_eq!(metrics.bucket.toggles.get("some-feature").unwrap().no, 0);
    }

    #[test]
    fn metrics_can_be_deserialized_from_legacy_data_structure() {
        let serialized_metrics = r#"
        {
            "appName": "some-app",
            "instanceId": "some-instance",
            "bucket": {
              "start": "1867-11-07T12:00:00Z",
              "stop": "1934-11-07T12:00:00Z",
              "toggles": {}
            }
          }
        "#;
        let metrics: ClientMetrics =
            serde_json::from_str(serialized_metrics).expect("Should have deserialized correctly");
        assert_eq!(metrics.metadata.yggdrasil_version, None);
    }

    #[test]
    fn metrics_can_be_deserialized_when_containing_metadata_fields() {
        let serialized_metrics = r#"
        {
            "appName": "some-app",
            "instanceId": "some-instance",
            "bucket": {
              "start": "1867-11-07T12:00:00Z",
              "stop": "1934-11-07T12:00:00Z",
              "toggles": {}
            },
            "sdkVersion": "malbolge-1.0.0"
          }
        "#;
        let metrics: ClientMetrics =
            serde_json::from_str(serialized_metrics).expect("Should have deserialized correctly");
        assert_eq!(metrics.metadata.sdk_version, Some("malbolge-1.0.0".into()));
    }

    #[test]
    fn registration_can_be_deserialized_from_legacy_data_structure() {
        let serialized_registration = r#"
        {
            "appName": "some-app",
            "environment": "some-instance",
            "projects": ["default"],
            "instanceId": "something",
            "interval": 15000,
            "started": "1867-11-07T12:00:00Z",
            "strategies": ["I-made-this-up"]
          }
        "#;
        let registration: ClientApplication = serde_json::from_str(serialized_registration)
            .expect("Should have deserialized correctly");
        assert_eq!(registration.metadata.yggdrasil_version, None);
    }

    #[test]
    fn registration_can_be_deserialized_when_containing_metadata_fields() {
        let serialized_metrics = r#"
        {
            "appName": "some-app",
            "instanceId": "some-instance",
            "bucket": {
              "start": "1867-11-07T12:00:00Z",
              "stop": "1934-11-07T12:00:00Z",
              "toggles": {}
            },
            "sdkVersion": "malbolge-1.0.0"
          }
        "#;
        let metrics: ClientMetrics =
            serde_json::from_str(serialized_metrics).expect("Should have deserialized correctly");

        assert_eq!(metrics.metadata.sdk_version, Some("malbolge-1.0.0".into()));
    }

    #[test]
    fn metrics_metadata_is_flattened_during_serialization() {
        let expected_metrics = r#"
        {
            "appName": "test-name",
            "bucket": {
              "start": "1970-01-01T00:16:40Z",
              "stop": "1970-01-01T00:16:40Z",
              "toggles": {}
            },
            "environment": "test-env",
            "instanceId": "test-instance-id",
            "connectionId": "test-connection-id",
            "sdkVersion": "rust-1.3.0",
            "sdkType": "backend",
            "yggdrasilVersion": null,
            "platformName": "rustc",
            "platformVersion": "1.7.9"
          }
        "#
        .replace(" ", "")
        .replace("\n", "");

        let metrics = ClientMetrics {
            app_name: "test-name".into(),
            environment: Some("test-env".into()),
            instance_id: Some("test-instance-id".into()),
            connection_id: Some("test-connection-id".into()),
            impact_metrics: None,
            bucket: MetricBucket {
                start: DateTime::<Utc>::from_timestamp(1000, 0).unwrap(),
                stop: DateTime::<Utc>::from_timestamp(1000, 0).unwrap(),
                toggles: HashMap::new(),
            },
            metadata: MetricsMetadata {
                sdk_version: Some("rust-1.3.0".into()),
                sdk_type: Some(SdkType::Backend),
                yggdrasil_version: None,
                platform_name: Some("rustc".into()),
                platform_version: Some("1.7.9".into()),
            },
        };

        let json_string = serde_json::to_string(&metrics).unwrap();
        assert_eq!(json_string, expected_metrics);
    }

    #[test]
    fn registration_metadata_is_flattened_during_serialization() {
        let expected_registration = r#"
        {
            "appName": "test-name",
            "connectVia": null,
            "environment": "test-env",
            "projects": ["default"],
            "instanceId": "test-instance-id",
            "connectionId": "test-connection-id",
            "interval": 15000,
            "started": "1970-01-01T00:16:40Z",
            "strategies": [],
            "sdkVersion": "rust-1.3.0",
            "sdkType": "backend",
            "yggdrasilVersion": null,
            "platformName": "rustc",
            "platformVersion": "1.7.9"
          }
        "#
        .replace(" ", "")
        .replace("\n", "");

        let metrics = ClientApplication {
            app_name: "test-name".into(),
            environment: Some("test-env".into()),
            projects: Some(vec!["default".into()]),
            instance_id: Some("test-instance-id".into()),
            connection_id: Some("test-connection-id".into()),
            metadata: MetricsMetadata {
                sdk_version: Some("rust-1.3.0".into()),
                sdk_type: Some(SdkType::Backend),
                yggdrasil_version: None,
                platform_name: Some("rustc".into()),
                platform_version: Some("1.7.9".into()),
            },
            connect_via: None,
            interval: 15000,
            started: DateTime::<Utc>::from_timestamp(1000, 0).unwrap(),
            strategies: vec![],
        };

        let json_string = serde_json::to_string(&metrics).unwrap();
        assert_eq!(json_string, expected_registration);
    }
}

#[cfg(test)]
#[cfg(feature = "wall-clock")]
mod clock_tests {
    use chrono::{Duration, Utc};

    use super::*;

    #[test]
    pub fn can_have_client_metrics_env_from_metrics_bucket() {
        let start = Utc::now();
        let mut stats_feature_one = ToggleStats::default();
        stats_feature_one.count_variant("red");
        stats_feature_one.count_variant("green");
        stats_feature_one.count_variant("green");
        stats_feature_one.count_variant("green");
        stats_feature_one.variant_disabled();
        let mut stats_feature_two = ToggleStats::default();
        stats_feature_two.count_variant("red");
        stats_feature_two.count_variant("red");
        stats_feature_two.count_variant("red");
        stats_feature_two.count_variant("green");
        stats_feature_two.yes();
        stats_feature_two.yes();
        stats_feature_two.yes();
        stats_feature_two.variant_disabled();
        let mut map = HashMap::new();
        map.insert("feature_one".to_string(), stats_feature_one);
        map.insert("feature_two".to_string(), stats_feature_two);
        let bucket = MetricBucket {
            start,
            stop: start + Duration::minutes(50),
            toggles: map,
        };
        let client_metrics_env = from_bucket_app_name_and_env(
            bucket,
            "unleash_edge_metrics".into(),
            "development".into(),
            MetricsMetadata {
                ..Default::default()
            },
        );
        assert_eq!(client_metrics_env.len(), 2);
        let feature_one_metrics = client_metrics_env
            .clone()
            .into_iter()
            .find(|e| e.feature_name == "feature_one")
            .unwrap();

        assert_eq!(feature_one_metrics.yes, 4);
        assert_eq!(feature_one_metrics.no, 1);

        let feature_two_metrics = client_metrics_env
            .into_iter()
            .find(|e| e.feature_name == "feature_two")
            .unwrap();

        assert_eq!(feature_two_metrics.yes, 7);
        assert_eq!(feature_two_metrics.no, 1);
    }

    #[test]
    pub fn can_connect_via_new_application() {
        let demo_data = ClientApplication {
            app_name: "demo".into(),
            interval: 15500,
            environment: Some("production".into()),
            started: Utc::now(),
            strategies: vec!["default".into(), "CustomStrategy".into()],
            ..Default::default()
        };
        let connected_via = demo_data.connect_via("unleash-edge", "edge-id-1");
        assert_eq!(
            connected_via.connect_via,
            Some(vec![ConnectVia {
                app_name: "unleash-edge".into(),
                instance_id: "edge-id-1".into(),
            }]),
        )
    }

    #[test]
    pub fn can_merge_connected_via_where_one_side_is_none() {
        let started = Utc::now();
        let demo_data_1 = ClientApplication {
            app_name: "demo".into(),
            interval: 15500,
            started,
            strategies: vec!["default".into(), "gradualRollout".into()],
            metadata: MetricsMetadata {
                sdk_version: Some("unleash-client-java:7.1.0".into()),
                ..Default::default()
            },
            ..Default::default()
        };

        let demo_data_2 = ClientApplication {
            connect_via: Some(vec![ConnectVia {
                app_name: "unleash-edge".into(),
                instance_id: "2".into(),
            }]),
            app_name: "demo".into(),
            interval: 15500,
            environment: Some("production".into()),
            started,
            strategies: vec!["default".into(), "CustomStrategy".into()],
            ..Default::default()
        };
        let merged = demo_data_1.clone().merge(demo_data_2.clone());
        assert_eq!(demo_data_2.connect_via, merged.connect_via);
        let reverse_merge = demo_data_2.clone().merge(demo_data_1);
        assert_eq!(demo_data_2.connect_via, reverse_merge.connect_via);
    }

    #[test]
    pub fn can_merge_connected_via() {
        let started = Utc::now();
        let demo_data_1 = ClientApplication {
            connect_via: Some(vec![ConnectVia {
                app_name: "unleash-edge".into(),
                instance_id: "1".into(),
            }]),
            app_name: "demo".into(),
            interval: 15500,
            started,
            strategies: vec!["default".into(), "gradualRollout".into()],
            metadata: MetricsMetadata {
                sdk_version: Some("unleash-client-java:7.1.0".into()),
                ..Default::default()
            },
            ..Default::default()
        };

        let demo_data_2 = ClientApplication {
            connect_via: Some(vec![ConnectVia {
                app_name: "unleash-edge".into(),
                instance_id: "2".into(),
            }]),
            app_name: "demo".into(),
            interval: 15500,
            environment: Some("production".into()),
            started,
            strategies: vec!["default".into(), "CustomStrategy".into()],
            ..Default::default()
        };

        let merged = demo_data_1.merge(demo_data_2);
        let connections = merged.connect_via.unwrap();
        assert_eq!(connections.len(), 2);
        assert_eq!(
            connections,
            vec![
                ConnectVia {
                    app_name: "unleash-edge".into(),
                    instance_id: "1".into(),
                },
                ConnectVia {
                    app_name: "unleash-edge".into(),
                    instance_id: "2".into(),
                }
            ]
        )
    }

    #[test]
    pub fn merging_two_client_applications_prioritizes_left_hand_side() {
        let started = Utc::now();
        let demo_data_1 = ClientApplication {
            app_name: "demo".into(),
            interval: 15500,
            started,
            strategies: vec!["default".into(), "gradualRollout".into()],
            metadata: MetricsMetadata {
                sdk_version: Some("unleash-client-java:7.1.0".into()),
                ..Default::default()
            },
            ..Default::default()
        };

        let demo_data_2 = ClientApplication {
            app_name: "demo".into(),
            interval: 15500,
            environment: Some("production".into()),
            started,
            strategies: vec!["default".into(), "CustomStrategy".into()],
            ..Default::default()
        };

        let left = demo_data_2.clone().merge(demo_data_1.clone());
        let right = demo_data_1.merge(demo_data_2);

        assert_eq!(left, right);
    }

    #[test]
    pub fn merging_two_client_applications_should_use_set_values() {
        let demo_data_orig = ClientApplication::new("demo", 15000);
        let demo_data_with_more_data = ClientApplication {
            app_name: "demo".into(),
            interval: 15500,
            environment: Some("development".into()),
            instance_id: Some("instance_id".into()),
            connection_id: Some("connection_id".into()),
            started: Utc::now(),
            strategies: vec!["default".into(), "gradualRollout".into()],
            metadata: MetricsMetadata {
                sdk_version: Some("unleash-client-java:7.1.0".into()),
                ..Default::default()
            },
            ..Default::default()
        };
        // Cloning orig here, to avoid the destructive merge preventing us from testing
        let merged = demo_data_orig.clone().merge(demo_data_with_more_data);
        assert_eq!(merged.interval, demo_data_orig.interval);
        assert_eq!(merged.environment, Some("development".into()));
        assert_eq!(
            merged.metadata.sdk_version,
            Some("unleash-client-java:7.1.0".into())
        );
        assert_eq!(merged.instance_id, Some("instance_id".into()));
        assert_eq!(merged.connection_id, Some("connection_id".into()));
        assert_eq!(merged.started, demo_data_orig.started);
        assert_eq!(merged.strategies.len(), 2);
    }

    #[test]
    pub fn merging_two_client_applications_should_eliminate_duplicate_strategies() {
        let mut demo_data_1 = ClientApplication::new("demo", 15000);
        let mut demo_data_2 = ClientApplication::new("demo", 15000);
        demo_data_1.add_strategies(vec!["default".into(), "gradualRollout".into()]);
        demo_data_2.add_strategies(vec!["default".into(), "randomRollout".into()]);
        let demo_data_3 = demo_data_1.merge(demo_data_2);
        assert_eq!(demo_data_3.strategies.len(), 3);
    }

    fn sort_samples_by_labels(mut impact_metric: ImpactMetric) -> ImpactMetric {
        impact_metric.samples.sort_by(|a, b| {
            let a_key = a.labels_to_key();
            let b_key = b.labels_to_key();
            a_key.cmp(&b_key)
        });
        impact_metric
    }

    #[test]
    pub fn merging_impact_metric_env_counter_type_adds_values() {
        let mut metric1 = ImpactMetricEnv {
            impact_metric: ImpactMetric {
                name: "test_counter".into(),
                help: "Test counter metric".into(),
                r#type: MetricType::Counter,
                samples: vec![
                    MetricSample::Numeric(NumericMetricSample {
                        value: 10.0,
                        labels: Some(BTreeMap::from([("label1".into(), "value1".into())])),
                    }),
                    MetricSample::Numeric(NumericMetricSample {
                        value: 20.0,
                        labels: Some(BTreeMap::from([("label2".into(), "value2".into())])),
                    }),
                ],
            },
            app_name: "test_app".into(),
            environment: "test_env".into(),
        };

        let metric2 = ImpactMetricEnv {
            impact_metric: ImpactMetric {
                name: "test_counter".into(),
                help: "Test counter metric".into(),
                r#type: MetricType::Counter,
                samples: vec![
                    MetricSample::Numeric(NumericMetricSample {
                        value: 15.0,
                        labels: Some(BTreeMap::from([("label1".into(), "value1".into())])),
                    }),
                    MetricSample::Numeric(NumericMetricSample {
                        value: 25.0,
                        labels: Some(BTreeMap::from([("label3".into(), "value3".into())])),
                    }),
                ],
            },
            app_name: "test_app".into(),
            environment: "test_env".into(),
        };

        metric1.merge(metric2);

        let expected_impact_metric = ImpactMetric {
            name: "test_counter".into(),
            help: "Test counter metric".into(),
            r#type: MetricType::Counter,
            samples: vec![
                MetricSample::Numeric(NumericMetricSample {
                    value: 25.0, // 10.0 + 15.0
                    labels: Some(BTreeMap::from([("label1".into(), "value1".into())])),
                }),
                MetricSample::Numeric(NumericMetricSample {
                    value: 20.0, // Only in metric1
                    labels: Some(BTreeMap::from([("label2".into(), "value2".into())])),
                }),
                MetricSample::Numeric(NumericMetricSample {
                    value: 25.0, // Only in metric2
                    labels: Some(BTreeMap::from([("label3".into(), "value3".into())])),
                }),
            ],
        };

        let sorted_merged = sort_samples_by_labels(metric1.impact_metric);
        let sorted_expected = sort_samples_by_labels(expected_impact_metric);

        assert_eq!(sorted_merged, sorted_expected);
    }

    #[test]
    pub fn merging_impact_metric_env_gauge_type_uses_last_value() {
        let mut metric1 = ImpactMetricEnv {
            impact_metric: ImpactMetric {
                name: "test_gauge".into(),
                help: "Test gauge metric".into(),
                r#type: MetricType::Gauge,
                samples: vec![
                    MetricSample::Numeric(NumericMetricSample {
                        value: 10.0,
                        labels: Some(BTreeMap::from([("label1".into(), "value1".into())])),
                    }),
                    MetricSample::Numeric(NumericMetricSample {
                        value: 20.0,
                        labels: Some(BTreeMap::from([("label2".into(), "value2".into())])),
                    }),
                ],
            },
            app_name: "test_app".into(),
            environment: "test_env".into(),
        };

        let metric2 = ImpactMetricEnv {
            impact_metric: ImpactMetric {
                name: "test_gauge".into(),
                help: "Test gauge metric".into(),
                r#type: MetricType::Gauge,
                samples: vec![
                    MetricSample::Numeric(NumericMetricSample {
                        value: 15.0,
                        labels: Some(BTreeMap::from([("label1".into(), "value1".into())])),
                    }),
                    MetricSample::Numeric(NumericMetricSample {
                        value: 25.0,
                        labels: Some(BTreeMap::from([("label3".into(), "value3".into())])),
                    }),
                ],
            },
            app_name: "test_app".into(),
            environment: "test_env".into(),
        };

        metric1.merge(metric2);

        let expected_impact_metric = ImpactMetric {
            name: "test_gauge".into(),
            help: "Test gauge metric".into(),
            r#type: MetricType::Gauge,
            samples: vec![
                MetricSample::Numeric(NumericMetricSample {
                    value: 15.0, // Last value from metric2
                    labels: Some(BTreeMap::from([("label1".into(), "value1".into())])),
                }),
                MetricSample::Numeric(NumericMetricSample {
                    value: 20.0, // Only in metric1
                    labels: Some(BTreeMap::from([("label2".into(), "value2".into())])),
                }),
                MetricSample::Numeric(NumericMetricSample {
                    value: 25.0, // Only in metric2
                    labels: Some(BTreeMap::from([("label3".into(), "value3".into())])),
                }),
            ],
        };

        let sorted_merged = sort_samples_by_labels(metric1.impact_metric);
        let sorted_expected = sort_samples_by_labels(expected_impact_metric);

        assert_eq!(sorted_merged, sorted_expected);
    }

    #[test]
    pub fn histogram_metric_serialization() {
        let histogram_metric = ImpactMetric {
            name: "test_histogram".into(),
            help: "Test histogram metric".into(),
            r#type: MetricType::Histogram,
            samples: vec![MetricSample::Bucket(BucketMetricSample {
                labels: Some(BTreeMap::from([("endpoint".into(), "/api/test".into())])),
                count: 50,
                sum: 125.5,
                buckets: vec![
                    Bucket { le: 0.1, count: 10 },
                    Bucket { le: 1.0, count: 30 },
                    Bucket { le: f64::INFINITY, count: 50 },
                ],
            })],
        };

        let json_string = serde_json::to_string(&histogram_metric).unwrap();
        let deserialized: ImpactMetric = serde_json::from_str(&json_string).unwrap();

        assert_eq!(deserialized, histogram_metric);
        // Check that the infinity bucket is serialized as "+Inf"
        assert!(
            json_string.contains("\"+Inf\""),
            "JSON should contain +Inf for infinity bucket. Got: {}",
            json_string
        );
    }

    #[test]
    pub fn merging_histogram_metrics() {
        let mut metric1 = ImpactMetricEnv {
            impact_metric: ImpactMetric {
                name: "test_histogram".into(),
                help: "Test histogram metric".into(),
                r#type: MetricType::Histogram,
                samples: vec![MetricSample::Bucket(BucketMetricSample {
                    labels: Some(BTreeMap::from([("service".into(), "api".into())])),
                    count: 10,
                    sum: 25.0,
                    buckets: vec![
                        Bucket { le: 0.1, count: 5 },
                        Bucket { le: 1.0, count: 8 },
                        Bucket { le: f64::INFINITY, count: 10 },
                    ],
                })],
            },
            app_name: "test_app".into(),
            environment: "test_env".into(),
        };

        let metric2 = ImpactMetricEnv {
            impact_metric: ImpactMetric {
                name: "test_histogram".into(),
                help: "Test histogram metric".into(),
                r#type: MetricType::Histogram,
                samples: vec![MetricSample::Bucket(BucketMetricSample {
                    labels: Some(BTreeMap::from([("service".into(), "api".into())])),
                    count: 5,
                    sum: 15.0,
                    buckets: vec![
                        Bucket { le: 0.1, count: 2 },
                        Bucket { le: 0.5, count: 4 }, // New bucket
                        Bucket { le: f64::INFINITY, count: 5 },
                    ],
                })],
            },
            app_name: "test_app".into(),
            environment: "test_env".into(),
        };

        metric1.merge(metric2);

        let expected = ImpactMetricEnv {
            impact_metric: ImpactMetric {
                name: "test_histogram".into(),
                help: "Test histogram metric".into(),
                r#type: MetricType::Histogram,
                samples: vec![MetricSample::Bucket(BucketMetricSample {
                    labels: Some(BTreeMap::from([("service".into(), "api".into())])),
                    count: 15, // 10 + 5
                    sum: 40.0, // 25.0 + 15.0
                    buckets: vec![
                        Bucket { le: 0.1, count: 7 }, // 5 + 2
                        Bucket { le: 0.5, count: 4 }, // New from metric2
                        Bucket { le: 1.0, count: 8 }, // Only from metric1
                        Bucket { le: f64::INFINITY, count: 15 }, // 10 + 5
                    ],
                })],
            },
            app_name: "test_app".into(),
            environment: "test_env".into(),
        };

        assert_eq!(metric1, expected);
    }

    #[test]
    fn bucket_ordering() {
        let bucket_1 = Bucket { le: 0.1, count: 10 };
        let bucket_2 = Bucket { le: 1.0, count: 20 };
        let bucket_3 = Bucket { le: 10.0, count: 30 };
        let bucket_inf = Bucket { le: f64::INFINITY, count: 40 };

        assert!(bucket_1 < bucket_2);
        assert!(bucket_2 < bucket_3);
        assert!(bucket_3 < bucket_inf);
        assert!(bucket_1 < bucket_inf);
    }

}
