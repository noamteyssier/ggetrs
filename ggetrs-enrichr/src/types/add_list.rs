use serde::{Deserialize, Serialize};
use std::fmt;

/// A struct to handle the results of the `Enrichr` function call: `addList`
///
/// details at: <https://maayanlab.cloud/Enrichr/help#api&q=1>
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseAddList {
    pub user_list_id: usize,
    pub short_id: String,
}
impl fmt::Display for ResponseAddList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).expect("cannot serialize")
        )
    }
}
