use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct TaxonContainer(pub Vec<Taxon>);
impl fmt::Display for TaxonContainer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).expect("cannot serialize")
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Taxon {
    pub sci_name: String,
    pub tax_id: String,
    pub common_name: Option<String>,
    pub matched_term: Option<String>,
}
impl fmt::Display for Taxon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).expect("cannot serialize")
        )
    }
}
