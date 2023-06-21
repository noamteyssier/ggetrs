use crate::uniprot::types::{UniprotInfo, UniprotInfoContainer};
use anyhow::{bail, Result};
use futures::{future::join_all, executor::block_on};
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;

fn build_query_string(gene: &str, freeform: bool, taxon: &Option<usize>) -> String {
    let gene_query = if gene.starts_with("ENS") || freeform {
        format!("({gene})")
    } else {
        format!("(gene:{})", gene)
    };
    let taxon_query = match taxon {
        Some(t) => {
            format!("AND(taxonomy_id:{})", t)
        }
        None => String::new(),
    };
    format!("{}{}", gene_query, taxon_query)
}

/// An asynchronous function which performs a uniprot query
async fn async_query_uniprot(
    gene: &str,
    freeform: bool,
    taxon: &Option<usize>,
) -> Result<Option<UniprotInfo>> {
    let query = build_query_string(gene, freeform, taxon);
    let url = format!(
        "https://rest.uniprot.org/uniprotkb/search?query={}+AND+reviewed:true",
        query
    );
    let value = Client::new()
        .get(url)
        .header("content-type", "application/json")
        .send()
        .await?
        .json::<Value>()
        .await?;
    let info = UniprotInfo::from_value(&value, gene)?;
    Ok(info)
}

/// An asynchronous function which joins all the handles from `async_query_uniprot`
async fn async_query_uniprot_multiple(
    ensembl_ids: &[String],
    freeform: bool,
    taxon: &Option<usize>,
) -> Result<Vec<Result<Option<UniprotInfo>>>> {
    let query_handles = ensembl_ids
        .iter()
        .map(|x| async_query_uniprot(x, freeform, taxon));
    let results = join_all(query_handles).await;
    Ok(results)
}

/// A synchronous function to perform a query for each of the terms provided.
pub fn query(
    terms: &[String],
    freeform: bool,
    taxon: &Option<usize>,
) -> anyhow::Result<UniprotInfoContainer> {
    let async_results = block_on(
        async_query_uniprot_multiple(terms, freeform, taxon)
    )?;
    let results = async_results
        .into_iter()
        .filter_map(|x| x.expect("could not create results"))
        .map(|x| (x.query.to_string(), x))
        .collect::<HashMap<String, UniprotInfo>>();

    if results.len() > 0 {
        Ok(UniprotInfoContainer(results))
    } else {
        bail!(format!("Found no results for terms: {:?}", terms))
    }
}

#[cfg(test)]
mod testing {
    use super::query;

    #[test]
    fn test_uniprot_query() {
        let terms = vec!["AP2S1".to_string()];
        let taxon = None;
        let response = query(&terms, false, &taxon);
        assert!(response.is_ok());
    }

    #[test]
    fn test_uniprot_nonsense_query() {
        let terms = vec!["AOSDKAPOWDNASD".to_string()];
        let taxon = None;
        let response = query(&terms, false, &taxon);
        assert!(response.is_err());
    }
}
