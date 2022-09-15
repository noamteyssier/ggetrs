use pyo3::{pyclass, types::PyDict, PyResult, Python};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// A struct to hold the results of an enrichment test.
///
/// The keys of this `HashMap` will be the background library
/// tested against and the values will each be an instance of [ResultEnrichr]
#[derive(Serialize, Deserialize, Debug)]
#[pyclass]
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
impl ResponseEnrich {
    pub fn as_pydict<'py>(&self, py: Python<'py>) -> PyResult<&'py PyDict> {
        let dict = PyDict::new(py);
        for (key, results) in (&self.0).iter() {
            let all_results: Vec<&'py PyDict> = results
                .iter()
                .map(|x| x.as_pydict(py).expect("could not create dictionary"))
                .collect();
            dict.set_item(key, all_results)?;
        }
        Ok(dict)
    }
}

/// A singular enrichment result.
///
/// Names were taken from <https://maayanlab.cloud/Enrichr/help#api&q=3>
#[derive(Serialize, Deserialize, Debug)]
#[pyclass(dict)]
pub struct ResultEnrichr {
    #[pyo3(get, set)]
    pub rank: usize,
    #[pyo3(get, set)]
    pub term_name: String,
    #[pyo3(get, set)]
    pub pvalue: f64,
    #[pyo3(get, set)]
    pub zscore: f64,
    #[pyo3(get, set)]
    pub combined_score: f64,
    #[pyo3(get, set)]
    pub overlapping_genes: Vec<String>,
    #[pyo3(get, set)]
    pub adj_pvalue: f64,
    #[pyo3(get, set)]
    pub old_pvalue: f64,
    #[pyo3(get, set)]
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
impl ResultEnrichr {
    pub fn as_pydict<'py>(&self, py: Python<'py>) -> PyResult<&'py PyDict> {
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
