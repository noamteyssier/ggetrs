use crate::enrichr::types::ResponseEnrich;
use reqwest::Result;

use super::shorthand;

/// Performs an API call to the `Enrichr`'s `enrich`.
///
/// This measures the significance of overlap of the provided gene list to the provided library
/// name.
pub fn enrich(list_id: usize, library_name: &str) -> Result<ResponseEnrich> {
    let alias = shorthand(library_name);
    let url = format!(
        "https://maayanlab.cloud/Enrichr/enrich?userListId={}&backgroundType={}",
        list_id, alias
    );
    reqwest::blocking::get(url)?.json::<ResponseEnrich>()
}

#[cfg(test)]
mod testing {
    use super::enrich;
    use crate::enrichr::add_list;

    fn get_list_id() -> usize {
        let gene_list = vec!["AP2S1", "NSD1", "LDB1"]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let response = add_list(&gene_list).unwrap();
        response.user_list_id
    }

    #[test]
    fn test_enrich() {
        let user_list_id = get_list_id();
        let library_name = "KEGG_2015";
        let response = enrich(user_list_id, library_name).unwrap();
        assert!(response.0.contains_key(library_name));
    }
}
