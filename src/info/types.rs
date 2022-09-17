use crate::{
    ensembl::types::LookupResponse, ncbi::types::NcbiResults, uniprot::UniprotInfoContainer,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Container for `Info` which aggregates results from multiple databases
#[derive(Serialize, Deserialize, Debug)]
pub struct InfoContainer(pub HashMap<String, Info>);
impl fmt::Display for InfoContainer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).expect("cannot serialize")
        )
    }
}
impl InfoContainer {
    #[must_use] pub fn from_queries(
        ensembl: &LookupResponse,
        uniprot: &UniprotInfoContainer,
        ncbi: &NcbiResults,
    ) -> Self {
        let map = ncbi
            .0
            .keys()
            .filter_map(|k| Info::from_queries(ensembl, uniprot, ncbi, k).map(|info| (k.to_string(), info)))
            .collect();
        Self(map)
    }
}

/// Container which aggregates query results from multiple databases
#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    ensembl_id: String,
    uniprot_id: String,
    ncbi_id: String,
    symbol: String,
    ensembl_description: String,
    uniprot_description: String,
    ncbi_description: String,
    species: String,
    assembly_name: String,
}
impl fmt::Display for Info {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).expect("cannot serialize")
        )
    }
}
impl Info {
    #[must_use] pub fn from_queries(
        ensembl: &LookupResponse,
        uniprot: &UniprotInfoContainer,
        ncbi: &NcbiResults,
        key: &str,
    ) -> Option<Self> {
        let ensembl_result = match ensembl.0.get(key) {
            Some(opt_result) => match opt_result {
                Some(result) => result,
                None => return None,
            },
            None => return None,
        };
        let uniprot_result = match uniprot.0.get(key) {
            Some(result) => result,
            None => return None,
        };
        let ncbi_result = match ncbi.0.get(key) {
            Some(result) => result,
            None => return None,
        };
        let ensembl_id = ensembl_result.id.to_owned();
        let uniprot_id = uniprot_result.uniprot_id.to_owned();
        let ncbi_id = ncbi_result.gene_id.to_owned();
        let symbol = ncbi_result.symbol.to_owned();
        let ensembl_description = ensembl_result.description.to_owned();
        let uniprot_description = uniprot_result.uniprot_description.to_owned();
        let ncbi_description = ncbi_result.description.to_owned();
        let species = ensembl_result.species.to_owned();
        let assembly_name = ensembl_result.assembly_name.to_owned();

        Some(Self {
            ensembl_id,
            uniprot_id,
            ncbi_id,
            symbol,
            ensembl_description,
            uniprot_description,
            ncbi_description,
            species,
            assembly_name,
        })
    }
}