use clap::ArgEnum;
use pyo3::{
    wrap_pyfunction, pyfunction, Python, PyResult, 
    types::{PyDict, PyModule}};
use super::{correlation, Species, tissue};

/// Wraps the `ARCHS4` correlation analysis
#[pyfunction(name="correlate")]
pub fn python_archs4_correlate<'py>(py: Python<'py>, gene_name: &str, count: usize) -> PyResult<&'py PyDict> {
    let results = correlation(gene_name, count).expect("Unable to query `matrixapi/coltop`");
    results.as_pydict(py)
}

/// Wraps the `ARCHS4` tissue expression analysis
#[pyfunction(name="tissue")]
pub fn python_archs4_tissue<'py>(py: Python<'py>, gene_name: &str, species: &str) -> PyResult<&'py PyDict> {
    let species = match species {
        "human" => Species::Human,
        "mouse" => Species::Mouse,
        _ => return Err(PyAssertionError::new_err("unexpected species name"))
    };
    let results = tissue(gene_name, &species).expect("Unable to query `archs4/tissue`");
    results.as_pydict(py)
}

/// Wraps `ARCHS4` specific functions into a python submodule
pub fn python_archs4(py: Python<'_>, module: &PyModule) -> PyResult<()> {
    let submodule = PyModule::new(py, "archs4")?;
    submodule.add_function(wrap_pyfunction!(python_archs4_tissue, module)?)?;
    submodule.add_function(wrap_pyfunction!(python_archs4_correlate, module)?)?;
    module.add_submodule(submodule)?;
    Ok(())
}

