use crate::enrichr::types::ResponseAddBackground;
use reqwest::{blocking::Client, Result};
use super::SPEEDRICHR_URL;


/// Performs a function call to the `addList` API.
pub fn add_background(gene_list: &[String]) -> Result<ResponseAddBackground> {
    // defines the web client
    let client = Client::new();

    // defines the url (aka the API)
    let url = format!("{}/api/addbackground", SPEEDRICHR_URL);

    // join the gene-list with a newline
    let query = gene_list.join("\n");

    // creates the form for the request
    let form = reqwest::blocking::multipart::Form::new()
        .text("background", query);

    // query the server
    client
        .post(url)
        .multipart(form)
        .send()?
        .json::<ResponseAddBackground>()
}

