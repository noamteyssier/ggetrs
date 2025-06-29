use pyo3::{
    Bound, PyResult, Python, pyfunction,
    types::{PyDict, PyModule, PyModuleMethods},
    wrap_pyfunction,
};

use super::{Species, correlation, tissue};

/// Wraps the `ARCHS4` correlation analysis
#[pyfunction(name = "correlate")]
pub fn python_archs4_correlate<'py>(
    py: Python<'py>,
    gene_name: &str,
    count: usize,
) -> PyResult<Bound<'py, PyDict>> {
    let results = correlation(gene_name, count).expect("Unable to query `matrixapi/coltop`");
    results.as_pydict(py)
}

/// Wraps the `ARCHS4` tissue expression analysis
#[pyfunction(name = "tissue")]
pub fn python_archs4_tissue<'py>(
    py: Python<'py>,
    gene_name: &str,
    species: &str,
) -> PyResult<Bound<'py, PyDict>> {
    let species_enum = Species::from_str(species).unwrap_or_default();
    let results = tissue(gene_name, &species_enum).expect("Unable to query `archs4/tissue`");
    results.as_pydict(py)
}

/// Wraps `ARCHS4` specific functions into a python submodule
pub fn python_archs4(py: Python<'_>, module: &Bound<'_, PyModule>) -> PyResult<()> {
    let submodule = PyModule::new(py, "archs4")?;
    submodule.add_function(wrap_pyfunction!(python_archs4_tissue, module)?)?;
    submodule.add_function(wrap_pyfunction!(python_archs4_correlate, module)?)?;
    module.add_submodule(&submodule)?;
    Ok(())
}
