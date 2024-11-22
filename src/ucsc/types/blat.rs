use pyo3::{
    types::{IntoPyDict, PyDict, PyDictMethods, PyList},
    Bound, PyResult, Python,
};
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
    #[must_use]
    pub fn from_value(value: &Value) -> Self {
        let results = value["blat"]
            .as_array()
            .map(|array| array.iter().map(Blat::from_value).collect::<Vec<Blat>>())
            .unwrap_or_default();
        Self(results)
    }
    pub fn as_pylist<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
        let vec_dict =
            self.0
                .iter()
                .cloned()
                .try_fold(Vec::new(), |mut acc, x| -> PyResult<Vec<_>> {
                    acc.push(x.into_py_dict(py)?);
                    Ok(acc)
                })?;
        PyList::new(py, vec_dict)
    }
}

/// Expected results from a BLAT query
#[derive(Serialize, Deserialize, Clone, Debug)]
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
impl<'py> IntoPyDict<'py> for Blat {
    fn into_py_dict(self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item("matches", self.matches)?;
        dict.set_item("mismatches", self.mismatches)?;
        dict.set_item("repmatches", self.repmatches)?;
        dict.set_item("n_count", self.n_count)?;
        dict.set_item("q_num_insert", self.q_num_insert)?;
        dict.set_item("q_base_insert", self.q_base_insert)?;
        dict.set_item("t_num_insert", self.t_num_insert)?;
        dict.set_item("t_base_insert", self.t_base_insert)?;
        dict.set_item("strand", &self.strand)?;
        dict.set_item("q_name", &self.q_name)?;
        dict.set_item("q_size", self.q_size)?;
        dict.set_item("q_start", self.q_start)?;
        dict.set_item("q_end", self.q_end)?;
        dict.set_item("t_name", &self.t_name)?;
        dict.set_item("t_size", self.t_size)?;
        dict.set_item("t_start", self.t_start)?;
        dict.set_item("t_end", self.t_end)?;
        dict.set_item("block_count", self.block_count)?;
        dict.set_item("block_sizes", &self.block_sizes)?;
        dict.set_item("q_starts", &self.q_starts)?;
        dict.set_item("q_starts", &self.t_starts)?;
        Ok(dict)
    }
}
impl Blat {
    #[must_use]
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
