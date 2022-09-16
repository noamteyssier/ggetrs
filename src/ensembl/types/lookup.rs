use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt};

/// Struct to handle the results of an ID/symbol lookup
#[derive(Serialize, Deserialize, Debug)]
pub struct LookupResult {
    id: String,
    display_name: String,
    canonical_transcript: String,
    description: String,
    start: usize,
    end: usize,
    strand: isize,
    biotype: String,
    logic_name: String,
    object_type: String,
    version: usize,
    source: String,
    seq_region_name: String,
    assembly_name: String,
    species: String,
    db_type: String,
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
