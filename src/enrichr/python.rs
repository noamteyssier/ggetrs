use std::collections::HashMap;

use pyo3::{pyfunction, PyResult};
use super::{add_list, enrich, ResultEnrichr};

#[pyfunction]
#[pyo3(name="enrichr")]
pub fn python_enrichr(library_name: &str, gene_list: Vec<String>) -> PyResult<HashMap<String, Vec<ResultEnrichr>>> {
    let add_list = add_list(&gene_list).expect("Unable to gene list");
    let results = enrich(add_list.user_list_id, library_name).expect("Unable to query gene list");
    Ok(results.0)
}
