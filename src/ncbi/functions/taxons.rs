use reqwest::blocking::Client;
use serde_json::Value;

use crate::ncbi::types::{TaxonContainer, Taxon};

pub fn taxons(query: &str, limit: usize) -> anyhow::Result<TaxonContainer> {
    let url = format!("https://api.ncbi.nlm.nih.gov/datasets/v1/gene/taxon_suggest/{}", query);
    let response = Client::new()
        .get(url)
        .header("accept", "application/json")
        .send()?
        .json::<Value>()?;
    let results = response["sci_name_and_ids"]
        .as_array()
        .map(|array| {
            array
                .iter()
                .take(limit)
                .map(|x| serde_json::from_value::<Taxon>(x.clone()).expect("Could not parse taxon"))
                .collect::<Vec<Taxon>>()
        })
        .unwrap_or(Vec::new());
    Ok(TaxonContainer(results))
}
