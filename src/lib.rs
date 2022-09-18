use pyo3::{pymodule, types::PyModule, wrap_pyfunction, PyResult, Python};

/// `Enrichr` submodule
pub mod enrichr;

/// `ARCHS4` submodule
pub mod archs4;

/// `ENSEMBL` submodule
pub mod ensembl;

/// `Uniprot` submodule
pub mod uniprot;

/// `NCBI` submodule
pub mod ncbi;

/// `RCSB PDB` submodule
pub mod pdb;

/// Module for constants shared throughout the crate
pub mod constants;

/// Module for utilities shared throughout the crate
pub mod utils;

/// Module for Info command
pub mod info;

/// Module for handling CLI
pub mod cli;

/// Useful generic type for Request Errors
pub type RequestError = Box<dyn std::error::Error + Send + Sync>;

#[pymodule]
fn ggetrs(py: Python<'_>, module: &PyModule) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(enrichr::python_enrichr, module)?)?;
    archs4::python_archs4(py, module)?;
    ensembl::python_ensembl(py, module)?;
    module.add_function(wrap_pyfunction!(ensembl::python_ensembl_search, module)?)?;
    Ok(())
}
