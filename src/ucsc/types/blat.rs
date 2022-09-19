use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

/// A container of BLAT results
#[derive(Serialize, Deserialize, Debug)]
pub struct BlatResults(pub Vec<Blat>);
impl fmt::Display for BlatResults {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).expect("cannot serialize")
        )
    }
}
impl BlatResults {
    pub fn from_value(value: &Value) -> Self {
        let results = value["blat"]
            .as_array()
            .map(|array| {
                array
                    .iter()
                    .map(|x| Blat::from_value(x))
                    .collect::<Vec<Blat>>()
            })
            .unwrap_or_default();
        Self(results)
    }
}

/// Expected results from a BLAT query
#[derive(Serialize, Deserialize, Debug)]
pub struct Blat {
    pub matches: usize,
    pub mismatches: usize,
    pub repmatches: usize,
    pub n_count: usize,
    pub q_num_insert: usize,
    pub q_base_insert: usize,
    pub t_num_insert: usize,
    pub t_base_insert: usize,
    pub strand: String,
    pub q_name: String,
    pub q_size: usize,
    pub q_start: usize,
    pub q_end: usize,
    pub t_name: String,
    pub t_size: usize,
    pub t_start: usize,
    pub t_end: usize,
    pub block_count: usize,
    pub block_sizes: String,
    pub q_starts: String,
    pub t_starts: String,
}
impl fmt::Display for Blat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).expect("cannot serialize")
        )
    }
}
impl Blat {
    pub fn from_value(value: &Value) -> Self {
        let arr = value.as_array().expect("Empty Array Found");
        let matches = arr[0].as_u64().unwrap_or_default() as usize;
        let mismatches = arr[1].as_u64().unwrap_or_default() as usize;
        let repmatches = arr[2].as_u64().unwrap_or_default() as usize;
        let n_count = arr[3].as_u64().unwrap_or_default() as usize;
        let q_num_insert = arr[4].as_u64().unwrap_or_default() as usize;
        let q_base_insert = arr[5].as_u64().unwrap_or_default() as usize;
        let t_num_insert = arr[6].as_u64().unwrap_or_default() as usize;
        let t_base_insert = arr[7].as_u64().unwrap_or_default() as usize;
        let strand = arr[8].as_str().unwrap_or_default().to_string();
        let q_name = arr[9].as_str().unwrap_or_default().to_string();
        let q_size = arr[10].as_u64().unwrap_or_default() as usize;
        let q_start = arr[11].as_u64().unwrap_or_default() as usize;
        let q_end = arr[12].as_u64().unwrap_or_default() as usize;
        let t_name = arr[13].as_str().unwrap_or_default().to_string();
        let t_size = arr[14].as_u64().unwrap_or_default() as usize;
        let t_start = arr[15].as_u64().unwrap_or_default() as usize;
        let t_end = arr[16].as_u64().unwrap_or_default() as usize;
        let block_count = arr[16].as_u64().unwrap_or_default() as usize;
        let block_sizes = arr[17].as_str().unwrap_or_default().to_string();
        let q_starts = arr[18].as_str().unwrap_or_default().to_string();
        let t_starts = arr[19].as_str().unwrap_or_default().to_string();
        Self {
            matches,
            mismatches,
            repmatches,
            n_count,
            q_num_insert,
            q_base_insert,
            t_num_insert,
            t_base_insert,
            strand,
            q_name,
            q_size,
            q_start,
            q_end,
            t_name,
            t_size,
            t_start,
            t_end,
            block_count,
            block_sizes,
            q_starts,
            t_starts,
        }
    }
}
