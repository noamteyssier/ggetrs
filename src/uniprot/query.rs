use futures::future::join_all;
use reqwest::Client;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct UniprotInfoContainer (
    Vec<UniprotInfo>
);
impl fmt::Display for UniprotInfoContainer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).expect("cannot serialize"))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UniprotInfo {
    uniprot_id: String,
    primary_gene_name: String,
    uniprot_synonyms: Vec<String>,
    protein_name: String,
    uniprot_description: String,
    ncbi_id: Option<String>,
    pdb_id: Option<String>,
    query: String
}
impl fmt::Display for UniprotInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).expect("cannot serialize"))
    }
}
impl UniprotInfo {
    pub fn from_value(value: Value, query: &str) -> Option<Self> {
        if !Self::is_valid(&value) { return None }
        let uniprot_id = Self::get_uniprot_id(&value);
        let primary_gene_name = Self::get_primary_gene_name(&value);
        let uniprot_synonyms = Self::get_uniprot_synonyms(&value);
        let protein_name = Self::get_protein_names(&value);
        let uniprot_description = Self::get_uniprot_description(&value);
        let ncbi_id = Self::get_ncbi_id(&value);
        let pdb_id = Self::get_pdb_id(&value);
        let query = query.to_string();
        Some(Self {
            uniprot_id,
            primary_gene_name,
            uniprot_synonyms,
            protein_name,
            uniprot_description,
            ncbi_id,
            pdb_id,
            query 
        })
    }

    fn is_valid(value: &Value) -> bool {
        !value["results"][0].is_null()
    }

    fn get_uniprot_id(value: &Value) -> String {
        value["results"][0]["primaryAccession"]
            .as_str().unwrap().to_string()
    }

    fn get_primary_gene_name(value: &Value) -> String {
        value["results"][0]["genes"][0]["geneName"]["value"]
            .as_str().unwrap().to_string()
    }

    fn get_uniprot_synonyms(value: &Value) -> Vec<String> {
        match value["results"][0]["genes"][0]["synonyms"].as_array() {
            Some(values) => {
                values
                    .iter()
                    .map(|x| x["value"].as_str().unwrap().to_string())
                    .collect()

            },
            None => Vec::new()
        }
    }

    fn get_protein_names(value: &Value) -> String {
        value["results"][0]["proteinDescription"]["recommendedName"]["fullName"]["value"]
            .as_str().unwrap().to_string()
    }

    fn get_uniprot_description(value: &Value) -> String {
        value["results"][0]["comments"][0]["texts"][0]["value"]
            .as_str().unwrap().to_string()
    }

    fn get_ncbi_id(value: &Value) -> Option<String> {
        match value["results"][0]["uniProtKBCrossReferences"].as_array() {
            Some(values) => {
                let reference = values
                    .iter()
                    .filter(|x| x["database"] == "GeneID")
                    .take(1)
                    .next();
                match reference {
                    Some(v) => {
                        Some(v["id"].as_str().unwrap().to_string())
                    },
                    None => None
                }

            },
            None => None
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
                match reference {
                    Some(v) => {
                        Some(v["id"].as_str().unwrap().to_string())
                    },
                    None => None
                }

            },
            None => None
        }
    }
}

async fn async_query_uniprot(gene: &str) -> reqwest::Result<Option<UniprotInfo>> {
    let query = if gene.starts_with("ENS") {
        gene.to_string()
    } else {
        format!("(gene:{})", gene)
    };
    let url = format!(
        "https://rest.uniprot.org/uniprotkb/search?query={}+AND+reviewed:true",
        query 
    );
    Client::new()
        .get(url)
        .header("content-type", "application/json")
        .send()
        .await?
        .json::<Value>()
        .await
        .map(|x| UniprotInfo::from_value(x, gene))
}

async fn async_query_uniprot_multiple(ensembl_ids: &Vec<String>) -> reqwest::Result<Vec<reqwest::Result<Option<UniprotInfo>>>> {
    let query_handles = ensembl_ids
        .iter()
        .map(|x| async_query_uniprot(x));

    let results = join_all(query_handles).await;
    Ok(results)
}

pub fn query(terms: &Vec<String>) -> anyhow::Result<UniprotInfoContainer> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    let results = rt.block_on(async move {
        async_query_uniprot_multiple(terms).await.expect("could not query uniprot")
    });
    let results = results
        .into_iter()
        .filter_map(|x| x.expect("could not create results"))
        .collect();
    Ok(UniprotInfoContainer(results))

}
