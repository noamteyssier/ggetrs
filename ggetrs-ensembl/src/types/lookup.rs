use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Struct to handle the results of an ID/symbol lookup
#[derive(Serialize, Deserialize, Debug)]
pub struct LookupResult {
    pub id: String,
    pub display_name: String,
    pub canonical_transcript: String,
    pub description: String,
    pub start: usize,
    pub end: usize,
    pub strand: isize,
    pub biotype: String,
    pub logic_name: String,
    pub object_type: String,
    pub version: usize,
    pub source: String,
    pub seq_region_name: String,
    pub assembly_name: String,
    pub species: String,
    pub db_type: String,
}
impl fmt::Display for LookupResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).expect("cannot serialize")
        )
    }
}

/// Container for multiple results of ID/symbol lookup
#[derive(Serialize, Deserialize, Debug)]
pub struct LookupResponse(pub HashMap<String, Option<LookupResult>>);
impl fmt::Display for LookupResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).expect("cannot serialize")
        )
    }
}
impl LookupResponse {
    #[must_use]
    pub fn get_id(&self, symbol: &str) -> Option<String> {
        if let Some(res) = self.0.get(symbol) {
            res.as_ref().map(|x| x.id.clone())
        } else {
            None
        }
    }
    #[must_use]
    pub fn get_symbol(&self, id: &str) -> Option<String> {
        if let Some(res) = self.0.get(id) {
            res.as_ref().map(|x| x.display_name.clone())
        } else {
            None
        }
    }
}
