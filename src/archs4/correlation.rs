use std::collections::HashMap;
use pyo3::{Python, PyResult, types::PyDict};
use reqwest::{blocking::Client, Result};
use serde::{Serialize, Deserialize};
use std::fmt;

/// Deserialization of the ARCHS4 result json
#[derive(Serialize, Deserialize, Debug)]
pub struct ResultCorrelation {
    pub index: Vec<usize>,
    pub column: String,
    pub rowids: Vec<String>,
    pub values: Vec<f64>
}
impl fmt::Display for ResultCorrelation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).expect("cannot serialize"))
    }
}
/// Implementation to convert to human-friendly format
#[allow(clippy::from_over_into)]
impl Into<Correlations> for ResultCorrelation {
    fn into(self) -> Correlations {
        let correlations = self.rowids.iter()
            .zip(self.values.iter())
            .map(|(g, c)| Correlation::new(g, *c))
            .collect();
        Correlations { correlations }
    }

}

/// Human friendly serialization of the ARCHS4 result json
#[derive(Serialize, Deserialize, Debug)]
pub struct Correlations {
    pub correlations: Vec<Correlation>
}
impl fmt::Display for Correlations {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).expect("cannot serialize"))
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
                .collect::<Vec<&PyDict>>()
            )?;
        Ok(dict)
    }
}


/// An instance of a result (i.e. a single genes pearson correlation with the query gene)
#[derive(Serialize, Deserialize, Debug)]
pub struct Correlation {
    pub gene_symbol: String,
    pub pearson_correlation: f64
}
impl Correlation {
    pub fn new(gene_symbol: &str, pearson_correlation: f64) -> Self {
        Self {
            gene_symbol: gene_symbol.to_string(),
            pearson_correlation
        }
    }
}
impl fmt::Display for Correlation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).expect("cannot serialize"))
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

/// Queries the most correlated genes for a provided gene.
///
/// Uses the ARCHS4 correlation API
pub fn correlation(gene_name: &str, count: usize) -> Result<Correlations> {
    let client = Client::new();
    let url = "https://maayanlab.cloud/matrixapi/coltop";
    let map = build_query(gene_name, count);
    let correlations = client.post(url)
        .json(&map)
        .send()?
        .json::<ResultCorrelation>()?;
    Ok(correlations.into())
}

/// Builds the `HashMap` to be converted to a JSON
fn build_query(gene_name: &str, count: usize) -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert(String::from("id"), gene_name.to_string());
    map.insert(String::from("count"), format!("{}", count));
    map
}

#[cfg(test)]
mod testing {
    use super::correlation;

    #[test]
    fn test_known() {
        let symbol = "AP2S1";
        let count = 5;
        let results = correlation(symbol, count).unwrap();
        assert_eq!(results.correlations.len(), count);
        assert_eq!(results.correlations[0].gene_symbol, "AP2S1");
        assert_eq!(results.correlations[1].gene_symbol, "ASNA1");
        assert_eq!(results.correlations[2].gene_symbol, "MRPL28");
        assert_eq!(results.correlations[3].gene_symbol, "SSNA1");
        assert_eq!(results.correlations[4].gene_symbol, "COX8A");
    }
}
