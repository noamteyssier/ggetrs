use crate::ncbi::types::NcbiResults;
use reqwest::{blocking::Client, Result};
use serde_json::Value;

pub fn query_symbols(symbols: &[String], taxon_id: usize) -> Result<NcbiResults> {
    let query = symbols.join("%2C");
    let query_url = format!(
        "https://api.ncbi.nlm.nih.gov/datasets/v1/gene/symbol/{}/taxon/{}?",
        query, taxon_id
    );
    let response = Client::new()
        .get(query_url)
        .header("accept", "application/json")
        .header("api-key", "ggetrs")
        .send()?
        .json::<Value>()?;
    let results = NcbiResults::from_value(&response).expect("Could not parse NCBI Results");
    Ok(results)
}

