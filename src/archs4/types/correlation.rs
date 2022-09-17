use pyo3::{types::PyDict, PyResult, Python};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Deserialization of the ARCHS4 result json
#[derive(Serialize, Deserialize, Debug)]
pub struct ResultCorrelation {
    pub index: Vec<usize>,
    pub column: String,
    pub rowids: Vec<String>,
    pub values: Vec<f64>,
}
impl fmt::Display for ResultCorrelation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).expect("cannot serialize")
        )
    }
}
/// Implementation to convert to human-friendly format
#[allow(clippy::from_over_into)]
impl Into<Correlations> for ResultCorrelation {
    fn into(self) -> Correlations {
        let correlations = self
            .rowids
            .iter()
            .zip(self.values.iter())
            .map(|(g, c)| Correlation::new(g, *c))
            .collect();
        Correlations { correlations }
    }
}

/// Human friendly serialization of the ARCHS4 result json
#[derive(Serialize, Deserialize, Debug)]
pub struct Correlations {
    pub correlations: Vec<Correlation>,
}
impl fmt::Display for Correlations {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).expect("cannot serialize")
        )
    }
}
impl Correlations {
    pub fn as_pydict<'py>(&self, py: Python<'py>) -> PyResult<&'py PyDict> {
        let dict = PyDict::new(py);
        dict.set_item(
            "correlations",
            self.correlations
                .iter()
                .map(|x| x.as_pydict(py).expect("could not create pydict"))
                .collect::<Vec<&PyDict>>(),
        )?;
        Ok(dict)
    }
}

/// An instance of a result (i.e. a single genes pearson correlation with the query gene)
#[derive(Serialize, Deserialize, Debug)]
pub struct Correlation {
    pub gene_symbol: String,
    pub pearson_correlation: f64,
}
impl Correlation {
    #[must_use]
    pub fn new(gene_symbol: &str, pearson_correlation: f64) -> Self {
        Self {
            gene_symbol: gene_symbol.to_string(),
            pearson_correlation,
        }
    }
}
impl fmt::Display for Correlation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).expect("cannot serialize")
        )
    }
}
impl Correlation {
    pub fn as_pydict<'py>(&self, py: Python<'py>) -> PyResult<&'py PyDict> {
        let dict = PyDict::new(py);
        dict.set_item("gene_symbol", &self.gene_symbol)?;
        dict.set_item("pearson_correlation", self.pearson_correlation)?;
        Ok(dict)
    }
}
