use reqwest::{blocking::Client, Result};
use serde::{Deserialize, Serialize};
use std::fmt;

/// A struct to handle the results of the `Enrichr` function call: `addList`
///
/// details at: <https://maayanlab.cloud/Enrichr/help#api&q=1>
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseAddList {
    pub user_list_id: usize,
    pub short_id: String,
}
impl fmt::Display for ResponseAddList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).expect("cannot serialize")
        )
    }
}

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
