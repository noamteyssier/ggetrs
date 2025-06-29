use reqwest::{Result, blocking::Client};
use serde_json::json;

use crate::types::LookupResponse;

/// Lookup gene symbols in a batched manner.
///
/// Limited to 1000 symbols at once.
///
/// API documentation found here:
/// <https://rest.ensembl.org/documentation/info/symbol_post>
pub fn lookup_symbol(symbols: &[String], species: &str) -> Result<LookupResponse> {
    let url = format!("https://rest.ensembl.org/lookup/symbol/{species}");
    let data = json!({ "symbols": symbols });
    let results = Client::new()
        .post(url)
        .header("content-type", "application/json")
        .header("accept", "application/json")
        .json(&data)
        .send()?
        .json::<LookupResponse>()?;
    Ok(results)
}

/// Asynchronous version of `lookup_symbol`
///
/// Limited to 1000 symbols at once.
///
/// API documentation found here:
/// <https://rest.ensembl.org/documentation/info/symbol_post>
pub async fn async_lookup_symbol(symbols: &[String], species: &str) -> Result<LookupResponse> {
    let url = format!("https://rest.ensembl.org/lookup/symbol/{species}");
    let data = json!({ "symbols": symbols });
    let results = reqwest::Client::new()
        .post(url)
        .header("content-type", "application/json")
        .header("accept", "application/json")
        .json(&data)
        .send()
        .await?
        .json::<LookupResponse>()
        .await?;
    Ok(results)
}

#[cfg(test)]
mod testing {
    use super::lookup_symbol;

    #[test]
    fn test_ensembl_lookup_symbol() {
        let ensembl_symbols = vec!["AP2S1".to_string()];
        let species = "homo_sapiens";
        let response = lookup_symbol(&ensembl_symbols, species).unwrap();
        assert_eq!(response.0.len(), 1);
        assert!(response.0.get("AP2S1").unwrap().is_some());
    }

    #[test]
    fn test_ensembl_lookup_symbol_nonsense() {
        let ensembl_symbols = vec!["AWDIAJWIDJIAWD".to_string()];
        let species = "homo_sapiens";
        let response = lookup_symbol(&ensembl_symbols, species).unwrap();
        assert_eq!(response.0.len(), 0);
    }

    #[test]
    fn test_ensembl_lookup_symbol_false_species() {
        let ensembl_symbols = vec!["AWDIAJWIDJIAWD".to_string()];
        let species = "aokdowaodawd";
        let response = lookup_symbol(&ensembl_symbols, species).unwrap();
        assert_eq!(response.0.len(), 0);
    }
}
