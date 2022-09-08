use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct Category {
    pub category_id: usize,
    pub name: String
}

#[derive(Deserialize, Debug)]
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

pub async fn get_libraries() -> Result<ResponseLibraries, Error> {
    let url = "https://maayanlab.cloud/Enrichr/datasetStatistics";
    reqwest::get(url)
        .await?
        .json::<ResponseLibraries>()
        .await
}
