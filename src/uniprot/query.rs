use futures::future::join_all;
use reqwest::{Result, Client};
use serde_json::Value;
use super::{UniprotInfo, UniprotInfoContainer};

/// An asynchronous function which performs a uniprot query
async fn async_query_uniprot(gene: &str) -> Result<Option<UniprotInfo>> {
    let query = if gene.starts_with("ENS") {
        gene.to_string()
    } else {
        format!("(gene:{})", gene)
    };
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
        .map(|x| UniprotInfo::from_value(x, gene))
}

/// An asynchronous function which joins all the handles from `async_query_uniprot`
async fn async_query_uniprot_multiple(ensembl_ids: &Vec<String>) -> Result<Vec<Result<Option<UniprotInfo>>>> {
    let query_handles = ensembl_ids
        .iter()
        .map(|x| async_query_uniprot(x));

    let results = join_all(query_handles).await;
    Ok(results)
}

/// A synchronous function to perform a query for each of the terms provided.
pub fn query(terms: &Vec<String>) -> anyhow::Result<UniprotInfoContainer> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    let results = rt.block_on(async move {
        async_query_uniprot_multiple(terms).await.expect("could not query uniprot")
    });
    let results = results
        .into_iter()
        .filter_map(|x| x.expect("could not create results"))
        .collect();
    Ok(UniprotInfoContainer(results))

}
