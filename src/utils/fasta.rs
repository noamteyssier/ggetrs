use pyo3::{
    prelude::*,
    types::{IntoPyDict, PyDict, PyList},
};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FastaRecords(pub Vec<FastaRecord>);
impl FastaRecords {
    pub fn as_pylist<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
        let vec_dict =
            self.0
                .iter()
                .cloned()
                .try_fold(Vec::default(), |mut acc, x| -> PyResult<Vec<_>> {
                    acc.push(x.into_py_dict(py)?);
                    Ok(acc)
                })?;
        PyList::new(py, vec_dict)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[pyclass]
pub struct FastaRecord {
    header: String,
    sequence: String,
}
impl Display for FastaRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, ">{}\n{}\n", self.header, self.sequence)
    }
}
impl<'py> IntoPyDict<'py> for FastaRecord {
    fn into_py_dict(self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let map = PyDict::new(py);
        map.set_item("header", self.header)?;
        map.set_item("sequence", self.sequence)?;
        Ok(map)
    }
}
impl FastaRecord {
    #[must_use]
    pub fn new(header: &str, sequence: &str) -> Self {
        Self {
            header: header.to_string(),
            sequence: sequence.to_string(),
        }
    }
}
