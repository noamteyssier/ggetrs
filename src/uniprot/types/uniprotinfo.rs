use crate::utils::{FastaRecord, FastaRecords};
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, fmt};

// A container for UniprotInfo
#[derive(Serialize, Deserialize, Debug)]
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
    pub fn to_fasta(&self) -> String {
        self.0
            .values()
            .into_iter()
            .map(|x| x.to_fasta())
            .fold(String::new(), |acc, x| acc + &x)
    }
    pub fn fasta_records(&self) -> FastaRecords {
        let records = self.0.values().into_iter().map(|x| x.as_fasta()).collect();
        FastaRecords(records)
    }
}

/// A structure to handle the relevant results of a `Uniprot` query.
#[derive(Serialize, Deserialize, Debug)]
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
    #[must_use]
    pub fn from_value(value: &Value, query: &str) -> Result<Option<Self>> {
        if !Self::is_valid(value) {
            return Ok(None);
        }
        let uniprot_id = Self::get_uniprot_id(value)?;
        let uniprot_entry_name = Self::get_uniprot_entry_name(value)?;
        let primary_gene_name = Self::get_primary_gene_name(value)?;
        let uniprot_synonyms = Self::get_uniprot_synonyms(value);
        let protein_name = Self::get_protein_names(value)?;
        let uniprot_description = Self::get_uniprot_description(value)?;
        let ncbi_id = Self::get_ncbi_id(value);
        let pdb_id = Self::get_pdb_id(value);
        let taxon_id = Self::get_taxon_id(value)?;
        let organism_name = Self::get_organism_name(value)?;
        let sequence = Self::get_protein_sequence(value)?;
        let sequence_version = Self::get_sequence_version(value)?;
        let protein_existence = Self::get_protein_existence(value)?;
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

    fn is_valid(value: &Value) -> bool {
        !value["results"][0].is_null()
    }

    fn get_uniprot_id(value: &Value) -> Result<String> {
        if let Some(s) = value["results"][0]["primaryAccession"].as_str() {
            Ok(s.to_string())
        } else {
            bail!("Could not parse Uniprot ID")
        }
    }

    fn get_uniprot_entry_name(value: &Value) -> Result<String> {
        if let Some(s) = value["results"][0]["uniProtkbId"].as_str() {
            Ok(s.to_string())
        } else {
            bail!("Could not parse Uniprot Entry Name")
        }
    }

    fn get_primary_gene_name(value: &Value) -> Result<String> {
        if let Some(s) = value["results"][0]["genes"][0]["geneName"]["value"].as_str() {
            Ok(s.to_string())
        } else {
            bail!("Could not parse primary gene name")
        }
    }

    fn get_protein_sequence(value: &Value) -> Result<String> {
        if let Some(s) = value["results"][0]["sequence"]["value"].as_str() {
            Ok(s.to_string())
        } else {
            bail!("Could not protein sequence")
        }
    }

    fn get_uniprot_synonyms(value: &Value) -> Vec<String> {
        match value["results"][0]["genes"][0]["synonyms"].as_array() {
            Some(values) => values
                .iter()
                .map(|x| x["value"].as_str().unwrap().to_string())
                .collect(),
            None => Vec::new(),
        }
    }

    fn get_protein_names(value: &Value) -> Result<String> {
        if let Some(s) = value["results"][0]["proteinDescription"]["recommendedName"]["fullName"]
            ["value"]
            .as_str()
        {
            Ok(s.to_string())
        } else {
            bail!("Could not parse protein names")
        }
    }

    fn get_uniprot_description(value: &Value) -> Result<String> {
        if let Some(s) = value["results"][0]["comments"][0]["texts"][0]["value"].as_str() {
            Ok(s.to_string())
        } else {
            bail!("Could not parse uniprot description")
        }
    }

    fn get_ncbi_id(value: &Value) -> Option<String> {
        match value["results"][0]["uniProtKBCrossReferences"].as_array() {
            Some(values) => {
                let reference = values
                    .iter()
                    .filter(|x| x["database"] == "GeneID")
                    .take(1)
                    .next();
                reference.map(|v| v["id"].as_str().unwrap().to_string())
            }
            None => None,
        }
    }
    fn get_pdb_id(value: &Value) -> Option<String> {
        match value["results"][0]["uniProtKBCrossReferences"].as_array() {
            Some(values) => {
                let reference = values
                    .iter()
                    .filter(|x| x["database"] == "PDB")
                    .take(1)
                    .next();
                reference.map(|v| v["id"].as_str().unwrap().to_string())
            }
            None => None,
        }
    }

    fn get_taxon_id(value: &Value) -> Result<usize> {
        if let Some(s) = value["results"][0]["organism"]["taxonId"].as_u64() {
            Ok(s as usize)
        } else {
            bail!("Could not parse taxon id")
        }
    }

    fn get_organism_name(value: &Value) -> Result<String> {
        if let Some(s) = value["results"][0]["organism"]["commonName"].as_str() {
            Ok(s.to_string())
        } else if let Some(s) = value["results"][0]["organism"]["scientificName"].as_str() {
            Ok(s.to_string())
        } else {
            bail!("Could not parse organism name")
        }
    }

    fn get_sequence_version(value: &Value) -> Result<usize> {
        if let Some(s) = value["results"][0]["entryAudit"]["sequenceVersion"].as_u64() {
            Ok(s as usize)
        } else {
            bail!("Could not parse sequence version")
        }
    }

    fn get_protein_existence(value: &Value) -> Result<String> {
        if let Some(s) = value["results"][0]["proteinExistence"].as_str() {
            Ok(s.to_string().chars().nth(0).unwrap().to_string())
        } else {
            bail!("Could not parse protein existence")
        }
    }

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

    pub fn to_fasta(&self) -> String {
        format!("{}", self.as_fasta())
    }

    pub fn as_fasta(&self) -> FastaRecord {
        FastaRecord::new(&self.fasta_header(), &self.sequence)
    }
}
