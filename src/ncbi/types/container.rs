use std::collections::HashMap;

use super::NcbiInfo;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct NcbiResults(pub HashMap<String, NcbiInfo>);

impl NcbiResults {
    #[must_use]
    pub fn from_value(value: &Value) -> Option<Self> {
        let results = match value["genes"].as_array() {
            Some(array) => {
                let map = array
                    .iter()
                    .filter_map(NcbiInfo::from_value)
                    .map(|x| (x.query.to_string(), x))
                    .collect();
                Some(map)
            }
            None => None,
        };
        results.map(Self)
    }
}
