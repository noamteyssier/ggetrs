use reqwest::{Result, blocking::Client};
use serde_json::json;
use crate::ensembl::types::LookupResponse;

/// Lookup ensembl ids in a batched manner. 
///
/// Limited to 1000 ensembl_ids at once.
///
/// API documentation found here:
/// <https://rest.ensembl.org/documentation/info/lookup_post>
pub fn lookup_id(ensembl_ids: &[String]) -> Result<LookupResponse> {
    let url = "https://rest.ensembl.org/lookup/id";
    let data = json!({"ids": ensembl_ids});
    let results = Client::new()
        .post(url)
        .header("content-type", "application/json")
        .header("accept", "application/json")
        .json(&data)
        .send()?
        .json::<LookupResponse>()?;
    Ok(results)
}
