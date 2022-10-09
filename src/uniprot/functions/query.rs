use std::collections::HashMap;

use crate::uniprot::types::{UniprotInfo, UniprotInfoContainer, UniprotSeq, UniprotSeqContainer};
use anyhow::bail;
use futures::future::join_all;
use reqwest::{Client, Result};
use serde_json::Value;

fn build_query_string(gene: &str, taxon: &Option<usize>) -> String {
    let gene_query = if gene.starts_with("ENS") {
        gene.to_string()
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
async fn async_query_uniprot(gene: &str, taxon: &Option<usize>) -> Result<Option<UniprotInfo>> {
    let query = build_query_string(gene, taxon);
    let url = format!(
        "https://rest.uniprot.org/uniprotkb/search?query={}+AND+reviewed:true",
        query
    );
    Client::new()
        .get(url)
        .header("content-type", "application/json")
        .send()
        .await?
        .json::<Value>()
        .await
        .map(|x| UniprotInfo::from_value(&x, gene))
}

/// An asynchronous function which joins all the handles from `async_query_uniprot`
async fn async_query_uniprot_multiple(
    ensembl_ids: &[String],
    taxon: &Option<usize>,
) -> Result<Vec<Result<Option<UniprotInfo>>>> {
    let query_handles = ensembl_ids.iter().map(|x| async_query_uniprot(x, taxon));

    let results = join_all(query_handles).await;
    Ok(results)
}

/// A synchronous function to perform a query for each of the terms provided.
pub fn query(terms: &[String], taxon: &Option<usize>) -> anyhow::Result<UniprotInfoContainer> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    let results = rt
        .block_on(async move {
            async_query_uniprot_multiple(terms, taxon)
                .await
                .expect("could not query uniprot")
        })
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

/// An asynchronous function which performs a query for seq
async fn async_query_seq_uniprot(gene: &str, taxon: &Option<usize>) -> Result<Option<UniprotSeq>> {
    let query = build_query_string(gene, taxon);
    let url = format!(
        "https://rest.uniprot.org/uniprotkb/search?query={}+AND+reviewed:true",
        query
    );
    Client::new()
        .get(url)
        .header("content-type", "application/json")
        .send()
        .await?
        .json::<Value>()
        .await
        .map(|x| UniprotSeq::from_value(&x, gene))
}

/// An asynchronous function which joins all the handles from `async_query_seq_uniprot`
async fn async_query_seq_uniprot_multiple(
    ensembl_ids: &[String],
    taxon: &Option<usize>,
) -> Result<Vec<Result<Option<UniprotSeq>>>> {
    let query_handles = ensembl_ids.iter().map(|x| async_query_seq_uniprot(x, taxon));

    let results = join_all(query_handles).await;
    Ok(results)
}

/// A synchronous function to perform a query for each of the terms provided.
pub fn query_seq(terms: &[String], taxon: &Option<usize>) -> anyhow::Result<UniprotSeqContainer> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    let results = rt
        .block_on(async move {
            async_query_seq_uniprot_multiple(terms, taxon)
                .await
                .expect("could not query uniprot")
        })
        .into_iter()
        .filter_map(|x| x.expect("could not create results"))
        .map(|x| (x.query.to_string(), x))
        .collect::<HashMap<String, UniprotSeq>>();

    if results.len() > 0 {
        Ok(UniprotSeqContainer(results))
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
        let response = query(&terms, &taxon);
        assert!(response.is_ok());
    }

    #[test]
    fn test_uniprot_nonsense_query() {
        let terms = vec!["AOSDKAPOWDNASD".to_string()];
        let taxon = None;
        let response = query(&terms, &taxon);
        assert!(response.is_err());
    }
}
