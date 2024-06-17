use super::{shorthand, ENRICHR_URL, SPEEDRICHR_URL};
use crate::enrichr::types::ResponseEnrich;
use anyhow::Result;
use reqwest::blocking::Client;

/// Performs an API call to the `Enrichr`'s `enrich`.
///
/// This measures the significance of overlap of the provided gene list to the provided library
/// name.
pub fn enrich(
    list_id: usize,
    library_name: &str,
    background_id: Option<&str>,
) -> Result<ResponseEnrich> {
    let alias = shorthand(library_name);
    let url = if background_id.is_some() {
        format!("{SPEEDRICHR_URL}/api/backgroundenrich")
    } else {
        format!("{ENRICHR_URL}/enrich")
    };

    if let Some(background_id) = background_id {
        let form = reqwest::blocking::multipart::Form::new()
            .text("userListId", list_id.to_string())
            .text("backgroundid", background_id.to_string())
            .text("backgroundType", alias.to_string());
        let client = Client::new();
        // Go through intermediate `text` to replace `Infinity` with `f64::MIN_POSITIVE`.
        let text = client
            .post(url.clone())
            .multipart(form)
            .send()?
            .text()?
            .replace("Infinity", format!("{:e}", f64::MIN_POSITIVE).as_str());
        // Parse the JSON from a string.
        Ok(serde_json::from_str::<ResponseEnrich>(&text)?)
    } else {
        let request_url =
            format!("{ENRICHR_URL}/enrich?userListId={list_id}&backgroundType={alias}");
        let client = Client::new();
        // Go through intermediate `text` to replace `Infinity` with `f64::MIN_POSITIVE`.
        let text = client
            .get(request_url)
            .send()?
            .text()?
            .replace("Infinity", format!("{:e}", f64::MIN_POSITIVE).as_str());
        // Parse the JSON from a string.
        Ok(serde_json::from_str::<ResponseEnrich>(&text)?)
    }
}

#[cfg(test)]
mod testing {
    use super::enrich;
    use crate::enrichr::functions::{add_background, add_list};

    fn get_list_id() -> usize {
        let gene_list = ["AP2S1", "NSD1", "LDB1"]
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        let response = add_list(&gene_list, false).unwrap();
        response.user_list_id
    }

    fn get_list_id_with_background() -> usize {
        let gene_list = ["AP2S1", "NSD1", "LDB1"]
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        let response = add_list(&gene_list, true).unwrap();
        response.user_list_id
    }

    fn get_background_id() -> String {
        let gene_list = ["AP2S1", "NSD1", "LDB1", "RFX3"]
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        let response = add_background(&gene_list).unwrap();
        response.backgroundid
    }

    #[test]
    fn test_enrich() {
        let user_list_id = get_list_id();
        let library_name = "KEGG_2015";
        let response = enrich(user_list_id, library_name, None).unwrap();
        assert!(response.0.contains_key(library_name));
    }

    #[test]
    fn test_enrich_with_background() {
        let user_list_id = get_list_id_with_background();
        let background_id = get_background_id();
        let library_name = "KEGG_2015";
        let response = enrich(user_list_id, library_name, Some(&background_id)).unwrap();
        assert!(response.0.contains_key(library_name));
    }
}
