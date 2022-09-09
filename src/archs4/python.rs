use pyo3::{PyResult, types::PyDict, pyfunction, Python};
use super::correlation;

#[pyfunction(name="archs4")]
pub fn python_archs4<'py>(py: Python<'py>, gene_name: &str, count: usize) -> PyResult<&'py PyDict> {
    let results = correlation(gene_name, count).expect("Unable to query `matrixapi/coltop`");
    results.as_pydict(py)
}

