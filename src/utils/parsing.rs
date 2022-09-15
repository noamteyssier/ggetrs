use serde_json::Value;

/// Parses a string from a json object
#[must_use] pub fn parse_primary_string(value: &Value, primary: &str) -> String {
    value[primary]
        .as_str()
        .unwrap_or_else(|| panic!("Missing: {}", primary))
        .to_string()
}

/// Parses an optional string from a json object
#[must_use] pub fn parse_primary_optional_string(value: &Value, primary: &str) -> Option<String> {
    value[primary]
        .as_str()
        .map(std::string::ToString::to_string)
}

/// Parses a usize from a json object
#[must_use] pub fn parse_primary_usize(value: &Value, primary: &str) -> usize {
    value[primary]
        .as_u64()
        .unwrap_or_else(|| panic!("Missing: {}", primary))
        as usize
}

/// Parses a usize from a json object from a secondary level
#[must_use] pub fn parse_secondary_string(value: &Value, primary: &str, secondary: &str) -> String {
    value[primary][secondary]
        .as_str()
        .unwrap_or_else(|| panic!("Missing: {}/{}", primary, secondary))
        .to_string()
}
