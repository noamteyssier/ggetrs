use anyhow::Result;
use reqwest::blocking::Client;

use crate::pdb::types::PdbFormat;

/// requests a pdb structure from rcsb pdb
pub fn structure(pdb_id: &str, header_only: bool, format: &PdbFormat) -> Result<Option<String>> {
    let base_url = if header_only {
        "https://files.rcsb.org/header"
    } else {
        "https://files.rcsb.org/view"
    };
    let url = format!("{}/{}.{}", base_url, pdb_id, format);
    let response = Client::new().get(url).send()?;
    if response.status().is_success() {
        Ok(Some(response.text()?))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod testing {
    use super::structure;
    use crate::pdb::types::PdbFormat;

    #[test]
    fn test_structure_pdb() {
        let pdb_id = "6URI";
        let header_only = false;
        let format = PdbFormat::Pdb;
        let response = structure(pdb_id, header_only, &format).unwrap();
        assert!(response.is_some());
    }

    #[test]
    fn test_structure_cif() {
        let pdb_id = "6URI";
        let header_only = false;
        let format = PdbFormat::Cif;
        let response = structure(pdb_id, header_only, &format).unwrap();
        assert!(response.is_some());
    }

    #[test]
    fn test_structure_pdb_header() {
        let pdb_id = "6URI";
        let header_only = true;
        let format = PdbFormat::Pdb;
        let response = structure(pdb_id, header_only, &format).unwrap();
        assert!(response.is_some());
    }

    #[test]
    fn test_structure_cif_header() {
        let pdb_id = "6URI";
        let header_only = true;
        let format = PdbFormat::Cif;
        let response = structure(pdb_id, header_only, &format).unwrap();
        assert!(response.is_some());
    }

    #[test]
    fn test_structure_missing_id() {
        let pdb_id = "BLAHBLAHBLAH";
        let header_only = false;
        let format = PdbFormat::Pdb;
        let response = structure(pdb_id, header_only, &format).unwrap();
        assert!(response.is_none());
    }
}
