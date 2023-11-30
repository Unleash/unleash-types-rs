use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[cfg(feature = "openapi")]
use utoipa::ToSchema;

use crate::Merge;

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
pub struct ClientApplication {
    pub app_name: String,
    pub connect_via: Option<Vec<ConnectVia>>,
    pub environment: Option<String>,
    pub instance_id: Option<String>,
    pub interval: u32,
    pub sdk_version: Option<String>,
    pub started: DateTime<Utc>,
    pub strategies: Vec<String>,
}

impl ClientApplication {
    pub fn new(app_name: &str, interval: u32) -> Self {
        Self {
            app_name: app_name.into(),
            connect_via: Some(vec![]),
            environment: None,
            instance_id: None,
            interval,
            sdk_version: None,
            started: Utc::now(),
            strategies: vec![],
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
        let mut connect_via = self.connect_via.clone().unwrap_or(vec![]);
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
                let other_iter = other.connect_via.clone().unwrap_or(vec![]).into_iter();
                let connect_via: Vec<ConnectVia> = initial.chain(other_iter).collect();
                connect_via
            })
            .or(other.connect_via.clone());

        ClientApplication {
            app_name: self.app_name,
            environment: self.environment.or(other.environment),
            instance_id: self.instance_id.or(other.instance_id),
            interval: self.interval,
            sdk_version: self.sdk_version.or(other.sdk_version),
            started: self.started,
            strategies: merged_strategies,
            connect_via: merged_connected_via,
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};

    use super::*;

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
    pub fn merging_two_client_applications_should_eliminate_duplicate_strategies() {
        let mut demo_data_1 = ClientApplication::new("demo", 15000);
        let mut demo_data_2 = ClientApplication::new("demo", 15000);
        demo_data_1.add_strategies(vec!["default".into(), "gradualRollout".into()]);
        demo_data_2.add_strategies(vec!["default".into(), "randomRollout".into()]);
        let demo_data_3 = demo_data_1.merge(demo_data_2);
        assert_eq!(demo_data_3.strategies.len(), 3);
    }

    #[test]
    pub fn merging_two_client_applications_should_use_set_values() {
        let demo_data_orig = ClientApplication::new("demo", 15000);
        let demo_data_with_more_data = ClientApplication {
            connect_via: None,
            app_name: "demo".into(),
            interval: 15500,
            environment: Some("development".into()),
            instance_id: Some("instance_id".into()),
            sdk_version: Some("unleash-client-java:7.1.0".into()),
            started: Utc::now(),
            strategies: vec!["default".into(), "gradualRollout".into()],
        };
        // Cloning orig here, to avoid the destructive merge preventing us from testing
        let merged = demo_data_orig.clone().merge(demo_data_with_more_data);
        assert_eq!(merged.interval, demo_data_orig.interval);
        assert_eq!(merged.environment, Some("development".into()));
        assert_eq!(merged.sdk_version, Some("unleash-client-java:7.1.0".into()));
        assert_eq!(merged.instance_id, Some("instance_id".into()));
        assert_eq!(merged.started, demo_data_orig.started);
        assert_eq!(merged.strategies.len(), 2);
    }

    #[test]
    pub fn merging_two_client_applications_prioritizes_left_hand_side() {
        let started = Utc::now();
        let demo_data_1 = ClientApplication {
            connect_via: None,
            app_name: "demo".into(),
            interval: 15500,
            environment: None,
            instance_id: None,
            sdk_version: Some("unleash-client-java:7.1.0".into()),
            started,
            strategies: vec!["default".into(), "gradualRollout".into()],
        };

        let demo_data_2 = ClientApplication {
            connect_via: None,
            app_name: "demo".into(),
            interval: 15500,
            environment: Some("production".into()),
            instance_id: None,
            sdk_version: None,
            started,
            strategies: vec!["default".into(), "CustomStrategy".into()],
        };

        let left = demo_data_2.clone().merge(demo_data_1.clone());
        let right = demo_data_1.merge(demo_data_2);

        assert_eq!(left, right);
    }

    #[test]
    pub fn can_connect_via_new_application() {
        let demo_data = ClientApplication {
            connect_via: None,
            app_name: "demo".into(),
            interval: 15500,
            environment: Some("production".into()),
            instance_id: None,
            sdk_version: None,
            started: Utc::now(),
            strategies: vec!["default".into(), "CustomStrategy".into()],
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
    pub fn can_merge_connected_via() {
        let started = Utc::now();
        let demo_data_1 = ClientApplication {
            connect_via: Some(vec![ConnectVia {
                app_name: "unleash-edge".into(),
                instance_id: "1".into(),
            }]),
            app_name: "demo".into(),
            interval: 15500,
            environment: None,
            instance_id: None,
            sdk_version: Some("unleash-client-java:7.1.0".into()),
            started,
            strategies: vec!["default".into(), "gradualRollout".into()],
        };

        let demo_data_2 = ClientApplication {
            connect_via: Some(vec![ConnectVia {
                app_name: "unleash-edge".into(),
                instance_id: "2".into(),
            }]),
            app_name: "demo".into(),
            interval: 15500,
            environment: Some("production".into()),
            instance_id: None,
            sdk_version: None,
            started,
            strategies: vec!["default".into(), "CustomStrategy".into()],
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
    pub fn can_merge_connected_via_where_one_side_is_none() {
        let started = Utc::now();
        let demo_data_1 = ClientApplication {
            connect_via: None,
            app_name: "demo".into(),
            interval: 15500,
            environment: None,
            instance_id: None,
            sdk_version: Some("unleash-client-java:7.1.0".into()),
            started,
            strategies: vec!["default".into(), "gradualRollout".into()],
        };

        let demo_data_2 = ClientApplication {
            connect_via: Some(vec![ConnectVia {
                app_name: "unleash-edge".into(),
                instance_id: "2".into(),
            }]),
            app_name: "demo".into(),
            interval: 15500,
            environment: Some("production".into()),
            instance_id: None,
            sdk_version: None,
            started,
            strategies: vec!["default".into(), "CustomStrategy".into()],
        };
        let merged = demo_data_1.clone().merge(demo_data_2.clone());
        assert_eq!(demo_data_2.connect_via, merged.connect_via);
        let reverse_merge = demo_data_2.clone().merge(demo_data_1);
        assert_eq!(demo_data_2.connect_via, reverse_merge.connect_via);
    }

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
}
