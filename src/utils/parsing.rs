use serde_json::Value;

/// Parses a string from a json object
pub fn parse_primary_string(value: &Value, primary: &str) -> String {
    value[primary]
        .as_str()
        .expect(&format!("Missing: {}", primary))
        .to_string()
}

/// Parses an optional string from a json object
pub fn parse_primary_optional_string(value: &Value, primary: &str) -> Option<String> {
    value[primary]
        .as_str()
        .map(|x| x.to_string())
}

/// Parses a usize from a json object
pub fn parse_primary_usize(value: &Value, primary: &str) -> usize {
    value[primary]
        .as_u64()
        .expect(&format!("Missing: {}", primary))
        as usize
}

/// Parses a usize from a json object from a secondary level
pub fn parse_secondary_string(value: &Value, primary: &str, secondary: &str) -> String {
    value[primary][secondary]
        .as_str()
        .expect(&format!("Missing: {}/{}", primary, secondary))
        .to_string()
}
