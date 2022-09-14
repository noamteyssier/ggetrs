use reqwest::{Result, blocking::Client};
use serde_json::Value;
use super::structs::NcbiResults;

pub fn query_ncbi_ids(ids: &Vec<usize>) -> Result<NcbiResults> {
    let url = "https://api.ncbi.nlm.nih.gov/datasets/v1/gene/id";
    let query = ids
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("%2C");
    let query_url = format!("{}/{}", url, query);
    let response = Client::new()
        .get(query_url)
        .header("accept", "application/json")
        .header("api-key", "ggetrs")
        .send()?
        .json::<Value>()?;
    let results = NcbiResults::from_value(&response)
        .expect("Could not parse NCBI Results");
    Ok(results)
}

pub fn query_ncbi_symbols(symbols: &Vec<String>, taxon_id: usize) -> Result<NcbiResults> {
    let query = symbols.join("%2C");
    let query_url = format!(
        "https://api.ncbi.nlm.nih.gov/datasets/v1/gene/symbol/{}/taxon/{}?",
        query,
        taxon_id
    );
    let response = Client::new()
        .get(query_url)
        .header("accept", "application/json")
        .header("api-key", "ggetrs")
        .send()?
        .json::<Value>()?;
    let results = NcbiResults::from_value(&response)
        .expect("Could not parse NCBI Results");
    Ok(results)
}
