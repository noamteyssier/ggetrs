use crate::ensembl::types::LookupResponse;
use reqwest::{blocking::Client, Result};
use serde_json::json;

/// Lookup ensembl ids in a batched manner.
///
/// Limited to 1000 `ensembl_ids` at once.
///
/// API documentation found here:
/// <https://rest.ensembl.org/documentation/info/lookup_post>
pub fn lookup_id(ensembl_ids: &[String]) -> Result<LookupResponse> {
    let url = "https://rest.ensembl.org/lookup/id";
    let data = json!({ "ids": ensembl_ids });
    let results = Client::new()
        .post(url)
        .header("content-type", "application/json")
        .header("accept", "application/json")
        .json(&data)
        .send()?
        .json::<LookupResponse>()?;
    Ok(results)
}

#[cfg(test)]
mod testing {
    use super::lookup_id;

    #[test]
    fn test_ensembl_lookup_id() {
        let ensembl_ids = vec!["ENSG00000042753".to_string()]; // AP2S1
        let response = lookup_id(&ensembl_ids).unwrap();
        assert_eq!(response.0.len(), 1);
        assert!(response.0.get("ENSG00000042753").unwrap().is_some())
    }

    #[test]
    fn test_ensembl_lookup_id_nonsense() {
        let ensembl_ids = vec!["AWDIAJWIDJIAWD".to_string()];
        let response = lookup_id(&ensembl_ids).unwrap();
        assert_eq!(response.0.len(), 1);
        assert!(response.0.get("AWDIAJWIDJIAWD").unwrap().is_none())
    }
}
