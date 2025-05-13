use pyo3::prelude::*;

#[pymodule]
#[pyo3(name = "ggetrs")]
fn ggetrs(py: Python<'_>, module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(ggetrs_enrichr::python_enrichr, module)?)?;
    module.add_function(wrap_pyfunction!(
        ggetrs_enrichr::python_enrichr_background,
        module
    )?)?;
    ggetrs_archs4::python_archs4(py, module)?;
    ggetrs_ensembl::python_ensembl(py, module)?;
    ggetrs_ucsc::python_ucsc(py, module)?;
    module.add_function(wrap_pyfunction!(
        ggetrs_ensembl::python_ensembl_search,
        module
    )?)?;
    module.add_function(wrap_pyfunction!(ggetrs_seq::python_seq, module)?)?;
    module.add_function(wrap_pyfunction!(ggetrs_info::python_info, module)?)?;
    module.add_function(wrap_pyfunction!(ggetrs_blast::python_blast, module)?)?;
    Ok(())
}
