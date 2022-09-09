use pyo3::pyclass;
use reqwest::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// A struct to hold the results of an enrichment test.
///
/// The keys of this `HashMap` will be the background library
/// tested against and the values will each be an instance of [ResultEnrichr]
#[derive(Serialize, Deserialize, Debug)]
#[pyclass]
pub struct ResponseEnrich (
    pub HashMap<String, Vec<ResultEnrichr>>
);
impl fmt::Display for ResponseEnrich {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).expect("cannot serialize"))
    }
}

/// A singular enrichment result.
///
/// Names were taken from <https://maayanlab.cloud/Enrichr/help#api&q=3>
#[derive(Serialize, Deserialize, Debug)]
#[pyclass(dict)]
pub struct ResultEnrichr {
    #[pyo3(get, set)]
    pub rank: usize,
    #[pyo3(get, set)]
    pub term_name: String,
    #[pyo3(get, set)]
    pub pvalue: f64,
    #[pyo3(get, set)]
    pub zscore: f64,
    #[pyo3(get, set)]
    pub combined_score: f64,
    #[pyo3(get, set)]
    pub overlapping_genes: Vec<String>,
    #[pyo3(get, set)]
    pub adj_pvalue: f64,
    #[pyo3(get, set)]
    pub old_pvalue: f64,
    #[pyo3(get, set)]
    pub old_adj_pvalue: f64
}
impl fmt::Display for ResultEnrichr{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).expect("cannot serialize"))
    }
}

/// Performs an API call to the `Enrichr`'s `enrich`. 
///
/// This measures the significance of overlap of the provided gene list to the provided library
/// name.
pub fn enrich(list_id: usize, library_name: &str) -> Result<ResponseEnrich> {
    let url = format!(
        "https://maayanlab.cloud/Enrichr/enrich?userListId={}&backgroundType={}",
        list_id,
        library_name
    );
    reqwest::blocking::get(url)?
        .json::<ResponseEnrich>()
}
