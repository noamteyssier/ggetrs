use reqwest::Result;
use serde::{Serialize, Deserialize};
use std::fmt;

/// An instance of a library contained within `Enrichr`
///
/// Data is stored as a json at <https://maayanlab.cloud/Enrichr/datasetStatistics>
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct Library {
    pub gene_coverage: usize,
    pub genes_per_term: f64,
    pub library_name: String,
    pub link: String,
    pub num_terms: usize,
    pub appyter: String,
    pub category_id: usize
}
impl fmt::Display for Library {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).expect("cannot serialize"))
    }
}


/// An instance of category contained within `Enrichr`
///
/// Data is stored as a json at <https://maayanlab.cloud/Enrichr/datasetStatistics>
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct Category {
    pub category_id: usize,
    pub name: String
}
impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).expect("cannot serialize"))
    }
}

/// All libraries contained within `Enrichr`.
///
/// The `statistics` attribute is a container of all known [Library].
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct ResponseLibraries {
    pub statistics: Vec<Library>,
    pub categories: Vec<Category>
}
impl ResponseLibraries {
    pub fn iter(&self) -> impl Iterator<Item = &Library> {
        self.statistics.iter()
    }
}
impl fmt::Display for ResponseLibraries {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).expect("cannot serialize"))
    }
}

/// Performs a `GET` call to retrieve the known libraries of `Enrichr`
pub fn get_libraries() -> Result<ResponseLibraries> {
    let url = "https://maayanlab.cloud/Enrichr/datasetStatistics";
    reqwest::blocking::get(url)?
        .json::<ResponseLibraries>()
}
