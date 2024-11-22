use crate::uniprot::types::{UniprotInfo, UniprotInfoContainer};
use anyhow::Result;
use futures::future::join_all;
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;

fn build_query_string(gene: &str, freeform: bool, taxon: Option<&usize>) -> String {
    let gene_query = if gene.starts_with("ENS") || freeform {
        format!("({gene})")
    } else {
        format!("(gene:{gene})")
    };
    let taxon_query = match taxon {
        Some(t) => {
            format!("AND(taxonomy_id:{t})")
        }
        None => String::new(),
    };
    format!("{gene_query}{taxon_query}")
}

/// An asynchronous function which performs a uniprot query
pub async fn async_query_uniprot(
    gene: &str,
    freeform: bool,
    taxon: &Option<usize>,
) -> Result<Option<UniprotInfo>> {
    let query = build_query_string(gene, freeform, taxon.as_ref());
    let url = format!("https://rest.uniprot.org/uniprotkb/search?query={query}+AND+reviewed:true",);
    let value = Client::new()
        .get(&url) // Updated to pass the URL by reference
        .header("content-type", "application/json")
        .send()
        .await?
        .json::<Value>()
        .await?;

    // Updated to check if the struct is non-empty
    let info = UniprotInfo::from_value(&value, gene)?;
    if let Some(uniprot_info) = info {
        if uniprot_info.is_non_empty() {
            return Ok(Some(uniprot_info));
        }
    }
    Ok(None) // Return None if the struct is empty
}

/// An asynchronous function which joins all the handles from `async_query_uniprot`
pub async fn async_query_uniprot_multiple(
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
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    let results = rt
        .block_on(async move {
            async_query_uniprot_multiple(terms, freeform, taxon)
                .await
                .expect("could not query uniprot")
        })
        .into_iter()
        .filter_map(|x| x.expect("could not create results"))
        .map(|x| (x.query.to_string(), x))
        .collect::<HashMap<String, UniprotInfo>>();

    Ok(if results.is_empty() {
        UniprotInfoContainer::default()
    } else {
        UniprotInfoContainer(results)
    })
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
        let uniprot_results = response.unwrap();
        assert!(uniprot_results.0.is_empty());
    }
}
