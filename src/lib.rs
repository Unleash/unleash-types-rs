pub mod client_features;
pub mod client_metrics;
pub mod frontend;
use std::{collections::HashSet, hash::Hash};

pub trait Merge {
    fn merge(self, other: Self) -> Self;
}

pub trait Upsert {
    /// If same entry exists in both self and other, should keep the one in other, if entry only exists in one, keep it.
    fn upsert(self, other: Self) -> Self;
}

pub trait Deduplicate<T>
where
    T: Hash + Eq,
{
    fn deduplicate(self) -> Self;
}

impl<T> Deduplicate<T> for Vec<T>
where
    T: Hash + Eq,
{
    fn deduplicate(self) -> Self {
        self.into_iter()
            .collect::<HashSet<T>>()
            .into_iter()
            .collect()
    }
}

impl<T> Upsert for Vec<T>
where
    T: Hash + Eq,
{
    fn upsert(self, other: Self) -> Self {
        let mut upserted = other;
        upserted.extend(self);
        upserted.deduplicate()
    }
}

impl<T> Merge for Vec<T>
where
    T: Hash + Eq,
{
    fn merge(self, other: Self) -> Self {
        let mut merged = self;
        merged.extend(other);
        merged.deduplicate()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Deduplicate, Merge};

    use super::client_features::*;
    use serde_json::json;
    use std::fs;
    use test_case::test_case;

    #[test_case("01-simple-examples"; "can parse legacy format")]
    #[test_case("08-variants"; "can parse variants")]
    #[test_case("13-constraint-operators"; "can parse advanced constraints")]
    #[test_case("14-constraint-semver-operators"; "can parse semver constraints")]
    #[test_case("15-global-constraints"; "can parse segments")]
    #[test_case("features_with_variantType"; "can handle weightType being part of content")]
    #[test_case("16-strategy-variants"; "can parse strategy variants")]
    pub fn run_parse_test(file_path: &str) {
        let content = fs::read_to_string(format!("./examples/{file_path}.json"))
            .expect("Could not read file");
        serde_json::from_str::<ClientFeatures>(&content)
            .expect("Could not parse to expected format");
    }

    #[test]
    pub fn materializes_invalid_operator_in_constraint() {
        let string_constraint = json!({
            "contextName": "environment",
            "operator": "STRING_IS_IP_ADDRESS",
            "value": "bob",
        });

        let constraint: Constraint = serde_json::from_value(string_constraint).unwrap();
        assert_eq!(
            constraint.operator,
            Operator::Unknown("STRING_IS_IP_ADDRESS".into())
        );
    }

    #[test_case("NOT_IN", Operator::NotIn)]
    #[test_case("IN", Operator::In)]
    #[test_case("STR_ENDS_WITH", Operator::StrEndsWith)]
    #[test_case("STR_STARTS_WITH", Operator::StrStartsWith)]
    #[test_case("STR_CONTAINS", Operator::StrContains)]
    #[test_case("NUM_EQ", Operator::NumEq)]
    #[test_case("NUM_GT", Operator::NumGt)]
    #[test_case("NUM_GTE", Operator::NumGte)]
    #[test_case("NUM_LT", Operator::NumLt)]
    #[test_case("NUM_LTE", Operator::NumLte)]
    #[test_case("DATE_AFTER", Operator::DateAfter)]
    #[test_case("DATE_BEFORE", Operator::DateBefore)]
    #[test_case("SEMVER_EQ", Operator::SemverEq)]
    #[test_case("SEMVER_LT", Operator::SemverLt)]
    #[test_case("SEMVER_GT", Operator::SemverGt)]
    pub fn parses_constraint_operators_correctly(operator: &str, expected: Operator) {
        let string_constraint = json!({
            "contextName": "environment",
            "operator": operator,
            "value": "bob",
        });

        let operator: Constraint = serde_json::from_value(string_constraint).unwrap();
        assert_eq!(operator.operator, expected)
    }

    #[test]
    fn deserializing_context_strips_null_properties_correctly() {
        let json_blob = json!({
            "userId": "some-user-id",
            "properties": {
                "lies": null,
                "truths": "something is something"
            }
        });

        let context: Context = serde_json::from_value(json_blob).unwrap();
        assert_eq!(context.properties.unwrap().len(), 1);
    }

    #[test]
    fn calling_deduplicate_correctly_removes_duplicated_items() {
        let first = vec![3, 2, 1];
        let second = vec![3, 4, 5];

        let result = first
            .into_iter()
            .chain(second.into_iter())
            .collect::<Vec<u32>>()
            .deduplicate();
        assert!(result.len() == 5);
    }

    #[test]
    fn merging_also_deduplicates() {
        let first = vec![3, 2, 1];
        let second = vec![3, 4, 5];

        let result = first.merge(second);

        assert!(result.len() == 5);
    }

    #[test]
    fn merging_unique_lists_keeps_everything() {
        let first = vec![1, 2, 3];
        let second = vec![4, 5, 6];

        let result = first.merge(second);

        assert!(result.len() == 6);
    }
}
