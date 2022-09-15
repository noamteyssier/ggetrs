use serde::{Deserialize, Serialize};
use std::fmt;

/// Struct to handle the response of gene list
///
/// Details can be found at <https://maayanlab.cloud/Enrichr/help#api&q=2>
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseViewList {
    pub genes: Vec<String>,
    pub description: String,
}
impl fmt::Display for ResponseViewList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).expect("cannot serialize")
        )
    }
}
