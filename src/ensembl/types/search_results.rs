use anyhow::Result;
use mysql::Row;
use pyo3::{types::PyDict, PyResult, Python};
use serde::{Deserialize, Serialize};
use std::fmt;

/// A unit struct container of [`SearchResult`]
#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResults(pub Vec<SearchResult>);
impl fmt::Display for SearchResults {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).expect("cannot serialize")
        )
    }
}
impl SearchResults {
    pub fn as_pydict<'py>(&self, py: Python<'py>) -> Result<&'py PyDict> {
        let dict = PyDict::new(py);
        dict.set_item(
            "results",
            self.0
                .iter()
                .map(|x| x.as_pydict(py).expect("could not create pydict"))
                .collect::<Vec<&PyDict>>(),
        )?;
        Ok(dict)
    }
}

/// An individual search result generated from the `Ensembl` SQL query
#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResult {
    pub stable_id: String,
    pub display_label: String,
    pub ensembl_description: String,
    pub xref_description: String,
    pub biotype: String,
}
impl fmt::Display for SearchResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).expect("cannot serialize")
        )
    }
}
impl SearchResult {
    /// Will parse the provided index into a string or return an empty string if it is null
    fn parse_index(row: &Row, index: usize) -> String {
        if let Some(Ok(value)) = row.get_opt::<String, usize>(index) {
            value
        } else {
            String::new()
        }
    }

    /// Generates from a [`Row`]
    pub fn from_row(row: &Row) -> anyhow::Result<Self> {
        if row.is_empty() {
            anyhow::bail!("empty row")
        }
        let stable_id = Self::parse_index(row, 0);
        let display_label = Self::parse_index(row, 1);
        let ensembl_description = Self::parse_index(row, 2);
        let xref_description = Self::parse_index(row, 3);
        let biotype = Self::parse_index(row, 4);

        Ok(Self {
            stable_id,
            display_label,
            ensembl_description,
            xref_description,
            biotype,
        })
    }

    pub fn as_pydict<'py>(&self, py: Python<'py>) -> PyResult<&'py PyDict> {
        let dict = PyDict::new(py);
        dict.set_item("stable_id", &self.stable_id)?;
        dict.set_item("display_label", &self.display_label)?;
        dict.set_item("ensembl_description", &self.ensembl_description)?;
        dict.set_item("xref_description", &self.xref_description)?;
        dict.set_item("biotype", &self.biotype)?;
        Ok(dict)
    }
}
