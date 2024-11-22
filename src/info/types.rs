use crate::{
    ensembl::types::LookupResponse, ncbi::types::NcbiResults, uniprot::UniprotInfoContainer,
};
use pyo3::types::{IntoPyDict, PyDict, PyDictMethods};
use pyo3::{Bound, PyResult};
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
impl<'py> IntoPyDict<'py> for InfoContainer {
    fn into_py_dict(self, py: pyo3::Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let map = PyDict::new(py);
        self.0.iter().for_each(|(k, v)| {
            map.set_item(k, v.clone().into_py_dict(py).unwrap())
                .unwrap();
        });
        Ok(map)
    }
}
impl InfoContainer {
    #[must_use]
    pub fn from_queries(
        ensembl: &LookupResponse,
        uniprot: &UniprotInfoContainer,
        ncbi: &NcbiResults,
    ) -> Self {
        let map = ncbi
            .0
            .keys()
            .filter_map(|k| {
                Info::from_queries(ensembl, uniprot, ncbi, k).map(|info| (k.to_string(), info))
            })
            .collect();
        Self(map)
    }
}

/// Container which aggregates query results from multiple databases
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Info {
    ensembl_id: String,
    uniprot_id: String,
    ncbi_id: String,
    pdb_id: Option<String>,
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
impl<'py> IntoPyDict<'py> for Info {
    fn into_py_dict(self, py: pyo3::Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let map = PyDict::new(py);
        map.set_item("ensembl_id", &self.ensembl_id).unwrap();
        map.set_item("uniprot_id", &self.uniprot_id).unwrap();
        map.set_item("ncbi_id", &self.ncbi_id).unwrap();
        map.set_item("symbol", &self.symbol).unwrap();
        map.set_item("pdb_id", &self.pdb_id).unwrap();
        map.set_item("ensembl_description", &self.ensembl_description)
            .unwrap();
        map.set_item("uniprot_description", &self.uniprot_description)
            .unwrap();
        map.set_item("ncbi_description", &self.ncbi_description)
            .unwrap();
        map.set_item("species", &self.species).unwrap();
        map.set_item("assembly_name", &self.assembly_name).unwrap();
        Ok(map)
    }
}
impl Info {
    #[must_use]
    pub fn from_queries(
        ensembl: &LookupResponse,
        uniprot: &UniprotInfoContainer,
        ncbi: &NcbiResults,
        key: &str,
    ) -> Option<Self> {
        let ensembl_result = ensembl.0.get(key)?.as_ref()?;
        let uniprot_result = uniprot.0.get(key)?;
        let ncbi_result = ncbi.0.get(key)?;
        let ensembl_id = ensembl_result.id.clone();
        let uniprot_id = uniprot_result.uniprot_id.clone();
        let ncbi_id = ncbi_result.gene_id.clone();
        let symbol = ncbi_result.symbol.clone();
        let ensembl_description = ensembl_result.description.clone();
        let uniprot_description = uniprot_result.uniprot_description.clone();
        let ncbi_description = ncbi_result.description.clone();
        let species = ensembl_result.species.clone();
        let assembly_name = ensembl_result.assembly_name.clone();
        let pdb_id = uniprot_result.pdb_id.clone();

        Some(Self {
            ensembl_id,
            uniprot_id,
            ncbi_id,
            pdb_id,
            symbol,
            ensembl_description,
            uniprot_description,
            ncbi_description,
            species,
            assembly_name,
        })
    }
}
