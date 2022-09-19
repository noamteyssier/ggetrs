use crate::pdb::types::PdbResource;
use anyhow::{bail, Result};
use reqwest::blocking::Client;
use serde_json::Value;

/// requests information from pdb resource
pub fn resource_info(
    pdb_id: &str,
    resource: &PdbResource,
    identifier: &Option<String>,
) -> Result<Value> {
    let base_url = "https://data.rcsb.org/rest/v1/core";
    if identifier.is_none() {
        if resource.requires_entity_id() {
            bail!(
                "Provided resource requires an entity ID (e.g. '1'): {}",
                resource
            );
        } else if resource.requires_chain_id() {
            bail!(
                "Provided resource requires a chain ID (e.g. 'A'): {}",
                resource
            );
        } else if resource.requires_assembly_id() {
            bail!(
                "Provided resource requires an assembly ID (e.g. '1'): {}",
                resource
            );
        }
    }
    let rest_url = if resource.requires_identifier() {
        format!(
            "{}/{}/{}/{}",
            base_url,
            resource,
            pdb_id,
            (identifier.clone()).expect("Resource Identifier Required")
        )
    } else {
        format!("{}/{}/{}", base_url, resource, pdb_id)
    };
    let response = Client::new().get(rest_url).send()?.json::<Value>()?;
    match response["status"].as_u64() {
        Some(code) => bail!(format!("Error: HTTP Code {}", code)),
        None => Ok(response),
    }
}

#[cfg(test)]
mod testing {
    use super::resource_info;
    use crate::pdb::types::PdbResource;

    #[test]
    fn test_resource_entry() {
        let pdb_id = "6URI"; // AP2S1
        let resource = PdbResource::Entry;
        let identifier = None;
        let info = resource_info(pdb_id, &resource, &identifier).unwrap();
        assert_eq!(info["entry"]["id"].as_str(), Some(pdb_id));
    }

    #[test]
    fn test_resource_pubmed() {
        let pdb_id = "6URI"; // AP2S1
        let resource = PdbResource::Pubmed;
        let identifier = None;
        let info = resource_info(pdb_id, &resource, &identifier).unwrap();
        assert_eq!(info["rcsb_id"].as_str(), Some("32719457"));
    }

    #[test]
    fn test_resource_assembly() {
        let pdb_id = "6URI"; // AP2S1
        let resource = PdbResource::Assembly;
        let identifier = Some(String::from("1"));
        let info = resource_info(pdb_id, &resource, &identifier).unwrap();
        assert_eq!(info["rcsb_id"].as_str(), Some("6URI-1"));
    }

    #[test]
    fn test_resource_missing_id() {
        let pdb_id = "MISSINGID";
        let resource = PdbResource::Entry;
        let identifier = None;
        let response = resource_info(pdb_id, &resource, &identifier);
        assert!(response.is_err());
    }

    #[test]
    fn test_resource_missing_identifier() {
        let pdb_id = "6URI";
        let identifier = None;
        let resources = vec![
            PdbResource::Assembly,
            PdbResource::BranchedEntity,
            PdbResource::NonpolymerEntity,
            PdbResource::PolymerEntity,
            PdbResource::Uniprot,
            PdbResource::BranchedEntityInstance,
            PdbResource::PolymerEntityInstance,
            PdbResource::NonpolymerEntityInstance,
        ];
        for resource in resources {
            let response = resource_info(pdb_id, &resource, &identifier);
            assert!(response.is_err());
        }
    }
}
