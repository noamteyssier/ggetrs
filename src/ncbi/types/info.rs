use super::NcbiTranscript;
use crate::utils::parsing::{
    parse_primary_vec_string, parse_secondary_string, parse_secondary_vec_optional_string,
    parse_secondary_vec_string,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct NcbiInfo {
    pub gene_id: String,
    pub symbol: String,
    pub ensembl_ids: Vec<String>,
    pub uniprot_ids: Vec<String>,
    pub synonyms: Option<Vec<String>>,
    pub chromosomes: Vec<String>,
    pub description: String,
    pub taxon_id: String,
    pub taxon_name: String,
    pub transcripts: Vec<NcbiTranscript>,
    pub query: String,
}
impl NcbiInfo {
    fn is_null(value: &Value) -> bool {
        value["gene"].is_null()
    }

    #[must_use]
    pub fn from_value(value: &Value) -> Option<Self> {
        if Self::is_null(value) {
            return None;
        }
        let gene_id = parse_secondary_string(value, "gene", "gene_id");
        let symbol = parse_secondary_string(value, "gene", "symbol");
        let ensembl_ids = parse_secondary_vec_string(value, "gene", "ensembl_gene_ids");
        let uniprot_ids = parse_secondary_vec_string(value, "gene", "swiss_prot_accessions");
        let synonyms = parse_secondary_vec_optional_string(value, "gene", "synonyms");
        let chromosomes = parse_secondary_vec_string(value, "gene", "chromosomes");
        let description = parse_secondary_string(value, "gene", "description");
        let taxon_id = parse_secondary_string(value, "gene", "tax_id");
        let taxon_name = parse_secondary_string(value, "gene", "taxname");
        let transcripts = Self::parse_transcripts(value);
        let query = parse_primary_vec_string(value, "query")[0].clone();
        Some(Self {
            gene_id,
            symbol,
            ensembl_ids,
            uniprot_ids,
            synonyms,
            chromosomes,
            description,
            taxon_id,
            taxon_name,
            transcripts,
            query,
        })
    }

    fn parse_transcripts(value: &Value) -> Vec<NcbiTranscript> {
        match value["gene"]["transcripts"].as_array() {
            Some(arr) => arr.iter().filter_map(NcbiTranscript::from_value).collect(),
            None => Vec::new(),
        }
    }
}
