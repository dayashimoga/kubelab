use crate::models::{StateAssertion, ValidationOperator};
use regex::Regex;
use serde_json::Value;

pub fn extract_json_field<'a>(root: &'a Value, path: &str) -> Option<&'a Value> {
    let clean_path = path.trim().trim_start_matches('$').trim_start_matches('.');
    if clean_path.is_empty() {
        return Some(root);
    }
    let parts: Vec<&str> = clean_path.split('.').collect();
    let mut current = root;

    for part in parts {
        if part.ends_with(']') && part.contains('[') {
            let open_bracket = part.find('[')?;
            let close_bracket = part.find(']')?;
            let key = &part[..open_bracket];
            let index_str = &part[open_bracket + 1..close_bracket];
            let index: usize = index_str.parse().ok()?;

            if !key.is_empty() {
                current = current.get(key)?;
            }
            current = current.get(index)?;
        } else {
            current = current.get(part)?;
        }
    }

    Some(current)
}

pub fn evaluate_assertion(assertion: &mut StateAssertion, actual_state: &Value) -> bool {
    let actual_val = extract_json_field(actual_state, &assertion.field);
    assertion.actual = actual_val.cloned();

    let passed = match actual_val {
        None => false,
        Some(val) => match assertion.operator {
            ValidationOperator::Equals => val == &assertion.expected,
            ValidationOperator::Contains => {
                if let (Some(val_str), Some(exp_str)) = (val.as_str(), assertion.expected.as_str())
                {
                    val_str.contains(exp_str)
                } else if let (Some(val_arr), exp) = (val.as_array(), &assertion.expected) {
                    val_arr.contains(exp)
                } else {
                    false
                }
            }
            ValidationOperator::MatchesRegex => {
                if let (Some(val_str), Some(exp_regex)) =
                    (val.as_str(), assertion.expected.as_str())
                {
                    if let Ok(re) = Regex::new(exp_regex) {
                        re.is_match(val_str)
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            ValidationOperator::GreaterThan => {
                if let (Some(v), Some(e)) = (val.as_f64(), assertion.expected.as_f64()) {
                    v > e
                } else {
                    false
                }
            }
            ValidationOperator::LessThan => {
                if let (Some(v), Some(e)) = (val.as_f64(), assertion.expected.as_f64()) {
                    v < e
                } else {
                    false
                }
            }
            ValidationOperator::GreaterThanOrEqual => {
                if let (Some(v), Some(e)) = (val.as_f64(), assertion.expected.as_f64()) {
                    v >= e
                } else {
                    false
                }
            }
            ValidationOperator::LessThanOrEqual => {
                if let (Some(v), Some(e)) = (val.as_f64(), assertion.expected.as_f64()) {
                    v <= e
                } else {
                    false
                }
            }
            ValidationOperator::HttpGet | ValidationOperator::JsonpathMatch => {
                val == &assertion.expected
            }
        },
    };

    assertion.passed = Some(passed);
    if !passed {
        assertion.error_message = Some(format!(
            "Assertion failed on field '{}': expected '{:?}', got '{:?}'",
            assertion.field, assertion.expected, assertion.actual
        ));
    }
    passed
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_extract_json_field() {
        let data = json!({
            "status": {
                "phase": "Running",
                "containerStatuses": [
                    { "name": "web", "ready": true }
                ]
            },
            "metadata": {
                "labels": { "app": "frontend" }
            }
        });

        assert_eq!(
            extract_json_field(&data, "status.phase"),
            Some(&json!("Running"))
        );
        assert_eq!(
            extract_json_field(&data, "metadata.labels.app"),
            Some(&json!("frontend"))
        );
        assert_eq!(
            extract_json_field(&data, "status.containerStatuses[0].ready"),
            Some(&json!(true))
        );
    }

    #[test]
    fn test_evaluate_assertion_equals() {
        let state = json!({ "status": { "phase": "Running" } });
        let mut assertion = StateAssertion {
            field: "status.phase".to_string(),
            operator: ValidationOperator::Equals,
            expected: json!("Running"),
            actual: None,
            passed: None,
            error_message: None,
        };

        assert!(evaluate_assertion(&mut assertion, &state));
        assert_eq!(assertion.passed, Some(true));
    }

    #[test]
    fn test_evaluate_assertion_regex() {
        let state = json!({ "spec": { "containers": [{ "image": "nginx:1.25.1-alpine" }] } });
        let mut assertion = StateAssertion {
            field: "spec.containers[0].image".to_string(),
            operator: ValidationOperator::MatchesRegex,
            expected: json!(r"^nginx:.*alpine$"),
            actual: None,
            passed: None,
            error_message: None,
        };

        assert!(evaluate_assertion(&mut assertion, &state));
    }
}
