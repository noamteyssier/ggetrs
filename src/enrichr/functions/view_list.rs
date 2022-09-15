use reqwest::Result;
use crate::enrichr::types::ResponseViewList;

/// Performs a `GET` call to retrieve the genes within a `userListId`.
pub fn view_list(user_list_id: usize) -> Result<ResponseViewList> {
    let url = format!(
        "https://maayanlab.cloud/Enrichr/view?userListId={}",
        user_list_id
    );
    reqwest::blocking::get(&url)?.json::<ResponseViewList>()
}
