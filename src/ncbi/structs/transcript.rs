use serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::utils::parsing::{
    parse_primary_string,
    parse_primary_usize,
    parse_primary_optional_string,
    parse_secondary_string
};

#[derive(Debug, Deserialize, Serialize)]
pub struct NcbiTranscript {
    name: String,
    biotype: String,
    ensembl_transcript: Option<String>,
    length: usize,
    accession_version: String,
    start: usize,
    end: usize,
    strand: String
}
impl NcbiTranscript {
    fn is_null(value: &Value) -> bool {
        value["name"].is_null()
    }

    pub fn from_value(value: &Value) -> Option<Self> {
        if Self::is_null(value) { return None }
        let name = parse_primary_string(value, "name");
        let ensembl_transcript = parse_primary_optional_string(value, "ensembl_transcript");
        let biotype = parse_primary_string(value, "type");
        let length = parse_primary_usize(value, "length");
        let accession_version = parse_secondary_string(value, "exons", "accession_version");
        let start = Self::parse_genomic_range(value, "begin")
            .parse::<usize>().expect("Malformed genomic range start");
        let end = Self::parse_genomic_range(value, "end")
            .parse::<usize>().expect("Malformed genomic range end");
        let strand = Self::parse_genomic_range(value, "orientation");
        Some(Self{name, ensembl_transcript, biotype, length, accession_version, start, end, strand})
    }

    fn parse_genomic_range(value: &Value, id: &str) -> String {
        value["genomic_range"]["range"][0][id]
            .as_str()
            .expect(&format!("Missing: genomic_range/range/[0]/{}", id))
            .to_string()
    }
}
