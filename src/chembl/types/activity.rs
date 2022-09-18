use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityResponse {
    pub activities: Vec<Activity>,
    page_meta: PageMeta
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
    activity_id: usize,
    assay_chembl_id: String,
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
    total_count: usize
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
