use serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::utils::parsing::parse_secondary_string;
use super::NcbiTranscript;

#[derive(Debug, Deserialize, Serialize)]
pub struct NcbiInfo {
    gene_id: String,
    symbol: String,
    description: String,
    taxon_id: String,
    taxon_name: String,
    transcripts: Vec<NcbiTranscript>
}
impl NcbiInfo {
    fn is_null(value: &Value) -> bool {
        value["gene"].is_null()
    }

    #[must_use] pub fn from_value(value: &Value) -> Option<Self> {
        if Self::is_null(value) { return None }
        let gene_id = parse_secondary_string(value, "gene", "gene_id");
        let symbol = parse_secondary_string(value, "gene", "symbol");
        let description = parse_secondary_string(value, "gene", "description");
        let taxon_id = parse_secondary_string(value, "gene", "tax_id");
        let taxon_name = parse_secondary_string(value, "gene", "taxname");
        let transcripts = Self::parse_transcripts(value);
        Some(Self {
            gene_id,
            symbol,
            description,
            taxon_id,
            taxon_name,
            transcripts
        })
    }

    fn parse_transcripts(value: &Value) -> Vec<NcbiTranscript> {
        match value["gene"]["transcripts"].as_array() {
            Some(arr) => {
                arr.iter().filter_map(NcbiTranscript::from_value).collect()
            },
            None => Vec::new()
        }

    }
}
