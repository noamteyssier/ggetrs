use pyo3::{pymodule, Python, types::PyModule, PyResult, wrap_pyfunction};

/// `Enrichr` submodule
pub mod enrichr;

/// `ARCHS4` submodule
pub mod archs4;

/// `ENSEMBL` submodule
pub mod ensembl;

/// Useful generic type for Request Errors
pub type RequestError = Box<dyn std::error::Error + Send + Sync>;

#[pymodule]
fn ggetrs(py: Python<'_>, module: &PyModule) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(enrichr::python_enrichr, module)?)?;
    archs4::python_archs4(py, module)?;
    module.add_function(wrap_pyfunction!(ensembl::python_ensembl_search, module)?)?;
    Ok(())
}
