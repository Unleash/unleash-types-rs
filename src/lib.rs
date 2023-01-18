pub mod client_features;
pub mod client_metrics;
pub mod frontend;

#[cfg(test)]
mod tests {
    use super::client_features::*;
    use serde_json::json;
    use std::fs;
    use test_case::test_case;

    #[test_case("01-simple-examples"; "can parse legacy format")]
    #[test_case("08-variants"; "can parse variants")]
    #[test_case("13-constraint-operators"; "can parse advanced constraints")]
    #[test_case("14-constraint-semver-operators"; "can parse semver constraints")]
    #[test_case("15-global-constraints"; "can parse segments")]
    pub fn run_parse_test(file_path: &str) {
        let content = fs::read_to_string(format!("./examples/{}.json", file_path))
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
}
