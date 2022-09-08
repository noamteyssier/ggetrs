use reqwest::{Client, Error};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct ResponseAddList {
    pub user_list_id: usize,
    pub short_id: String
}

pub async fn add_list(gene_list: &[String]) -> Result<ResponseAddList, Error> {

    // defines the web client
    let client = Client::new();

    // defines the url (aka the API)
    let url = "https://maayanlab.cloud/Enrichr/addList";

    // join the gene-list with a newline
    let query = gene_list.join("\n");

    // basic description
    let description = String::from("rust-gget");

    // creates the form for the request
    let form = reqwest::multipart::Form::new()
        .text("list", query)
        .text("description", description);

    // query the server
    client.post(url)
        .multipart(form)
        .send()
        .await?
        .json::<ResponseAddList>()
        .await

}

