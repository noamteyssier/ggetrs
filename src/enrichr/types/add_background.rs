use serde::{Deserialize, Serialize};
use std::fmt;

/// A struct to handle the results of the `Enrichr` function call: `addbackground`
///
/// details at: <https://maayanlab.cloud/Enrichr/help#api&q=4>
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseAddBackground {
    pub backgroundid: String,
}
impl fmt::Display for ResponseAddBackground {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).expect("cannot serialize")
        )
    }
}
