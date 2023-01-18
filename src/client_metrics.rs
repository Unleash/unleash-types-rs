use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default, Builder)]
pub struct ToggleStats {
    pub no: u64,
    pub yes: u64,
    pub variants: HashMap<String, u64>,
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
pub struct MetricBucket {
    pub start: DateTime<Utc>,
    pub stop: DateTime<Utc>,
    pub toggles: HashMap<String, ToggleStats>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct ClientMetrics {
    pub app_name: String,
    pub bucket: MetricBucket,
    pub environment: Option<String>,
    pub instance_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Builder)]
#[serde(rename_all = "camelCase")]
pub struct ClientApplication {
    pub app_name: String,
    pub environment: Option<String>,
    pub instance_id: Option<String>,
    pub interval: u64,
    pub sdk_version: Option<String>,
    pub started: DateTime<Utc>,
    pub strategies: Vec<String>,
}

impl ClientApplication {
    pub fn new(app_name: &str, interval: u64) -> Self {
        Self {
            app_name: app_name.into(),
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
            .chain(strategies.into_iter())
            .collect::<HashSet<String>>()
            .into_iter()
            .collect();
        self.strategies = unique_strats;
    }

    /// Will keep all set fields from self, overwriting None with Somes from other
    /// Will merge strategies from self and other, deduplicating
    pub fn merge(self, other: ClientApplication) -> ClientApplication {
        let mut merged_strategies: Vec<String> = self
            .strategies
            .into_iter()
            .chain(other.strategies.into_iter())
            .collect::<HashSet<String>>()
            .into_iter()
            .collect();
        merged_strategies.sort();

        ClientApplication {
            app_name: self.app_name,
            environment: self.environment.or(other.environment),
            instance_id: self.instance_id.or(other.instance_id),
            interval: self.interval,
            sdk_version: self.sdk_version.or(other.sdk_version),
            started: self.started,
            strategies: merged_strategies,
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::{ClientApplication, ToggleStats};

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
            app_name: "demo".into(),
            interval: 15500,
            environment: None,
            instance_id: None,
            sdk_version: Some("unleash-client-java:7.1.0".into()),
            started: started.clone(),
            strategies: vec!["default".into(), "gradualRollout".into()],
        };

        let demo_data_2 = ClientApplication {
            app_name: "demo".into(),
            interval: 15500,
            environment: Some("production".into()),
            instance_id: None,
            sdk_version: None,
            started,
            strategies: vec!["default".into(), "CustomStrategy".into()],
        };

        let left = demo_data_2.clone().merge(demo_data_1.clone());
        let right = demo_data_1.clone().merge(demo_data_2.clone());

        assert_eq!(left, right);
    }
}
