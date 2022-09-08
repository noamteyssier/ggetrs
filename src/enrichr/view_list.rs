use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct ResponseViewList {
    pub genes: Vec<String>,
    pub description: String
}

pub async fn view_list(user_list_id: usize) -> Result<ResponseViewList, Error> {
    let url = format!("https://maayanlab.cloud/Enrichr/view?userListId={}", user_list_id);
    reqwest::get(&url)
        .await?
        .json::<ResponseViewList>()
        .await
}
