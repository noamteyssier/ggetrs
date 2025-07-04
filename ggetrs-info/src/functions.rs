use anyhow::Result;

use ggetrs_ensembl::lookup_symbol;
use ggetrs_ncbi::query_symbols;
use ggetrs_uniprot::query;

use super::types::InfoContainer;

pub fn info(search_terms: &[String], species: &str, taxon_id: usize) -> Result<InfoContainer> {
    let ensembl_query = lookup_symbol(search_terms, species)?;
    let uniprot_query = query(search_terms, false, &Some(taxon_id))?;
    let ncbi_query = query_symbols(search_terms, taxon_id)?;
    Ok(InfoContainer::from_queries(
        &ensembl_query,
        &uniprot_query,
        &ncbi_query,
    ))
}

#[cfg(test)]
mod testing {
    use super::info;

    #[test]
    fn test_info() {
        let search_terms = vec!["AP2S1".to_string()];
        let species = "homo_sapiens";
        let taxon_id = 9606;
        let response = info(&search_terms, species, taxon_id);
        assert!(response.is_ok());
    }

    #[test]
    fn test_info_nonsense_query() {
        let search_terms = vec!["ALSKDOAWKDASDN".to_string()];
        let species = "homo_sapiens";
        let taxon_id = 9606;
        let response = info(&search_terms, species, taxon_id);
        let info_results = response.unwrap();
        assert!(info_results.0.is_empty());
    }

    #[test]
    fn test_info_mismatch_species() {
        let search_terms = vec!["AP2S1".to_string()];
        let species = "homo_sapiens";
        let taxon_id = 0000;
        let response = info(&search_terms, species, taxon_id);
        assert!(response.is_err());
    }
}
