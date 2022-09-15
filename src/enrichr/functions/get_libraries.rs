use reqwest::Result;
use crate::enrichr::types::ResponseLibraries;

/// Performs a `GET` call to retrieve the known libraries of `Enrichr`
pub fn get_libraries() -> Result<ResponseLibraries> {
    let url = "https://maayanlab.cloud/Enrichr/datasetStatistics";
    reqwest::blocking::get(url)?.json::<ResponseLibraries>()
}
