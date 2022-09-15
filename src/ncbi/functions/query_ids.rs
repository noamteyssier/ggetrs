use crate::ncbi::types::NcbiResults;
use reqwest::{blocking::Client, Result};
use serde_json::Value;

pub fn query_ids(ids: &[usize]) -> Result<NcbiResults> {
    let url = "https://api.ncbi.nlm.nih.gov/datasets/v1/gene/id";
    let query = ids
        .iter()
        .map(std::string::ToString::to_string)
        .collect::<Vec<String>>()
        .join("%2C");
    let query_url = format!("{}/{}", url, query);
    let response = Client::new()
        .get(query_url)
        .header("accept", "application/json")
        .header("api-key", "ggetrs")
        .send()?
        .json::<Value>()?;
    let results = NcbiResults::from_value(&response).expect("Could not parse NCBI Results");
    Ok(results)
}
