use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityResponse {
    pub activities: Vec<Activity>,
    #[serde(skip_serializing)]
    pub page_meta: PageMeta,
}
impl fmt::Display for ActivityResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).expect("cannot serialize")
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Activity {
    pub activity_id: usize,
    pub activity_comment: Option<String>,
    pub assay_chembl_id: String,
    pub assay_description: Option<String>,
    pub canonical_smiles: String,
    pub molecule_chembl_id: String,
    pub molecule_pref_name: Option<String>,
    pub target_chembl_id: String,
    pub assay_type: String,
}
impl fmt::Display for Activity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).expect("cannot serialize")
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PageMeta {
    limit: usize,
    next: Option<String>,
    offset: usize,
    previous: Option<String>,
    total_count: usize,
}
impl fmt::Display for PageMeta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).expect("cannot serialize")
        )
    }
}
