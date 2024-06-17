use super::SPEEDRICHR_URL;
use crate::enrichr::types::ResponseAddBackground;
use reqwest::{blocking::Client, Result};

/// Performs a function call to the `addList` API.
pub fn add_background(gene_list: &[String]) -> Result<ResponseAddBackground> {
    // defines the web client
    let client = Client::new();

    // defines the url (aka the API)
    let url = format!("{SPEEDRICHR_URL}/api/addbackground");

    // join the gene-list with a newline
    let query = gene_list.join("\n");

    // creates the form for the request
    let form = reqwest::blocking::multipart::Form::new().text("background", query);

    // query the server
    client
        .post(url)
        .multipart(form)
        .send()?
        .json::<ResponseAddBackground>()
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn test_add_background() {
        let gene_list = ["AP2S1", "NSD1", "LDB1"]
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        let response = add_background(&gene_list).unwrap();
        assert!(response.backgroundid.len() > 1);
    }
}
