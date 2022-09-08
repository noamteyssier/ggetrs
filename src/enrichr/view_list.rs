use reqwest::Error;
use serde::Deserialize;

/// Struct to handle the response of gene list
/// 
/// Details can be found at <https://maayanlab.cloud/Enrichr/help#api&q=2>
#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct ResponseViewList {
    pub genes: Vec<String>,
    pub description: String
}

/// Performs a `GET` call to retrieve the genes within a `userListId`.
pub async fn view_list(user_list_id: usize) -> Result<ResponseViewList, Error> {
    let url = format!("https://maayanlab.cloud/Enrichr/view?userListId={}", user_list_id);
    reqwest::get(&url)
        .await?
        .json::<ResponseViewList>()
        .await
}
