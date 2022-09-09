use pyo3::{pymodule, Python, types::PyModule, PyResult, wrap_pyfunction};

/// `Enrichr` submodule
pub mod enrichr;

/// Useful generic type for Request Errors
pub type RequestError = Box<dyn std::error::Error + Send + Sync>;

#[pymodule]
fn ggetrs(_py: Python<'_>, module: &PyModule) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(enrichr::python_enrichr, module)?)?;
    Ok(())
}
