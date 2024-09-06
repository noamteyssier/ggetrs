use super::sequence;
use crate::uniprot::query;
use anyhow::{bail, Result};
use pyo3::{pyfunction, types::PyList, Bound, Python};

#[pyfunction(name = "seq")]
#[pyo3(signature = (search_terms, translate = None, species = None))]
#[allow(clippy::needless_pass_by_value)]
pub fn python_seq(
    py: Python<'_>,
    search_terms: Vec<String>,
    translate: Option<bool>,
    species: Option<String>,
) -> Result<Bound<'_, PyList>> {
    if search_terms.is_empty() {
        bail!("Must pass in more than one search term!");
    } else if search_terms[0].len() == 1 {
        bail!("Must pass in search terms as a list!");
    }
    let translate = translate.unwrap_or(false);
    let species = Some(species.unwrap_or("homo_sapiens".to_string()));

    let records = if translate {
        let results = query(&search_terms, false, &None)?;
        results.fasta_records()
    } else {
        let results = sequence(&search_terms, &species)?;
        results.fasta_records()
    }
    .as_pylist(py)?;
    Ok(records)
}
