use super::info;
use anyhow::{bail, Result};
use pyo3::{
    pyfunction,
    types::{IntoPyDict, PyDict},
    Bound, Python,
};

#[pyfunction(name = "info")]
#[pyo3(signature = (search_terms, species = None, taxon_id = None))]
#[allow(clippy::needless_pass_by_value)]
pub fn python_info(
    py: Python<'_>,
    search_terms: Vec<String>,
    species: Option<String>,
    taxon_id: Option<usize>,
) -> Result<Bound<'_, PyDict>> {
    if search_terms.is_empty() {
        bail!("Must pass in more than one search term!");
    } else if search_terms[0].len() == 1 {
        bail!("Must pass in search terms as a list!");
    }
    let species = species.unwrap_or("homo_sapiens".to_string());
    let taxon_id = taxon_id.unwrap_or(9606);
    let results = info(&search_terms, &species, taxon_id)?;
    Ok(results.into_py_dict(py)?)
}
