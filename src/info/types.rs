use crate::{ensembl::types::LookupResponse, uniprot::UniprotInfoContainer, ncbi::types::NcbiResults};

pub struct InfoContainer {
    results: Vec<Info>
}
impl InfoContainer {
    pub fn from_queries(
        ensembl: &LookupResponse, 
        uniprot: &UniprotInfoContainer, 
        ncbi: &NcbiResults) -> Self 
    {
        let results = Vec::new();
        Self { results }
    }

}

pub struct Info;
impl Info {
    pub fn from_queries(
        ensembl: &LookupResponse, 
        uniprot: &UniprotInfoContainer, 
        ncbi: &NcbiResults) -> Self 
    {
        Self
    }
}
