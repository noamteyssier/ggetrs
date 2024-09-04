use crate::utils::{FastaRecord, FastaRecords};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt};

// A container for UniprotInfo
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct UniprotInfoContainer(pub HashMap<String, UniprotInfo>);
impl fmt::Display for UniprotInfoContainer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).expect("cannot serialize")
        )
    }
}
impl UniprotInfoContainer {
    #[must_use]
    pub fn to_fasta(&self) -> String {
        self.0
            .values()
            .map(UniprotInfo::to_fasta)
            .fold(String::new(), |mut acc, x| {
                acc.push_str(&x);
                acc
            })
    }
    #[must_use]
    pub fn fasta_records(&self) -> FastaRecords {
        let records = self.0.values().map(UniprotInfo::as_fasta).collect();
        FastaRecords(records)
    }
}

/// A structure to handle the relevant results of a `Uniprot` query.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct UniprotInfo {
    pub uniprot_id: String,
    pub uniprot_entry_name: String,
    pub primary_gene_name: String,
    pub uniprot_synonyms: Vec<String>,
    pub protein_name: String,
    pub uniprot_description: String,
    pub ncbi_id: Option<String>,
    pub pdb_id: Option<String>,
    pub taxon_id: usize,
    pub organism_name: String,
    pub sequence: String,
    pub sequence_version: usize,
    pub protein_existence: String,
    pub query: String,
}

impl fmt::Display for UniprotInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).expect("cannot serialize")
        )
    }
}

impl UniprotInfo {
    pub fn from_value(value: &serde_json::Value, query: &str) -> Result<Option<Self>> {
        if !Self::is_valid(value) {
            return Ok(Some(Self::default()));
        }

        let uniprot_id = Self::get_uniprot_id(value).unwrap_or_default();
        let uniprot_entry_name = Self::get_uniprot_entry_name(value).unwrap_or_default();
        let primary_gene_name = Self::get_primary_gene_name(value).unwrap_or_default();
        let uniprot_synonyms = Self::get_uniprot_synonyms(value);
        let protein_name = Self::get_protein_names(value).unwrap_or_default();
        let uniprot_description = Self::get_uniprot_description(value).unwrap_or_default();
        let ncbi_id = Self::get_ncbi_id(value);
        let pdb_id = Self::get_pdb_id(value);
        let taxon_id = Self::get_taxon_id(value).unwrap_or_default();
        let organism_name = Self::get_organism_name(value).unwrap_or_default();
        let sequence = Self::get_protein_sequence(value).unwrap_or_default();
        let sequence_version = Self::get_sequence_version(value).unwrap_or_default();
        let protein_existence = Self::get_protein_existence(value).unwrap_or_default();
        let query = query.to_string();

        Ok(Some(Self {
            uniprot_id,
            uniprot_entry_name,
            primary_gene_name,
            uniprot_synonyms,
            protein_name,
            uniprot_description,
            ncbi_id,
            pdb_id,
            taxon_id,
            organism_name,
            sequence,
            sequence_version,
            protein_existence,
            query,
        }))
    }

    fn is_valid(value: &serde_json::Value) -> bool {
        !value["results"][0].is_null()
    }

    fn get_uniprot_id(value: &serde_json::Value) -> Result<String> {
        value["results"][0]["primaryAccession"]
            .as_str()
            .map(std::string::ToString::to_string)
            .ok_or_else(|| anyhow!("Could not parse Uniprot ID"))
    }

    fn get_uniprot_entry_name(value: &serde_json::Value) -> Result<String> {
        value["results"][0]["uniProtkbId"]
            .as_str()
            .map(std::string::ToString::to_string)
            .ok_or_else(|| anyhow!("Could not parse Uniprot Entry Name"))
    }

    fn get_primary_gene_name(value: &serde_json::Value) -> Result<String> {
        value["results"][0]["genes"][0]["geneName"]["value"]
            .as_str()
            .map(std::string::ToString::to_string)
            .ok_or_else(|| anyhow!("Could not parse primary gene name"))
    }

    fn get_protein_sequence(value: &serde_json::Value) -> Result<String> {
        value["results"][0]["sequence"]["value"]
            .as_str()
            .map(std::string::ToString::to_string)
            .ok_or_else(|| anyhow!("Could not parse protein sequence"))
    }

    fn get_uniprot_synonyms(value: &serde_json::Value) -> Vec<String> {
        value["results"][0]["genes"][0]["synonyms"]
            .as_array()
            .map_or(Vec::new(), |values| {
                values
                    .iter()
                    .map(|x| x["value"].as_str().unwrap_or_default().to_string())
                    .collect()
            })
    }

    fn get_protein_names(value: &serde_json::Value) -> Result<String> {
        value["results"][0]["proteinDescription"]["recommendedName"]["fullName"]["value"]
            .as_str()
            .map(std::string::ToString::to_string)
            .ok_or_else(|| anyhow!("Could not parse protein names"))
    }

    fn get_uniprot_description(value: &serde_json::Value) -> Result<String> {
        value["results"][0]["comments"][0]["texts"][0]["value"]
            .as_str()
            .map(std::string::ToString::to_string)
            .ok_or_else(|| anyhow!("Could not parse uniprot description"))
    }

    fn get_ncbi_id(value: &serde_json::Value) -> Option<String> {
        value["results"][0]["uniProtKBCrossReferences"]
            .as_array()
            .and_then(|values| {
                values
                    .iter()
                    .find(|x| x["database"] == "GeneID")
                    .and_then(|v| v["id"].as_str().map(std::string::ToString::to_string))
            })
    }

    fn get_pdb_id(value: &serde_json::Value) -> Option<String> {
        value["results"][0]["uniProtKBCrossReferences"]
            .as_array()
            .and_then(|values| {
                values
                    .iter()
                    .find(|x| x["database"] == "PDB")
                    .and_then(|v| v["id"].as_str().map(std::string::ToString::to_string))
            })
    }

    fn get_taxon_id(value: &serde_json::Value) -> Result<usize> {
        value["results"][0]["organism"]["taxonId"]
            .as_u64()
            .map(|s| s as usize)
            .ok_or_else(|| anyhow!("Could not parse taxon id"))
    }

    fn get_organism_name(value: &serde_json::Value) -> Result<String> {
        value["results"][0]["organism"]["commonName"]
            .as_str()
            .or_else(|| value["results"][0]["organism"]["scientificName"].as_str())
            .map(std::string::ToString::to_string)
            .ok_or_else(|| anyhow!("Could not parse organism name"))
    }

    fn get_sequence_version(value: &serde_json::Value) -> Result<usize> {
        value["results"][0]["entryAudit"]["sequenceVersion"]
            .as_u64()
            .map(|s| s as usize)
            .ok_or_else(|| anyhow!("Could not parse sequence version"))
    }

    fn get_protein_existence(value: &serde_json::Value) -> Result<String> {
        value["results"][0]["proteinExistence"]
            .as_str()
            .map(|s| s.chars().nth(0).unwrap().to_string())
            .ok_or_else(|| anyhow!("Could not parse protein existence"))
    }

    // Method to check if the struct is non-empty
    #[must_use]
    pub fn is_non_empty(&self) -> bool {
        !self.uniprot_id.is_empty()
            || !self.uniprot_entry_name.is_empty()
            || !self.primary_gene_name.is_empty()
            || !self.uniprot_synonyms.is_empty()
            || !self.protein_name.is_empty()
            || !self.uniprot_description.is_empty()
            || self.ncbi_id.is_some()
            || self.pdb_id.is_some()
            || self.taxon_id != 0
            || !self.organism_name.is_empty()
            || !self.sequence.is_empty()
            || self.sequence_version != 0
            || !self.protein_existence.is_empty()
            || !self.query.is_empty()
    }

    #[must_use]
    pub fn fasta_header(&self) -> String {
        format!(
            "sp|{}|{} {} OS={} OX={} [GN={} ] PE={} SV={}",
            self.uniprot_id,
            self.uniprot_entry_name,
            self.protein_name,
            self.organism_name,
            self.taxon_id,
            self.primary_gene_name,
            self.protein_existence,
            self.sequence_version,
        )
    }

    #[must_use]
    pub fn to_fasta(&self) -> String {
        format!("{}", self.as_fasta())
    }

    #[must_use]
    pub fn as_fasta(&self) -> FastaRecord {
        FastaRecord::new(&self.fasta_header(), &self.sequence)
    }
}
