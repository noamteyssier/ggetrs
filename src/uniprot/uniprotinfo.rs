use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

/// A container of [`UniprotInfo`]
#[derive(Serialize, Deserialize, Debug)]
pub struct UniprotInfoContainer(pub Vec<UniprotInfo>);
impl fmt::Display for UniprotInfoContainer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).expect("cannot serialize")
        )
    }
}

/// A structure to handle the relevant results of a `Uniprot` query.
#[derive(Serialize, Deserialize, Debug)]
pub struct UniprotInfo {
    uniprot_id: String,
    primary_gene_name: String,
    uniprot_synonyms: Vec<String>,
    protein_name: String,
    uniprot_description: String,
    ncbi_id: Option<String>,
    pdb_id: Option<String>,
    taxon_id: usize,
    organism_name: String,
    query: String,
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
    pub fn from_value(value: &Value, query: &str) -> Option<Self> {
        if !Self::is_valid(value) {
            return None;
        }
        let uniprot_id = Self::get_uniprot_id(value);
        let primary_gene_name = Self::get_primary_gene_name(value);
        let uniprot_synonyms = Self::get_uniprot_synonyms(value);
        let protein_name = Self::get_protein_names(value);
        let uniprot_description = Self::get_uniprot_description(value);
        let ncbi_id = Self::get_ncbi_id(value);
        let pdb_id = Self::get_pdb_id(value);
        let taxon_id = Self::get_taxon_id(value);
        let organism_name = Self::get_organism_name(value);
        let query = query.to_string();
        Some(Self {
            uniprot_id,
            primary_gene_name,
            uniprot_synonyms,
            protein_name,
            uniprot_description,
            ncbi_id,
            pdb_id,
            taxon_id,
            organism_name,
            query,
        })
    }

    fn is_valid(value: &Value) -> bool {
        !value["results"][0].is_null()
    }

    fn get_uniprot_id(value: &Value) -> String {
        value["results"][0]["primaryAccession"]
            .as_str()
            .unwrap()
            .to_string()
    }

    fn get_primary_gene_name(value: &Value) -> String {
        value["results"][0]["genes"][0]["geneName"]["value"]
            .as_str()
            .unwrap()
            .to_string()
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

    fn get_protein_names(value: &Value) -> String {
        value["results"][0]["proteinDescription"]["recommendedName"]["fullName"]["value"]
            .as_str()
            .unwrap()
            .to_string()
    }

    fn get_uniprot_description(value: &Value) -> String {
        value["results"][0]["comments"][0]["texts"][0]["value"]
            .as_str()
            .unwrap()
            .to_string()
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

    fn get_taxon_id(value: &Value) -> usize {
        value["results"][0]["organism"]["taxonId"]
            .as_u64()
            .expect("Missing taxon_id") as usize
    }

    fn get_organism_name(value: &Value) -> String {
        value["results"][0]["organism"]["commonName"]
            .as_str()
            .unwrap()
            .to_string()
    }
}
