use super::{add_list, enrich};
use pyo3::{pyfunction, types::PyDict, PyResult, Python};

#[pyfunction(name = "enrichr")]
#[allow(clippy::needless_pass_by_value)]
pub fn python_enrichr<'py>(
    py: Python<'py>,
    library_name: &str,
    gene_list: Vec<String>,
) -> PyResult<&'py PyDict> {
    let add_list = add_list(&gene_list).expect("Unable to query `addList`");
    let results = enrich(add_list.user_list_id, library_name).expect("Unable to perform `enrich`");
    results.as_pydict(py)
}
