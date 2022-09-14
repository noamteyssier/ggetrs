use serde::{Serialize, Deserialize};
use serde_json::Value;
use super::NcbiInfo;

#[derive(Debug, Deserialize, Serialize)]
pub struct NcbiResults {
    results: Vec<NcbiInfo>
}
impl NcbiResults {
    pub fn from_value(value: &Value) -> Option<Self> {
        let results = match value["genes"].as_array() {
            Some(x) => {
                x.iter().filter_map(|x| NcbiInfo::from_value(x)).collect()
            },
            None => return None
        };
        Some(Self{results})
    }
}
