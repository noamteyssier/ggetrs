use reqwest::{blocking::Client, Result};
use crate::archs4::types::{Species, ResponseTissue};

/// Returns the tissue-specific expression of a provided `gene_name`
pub fn tissue(gene_name: &str, species: &Species) -> Result<ResponseTissue> {
    let client = Client::new();
    let query_string = format!("search={}&species={}&type=tissue", gene_name, species);

    let url = format!(
        "https://maayanlab.cloud/archs4/search/loadExpressionTissue.php?{}",
        query_string
    );

    let raw_response = client
        .post(url)
        .header("Content-Type", "application/json")
        .send()?
        .text()?;

    let response = ResponseTissue::from_str(&raw_response);

    Ok(response)
}
