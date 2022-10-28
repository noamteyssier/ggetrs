use pyo3::{Python, types::PyList, PyResult, pyfunction};
use crate::uniprot::query;
use super::sequence;

#[pyfunction(name = "seq")]
#[pyo3(text_signature = "(search_terms, transcribe = False, db_name = 'homo_sapiens')")]
pub fn python_seq<'py>(
    py: Python<'py>,
    search_terms: Vec<String>,
    transcribe: Option<bool>,
    species: Option<String>,
) -> PyResult<&'py PyList> {

    let transcribe = transcribe.unwrap_or(false);
    let species = Some(species.unwrap_or("homo_sapiens".to_string()));

    let records = if transcribe {
        let results = query(&search_terms, &None)?;
        results.fasta_records()
    } else {
        let results = sequence(&search_terms, &species)?;
        results.fasta_records()
    }.as_pylist(py)?;
    Ok(records)
}
