use crate::ncbi::types::NcbiResults;
use anyhow::{bail, Result};
use reqwest::blocking::Client;
use serde_json::Value;

/// Query information given gene symbols
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

    match NcbiResults::from_value(&response) {
        Some(result) => {
            if result.0.len() > 0 {
                Ok(result)
            } else {
                bail!(format!("No results found for symbols: {:?}", symbols))
            }
        }
        None => bail!("Unable to parse response from NCBI"),
    }
}

#[cfg(test)]
mod testing {
    use super::query_symbols;

    #[test]
    fn test_ncbi_symbols() {
        let symbols = vec!["AP2S1".to_string()];
        let taxon_id = 9606;
        let response = query_symbols(&symbols, taxon_id);
        assert!(response.is_ok());
    }

    #[test]
    fn test_ncbi_symbols_missing() {
        let symbols = vec!["BLAHBLAHBLAH".to_string()];
        let taxon_id = 9606;
        let response = query_symbols(&symbols, taxon_id);
        assert!(response.is_err());
    }

    #[test]
    fn test_ncbi_symbols_wrong_taxon() {
        let symbols = vec!["AP2S1".to_string()];
        let taxon_id = 0000;
        let response = query_symbols(&symbols, taxon_id);
        assert!(response.is_err());
    }
}
