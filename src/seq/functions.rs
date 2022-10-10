use anyhow::Result;
use serde_json::json;
use reqwest::blocking::Client;
use super::ResultSeqContainer;

pub fn sequence(ensembl_ids: &Vec<String>) -> Result<ResultSeqContainer> {
    let ensembl_url = "http://rest.ensembl.org/sequence/id";
    let data = json!({
        "ids": ensembl_ids
    });
    let response: ResultSeqContainer = Client::new()
        .post(ensembl_url)
        .header("content-type", "application/json")
        .json(&data)
        .send()?
        .json::<ResultSeqContainer>()?;
        Ok(response)
}

#[cfg(test)]
mod testing {
    use super::sequence;

    #[test]
    fn test_seq_query() {
        let terms = vec!["ENSG00000131095".to_string()];
        let response = sequence(&terms);
        assert!(response.is_ok());
    }

    #[test]
    fn test_uniprot_nonsense_query() {
        let terms = vec!["AOSDKAPOWDNASD".to_string()];
        let response = sequence(&terms);
        assert!(response.is_err());
    }
}