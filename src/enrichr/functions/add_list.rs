use super::{ENRICHR_URL, SPEEDRICHR_URL};
use crate::enrichr::types::ResponseAddList;
use reqwest::{blocking::Client, Result};

/// Performs a function call to the `addList` API.
pub fn add_list(gene_list: &[String], speedrichr: bool) -> Result<ResponseAddList> {
    // defines the web client
    let client = Client::new();

    // defines the url (aka the API)
    let url = if speedrichr {
        format!("{SPEEDRICHR_URL}/api/addList")
    } else {
        format!("{ENRICHR_URL}/addList")
    };

    // join the gene-list with a newline
    let query = gene_list.join("\n");

    // basic description
    let description = String::from("rust-gget");

    // creates the form for the request
    let form = reqwest::blocking::multipart::Form::new()
        .text("list", query.clone())
        .text("description", description.clone());

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
        let gene_list = ["AP2S1", "NSD1", "LDB1"]
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        let response = add_list(&gene_list, false).unwrap();
        assert!(response.user_list_id > 1);
        assert!(response.short_id.len() > 1);
    }

    #[test]
    fn test_add_list_with_background() {
        let gene_list = ["AP2S1", "NSD1", "LDB1"]
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        let response = add_list(&gene_list, true).unwrap();
        assert!(response.user_list_id > 1);
        assert!(response.short_id.len() > 1);
    }
}
