use super::NcbiInfo;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct NcbiResults {
    results: Vec<NcbiInfo>,
}
impl NcbiResults {
    #[must_use]
    pub fn from_value(value: &Value) -> Option<Self> {
        let results = match value["genes"].as_array() {
            Some(x) => x.iter().filter_map(NcbiInfo::from_value).collect(),
            None => return None,
        };
        Some(Self { results })
    }
}
