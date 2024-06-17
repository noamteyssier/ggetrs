use super::{add_list, enrich, functions::add_background};
use pyo3::{pyfunction, types::PyDict, Bound, PyResult, Python};

#[pyfunction(name = "enrichr")]
#[allow(clippy::needless_pass_by_value)]
/// Perform an gene set enrichment analysis using Enrichr.
///
/// # Arguments
/// * `library_name` - any database listed at: https://maayanlab.cloud/Enrichr/#libraries
/// some shorthands include: pathway, transcription, ontology, diseases_drugs, celltypes,
/// and kinase_interactions.
/// * `gene_list` - list of gene symbols to perform enrichment analysis on.
///
/// # Returns
/// A dictionary of results. The keys of this `HashMap` will be the background library
pub fn python_enrichr(
    py: Python<'_>,
    library_name: String,
    gene_list: Vec<String>,
) -> PyResult<Bound<'_, PyDict>> {
    let add_list = add_list(&gene_list, false).expect("Unable to query `addList`");
    let results =
        enrich(add_list.user_list_id, &library_name, None).expect("Unable to perform `enrich`");
    results.as_pydict(py)
}

#[pyfunction(name = "enrichr_background")]
#[allow(clippy::needless_pass_by_value)]
/// Perform an gene set enrichment analysis using Enrichr using a background gene list.
///
/// *Note*: This function will not work if the gene set is not a subset of the background gene list.
///
/// # Arguments
/// * `library_name` - any database listed at: https://maayanlab.cloud/Enrichr/#libraries
/// some shorthands include: pathway, transcription, ontology, diseases_drugs, celltypes,
/// and kinase_interactions.
/// * `gene_list` - list of gene symbols to perform enrichment analysis on.
/// * `background_list` - list of gene symbols to use as the background for the enrichment analysis.
///
/// # Returns
/// A dictionary of results. The keys of this `HashMap` will be the background library
pub fn python_enrichr_background(
    py: Python<'_>,
    library_name: String,
    gene_list: Vec<String>,
    background_list: Vec<String>,
) -> PyResult<Bound<'_, PyDict>> {
    let add_list = add_list(&gene_list, true).expect("Unable to query `addList`");
    let background_list =
        add_background(&background_list).expect("Unable to query `addBackground`");
    let results = enrich(
        add_list.user_list_id,
        &library_name,
        Some(&background_list.backgroundid),
    )
    .expect("Unable to perform `enrich`");
    results.as_pydict(py)
}
