use crate::ensembl::lookup_symbol;

use super::ResultSeqContainer;
use anyhow::{bail, Result};
use reqwest::blocking::Client;
use serde_json::json;

fn retrieve_sequence(ensembl_ids: &Vec<String>) -> Result<ResultSeqContainer> {
    let ensembl_url = "http://rest.ensembl.org/sequence/id";
    let data = json!({ "ids": ensembl_ids });
    let response: ResultSeqContainer = Client::new()
        .post(ensembl_url)
        .header("content-type", "application/json")
        .json(&data)
        .send()?
        .json::<ResultSeqContainer>()?;
    Ok(response)
}

fn convert_to_ensembl_ids(symbols: &[String], species: &str) -> Result<Vec<String>> {
    let non_ensembl_ids = symbols.iter().filter(|x| !x.starts_with("ENS")).map(|x| x.to_string()).collect::<Vec<String>>();
    let response = lookup_symbol(&non_ensembl_ids, species)?;
    let ensembl_ids = symbols
        .iter()
        .map(|x| {
            if !x.starts_with("ENS") {
                response.get_id(x).expect(&format!("Unable to find ensembl id for symbol {x}"))
            } else {
                x.to_owned()
            }
        })
        .collect();
    Ok(ensembl_ids)
}

pub fn sequence(ensembl_ids: &Vec<String>, species: &Option<String>) -> Result<ResultSeqContainer> {
    if !ensembl_ids.iter().all(|x| x.starts_with("ENS")) {
        let species_name = if let Some(s) = species {
            s
        } else {
            bail!("Not all provided symbols are Ensembl IDs - so a species must be provided to identify them");
        };
        let linked_ids = convert_to_ensembl_ids(ensembl_ids, &species_name)?;
        retrieve_sequence(&linked_ids)
    } else {
        retrieve_sequence(ensembl_ids)
    }
}

#[cfg(test)]
mod testing {
    use super::sequence;

    #[test]
    fn test_seq_query() {
        let terms = vec!["ENSG00000131095".to_string()];
        let response = sequence(&terms, &None);
        assert!(response.is_ok());
    }

    #[test]
    fn test_uniprot_nonsense_query() {
        let terms = vec!["AOSDKAPOWDNASD".to_string()];
        let response = sequence(&terms, &None);
        assert!(response.is_err());
    }
}
