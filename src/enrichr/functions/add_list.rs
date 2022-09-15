use reqwest::{blocking::Client, Result};
use crate::enrichr::types::ResponseAddList;

/// Performs a function call to the `addList` API.
pub fn add_list(gene_list: &[String]) -> Result<ResponseAddList> {
    // defines the web client
    let client = Client::new();

    // defines the url (aka the API)
    let url = "https://maayanlab.cloud/Enrichr/addList";

    // join the gene-list with a newline
    let query = gene_list.join("\n");

    // basic description
    let description = String::from("rust-gget");

    // creates the form for the request
    let form = reqwest::blocking::multipart::Form::new()
        .text("list", query)
        .text("description", description);

    // query the server
    client
        .post(url)
        .multipart(form)
        .send()?
        .json::<ResponseAddList>()
}
