use super::types::InfoContainer;
use crate::{ensembl::lookup_symbol, ncbi::query_symbols, uniprot::query};
use anyhow::Result;

pub fn info(search_terms: &[String], species: &str, taxon_id: usize) -> Result<InfoContainer> {
    let ensembl_query = lookup_symbol(search_terms, species)?;
    let uniprot_query = query(search_terms, &Some(taxon_id))?;
    let ncbi_query = query_symbols(search_terms, taxon_id)?;
    Ok(InfoContainer::from_queries(
        &ensembl_query,
        &uniprot_query,
        &ncbi_query,
    ))
}
