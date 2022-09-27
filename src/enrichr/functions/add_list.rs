use crate::enrichr::types::ResponseAddList;
use reqwest::{blocking::Client, Result};

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

#[cfg(test)]
mod testing {
    use super::add_list;

    #[test]
    fn test_add_list() {
        let gene_list = vec!["AP2S1", "NSD1", "LDB1"]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let response = add_list(&gene_list).unwrap();
        assert!(response.user_list_id > 1);
        assert!(response.short_id.len() > 1);
    }
}
