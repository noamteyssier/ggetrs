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