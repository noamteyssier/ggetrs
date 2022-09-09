use pyo3::{pymodule, Python, types::PyModule, PyResult, wrap_pyfunction};

/// `Enrichr` submodule
pub mod enrichr;

/// `ARCHS4` submodule
pub mod archs4;

/// Useful generic type for Request Errors
pub type RequestError = Box<dyn std::error::Error + Send + Sync>;

#[pymodule]
fn ggetrs(_py: Python<'_>, module: &PyModule) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(enrichr::python_enrichr, module)?)?;
    module.add_function(wrap_pyfunction!(archs4::python_archs4, module)?)?;
    Ok(())
}
