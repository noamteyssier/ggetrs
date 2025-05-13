use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};

#[cfg(feature = "python")]
use pyo3::{
    Bound, PyResult, Python, pyclass,
    types::{PyDict, PyDictMethods},
};

/// A struct to hold the results of an enrichment test.
///
/// The keys of this `HashMap` will be the background library
/// tested against and the values will each be an instance of [ResultEnrichr]
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "python", pyclass)]
pub struct ResponseEnrich(pub HashMap<String, Vec<ResultEnrichr>>);
impl fmt::Display for ResponseEnrich {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).expect("cannot serialize")
        )
    }
}
#[cfg(feature = "python")]
impl ResponseEnrich {
    pub fn as_pydict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        for (key, results) in &self.0 {
            let all_results =
                results
                    .iter()
                    .try_fold(Vec::new(), |mut acc, x| -> PyResult<Vec<_>> {
                        acc.push(x.as_pydict(py)?);
                        Ok(acc)
                    })?;
            dict.set_item(key, all_results)?;
        }
        Ok(dict)
    }
}

/// A singular enrichment result.
///
/// Names were taken from <https://maayanlab.cloud/Enrichr/help#api&q=3>
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "python", pyclass(dict, get_all, set_all))]
#[allow(clippy::unsafe_derive_deserialize)]
pub struct ResultEnrichr {
    pub rank: usize,
    pub term_name: String,
    pub pvalue: f64,
    pub zscore: f64,
    pub combined_score: f64,
    pub overlapping_genes: Vec<String>,
    pub adj_pvalue: f64,
    pub old_pvalue: f64,
    pub old_adj_pvalue: f64,
}
impl fmt::Display for ResultEnrichr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).expect("cannot serialize")
        )
    }
}
#[cfg(feature = "python")]
impl ResultEnrichr {
    pub fn as_pydict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item("rank", self.rank)?;
        dict.set_item("term_name", &self.term_name)?;
        dict.set_item("pvalue", self.pvalue)?;
        dict.set_item("zscore", self.zscore)?;
        dict.set_item("combined_score", self.combined_score)?;
        dict.set_item("overlapping_genes", &self.overlapping_genes)?;
        dict.set_item("adj_pvalue", self.adj_pvalue)?;
        dict.set_item("old_pvalue", self.old_pvalue)?;
        dict.set_item("old_adj_pvalue", self.old_adj_pvalue)?;
        Ok(dict)
    }
}
