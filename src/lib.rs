use pyo3::{pymodule, types::PyModule, wrap_pyfunction, PyResult, Python};

/// `Enrichr` submodule
pub mod enrichr;

/// `ARCHS4` submodule
pub mod archs4;

/// `BLAST` submodule
pub mod blast;

/// Command Line Interface
pub mod cli;

/// `Chembl` submodule
pub mod chembl;

/// `ENSEMBL` submodule
pub mod ensembl;

/// `Uniprot` submodule
pub mod uniprot;

/// `NCBI` submodule
pub mod ncbi;

/// `RCSB PDB` submodule
pub mod pdb;

/// `UCSC` Genome Browser submodule
pub mod ucsc;

/// Module for constants shared throughout the crate
pub mod constants;

/// Module for utilities shared throughout the crate
pub mod utils;

/// Module for Info command
pub mod info;

// Module for Sequence query
pub mod seq;

/// Useful generic type for Request Errors
pub type RequestError = Box<dyn std::error::Error + Send + Sync>;

#[pymodule]
fn ggetrs(py: Python<'_>, module: &PyModule) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(enrichr::python_enrichr, module)?)?;
    archs4::python_archs4(py, module)?;
    ensembl::python_ensembl(py, module)?;
    ucsc::python_ucsc(py, module)?;
    module.add_function(wrap_pyfunction!(ensembl::python_ensembl_search, module)?)?;
    module.add_function(wrap_pyfunction!(seq::python_seq, module)?)?;
    module.add_function(wrap_pyfunction!(info::python_info, module)?)?;
    module.add_function(wrap_pyfunction!(blast::python_blast, module)?)?;
    Ok(())
}
