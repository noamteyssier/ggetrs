use crate::enrichr::types::ResponseEnrich;
use reqwest::Result;

/// Performs an API call to the `Enrichr`'s `enrich`.
///
/// This measures the significance of overlap of the provided gene list to the provided library
/// name.
pub fn enrich(list_id: usize, library_name: &str) -> Result<ResponseEnrich> {
    let url = format!(
        "https://maayanlab.cloud/Enrichr/enrich?userListId={}&backgroundType={}",
        list_id, library_name
    );
    reqwest::blocking::get(url)?.json::<ResponseEnrich>()
}
