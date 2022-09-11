use pyo3::{
    pyfunction, Python, PyResult,
    types::PyDict
};
use super::search;


#[pyfunction(name="search")]
pub fn python_ensembl_search<'py>(py: Python<'py>, search_terms: Vec<String>, species: &str, db_type: &str, release: usize, assembly: &str) -> PyResult<&'py PyDict> {
    let db_name = format!("{}_{}_{}_{}", species, db_type, release, assembly);
    let results = search(&db_name, &search_terms).expect("Unable to query ensembl search");
    results.as_pydict(py)
}
