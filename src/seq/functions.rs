use crate::ensembl::{lookup_symbol, types::LookupResponse};

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

/// recovers all non ensembl ids from list of symbols
fn strip_symbols(symbols: &[String]) -> Vec<String> {
    symbols
        .iter()
        .filter(|x| !x.starts_with("ENS"))
        .map(|x| x.to_string())
        .collect()
}

/// Validates all non ensembl ids are found in lookup response
fn validate_full_recovery(non_ensembl_ids: &[String], response: &LookupResponse) -> Result<()> {
    for n in non_ensembl_ids {
        if response.get_id(&n).is_none() {
            bail!(format!("Unable to find ensembl id for symbol {n}"));
        }
    }
    Ok(())
}

/// recover ensembl ids from response
fn recover_ensembl_ids(symbols: &[String], response: &LookupResponse) -> Vec<String> {
    symbols
        .iter()
        .map(|x| {
            if !x.starts_with("ENS") {
                response.get_id(x).unwrap() // unwrap okay because I validate before calling this
            } else {
                x.to_owned()
            }
        })
        .collect()
}

/// convert any non-ensembl ids to ensembl ids
fn convert_to_ensembl_ids(symbols: &[String], species: &str) -> Result<Vec<String>> {
    let non_ensembl_ids = strip_symbols(symbols);
    let response = lookup_symbol(&non_ensembl_ids, species)?;
    validate_full_recovery(&non_ensembl_ids, &response)?;
    Ok(recover_ensembl_ids(symbols, &response))
}

pub fn sequence(
    search_terms: &Vec<String>,
    species: &Option<String>,
) -> Result<ResultSeqContainer> {
    // case where not all search terms are ensembl ids
    if !search_terms.iter().all(|x| x.starts_with("ENS")) {
        let species_name = if let Some(s) = species {
            s
        } else {
            bail!("Not all provided symbols are Ensembl IDs - so a species must be provided to identify them");
        };
        let ensembl_ids = convert_to_ensembl_ids(search_terms, &species_name)?;
        retrieve_sequence(&ensembl_ids)
    } else {
        retrieve_sequence(search_terms)
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

    #[test]
    fn test_seq_query_non_ensembl() {
        let terms = vec!["AP2S1".to_string()];
        let response = sequence(&terms, &Some("homo_sapiens".to_string()));
        assert!(response.is_ok());
    }

    #[test]
    fn test_seq_query_non_ensembl_missing_species() {
        let terms = vec!["AP2S1".to_string()];
        let response = sequence(&terms, &None);
        assert!(response.is_err());
    }

    #[test]
    fn test_seq_query_non_ensembl_random_gene_name() {
        let terms = vec!["SOMERANDOMGENENAME".to_string()];
        let response = sequence(&terms, &Some("homo_sapiens".to_string()));
        assert!(response.is_err());
    }

    #[test]
    fn test_seq_query_mixed_symbols() {
        let terms = vec!["ENSG00000131095".to_string(), "AP2S1".to_string()];
        let response = sequence(&terms, &Some("homo_sapiens".to_string()));
        assert!(response.is_ok());
    }
}
