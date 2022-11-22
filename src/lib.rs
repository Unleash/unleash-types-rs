pub mod client_features;

#[cfg(test)]
mod tests {
    use super::client_features::*;
    use serde_json::json;
    use std::fs;
    use test_case::test_case;

    #[test_case("01-simple-examples"; "can parse legacy format")]
    #[test_case("08-variants"; "can parse variants")]
    #[test_case("14-constraint-semver-operators"; "can parse advanced constraints")]
    #[test_case("15-global-constraints"; "can parse segments")]
    pub fn run_parse_test(file_path: &str) {
        let content = fs::read_to_string(format!("./examples/{}.json", file_path))
            .expect("Could not read file");
        serde_json::from_str::<ClientFeatures>(&content)
            .expect("Could not parse to expected format");
    }

    #[test]
    pub fn materializes_operator_in_constraint() {
        let string_constraint = json!({
            "contextName": "environment",
            "operator": "STRING_IS_IP_ADDRESS",
            "value": "bob",
        });

        let constraint: Result<Constraint, serde_json::Error> =
            serde_json::from_value(string_constraint);
        assert!(constraint.is_err());
    }
}
