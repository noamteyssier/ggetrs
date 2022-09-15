use crate::ensembl::types::LookupResponse;
use reqwest::{blocking::Client, Result};
use serde_json::json;

/// Lookup gene symbols in a batched manner.
///
/// Limited to 1000 symbols at once.
///
/// API documentation found here:
/// <https://rest.ensembl.org/documentation/info/symbol_post>
pub fn lookup_symbol(symbols: &[String], species: &str) -> Result<LookupResponse> {
    let url = format!("https://rest.ensembl.org/lookup/symbol/{}", species);
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
