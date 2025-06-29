use anyhow::{Result, bail};
use reqwest::blocking::Client;
use serde_json::Value;

use crate::types::{Taxon, TaxonContainer};

/// Finds taxon information from a string query
pub fn taxons(query: &str, limit: usize) -> Result<TaxonContainer> {
    let url = format!("https://api.ncbi.nlm.nih.gov/datasets/v1/gene/taxon_suggest/{query}",);

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
        .unwrap_or_default();

    if results.is_empty() {
        bail!(format!("No results found for query: {}", query))
    }
    Ok(TaxonContainer(results))
}

#[cfg(test)]
mod testing {
    use super::taxons;

    #[test]
    fn test_query() {
        let query = "human";
        let limit = 1;
        let response = taxons(query, limit).unwrap();
        assert_eq!(response.0.len(), 1);
        assert_eq!(response.0[0].tax_id, "9606");
    }

    #[test]
    fn test_query_nonsense() {
        let query = "asjdiawjdpansopdnawd";
        let limit = 1;
        let response = taxons(query, limit);
        assert!(response.is_err());
    }
}
