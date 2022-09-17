use serde::{Serialize, Deserialize};
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
    sci_name: String,
    tax_id: String,
    common_name: Option<String>,
    matched_term: Option<String>,
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
