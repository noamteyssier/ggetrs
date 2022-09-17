use crate::pdb::types::PdbResource;
use anyhow::{Result, bail};
use reqwest::blocking::Client;
use serde_json::Value;

/// requests information from pdb resource
pub fn resource_info(pdb_id: &str, resource: &PdbResource, identifier: &Option<String>) -> Result<Value> {
    let base_url = "https://data.rcsb.org/rest/v1/core";
    if identifier.is_none() {
        if resource.requires_entity_id() {
            bail!("Provided resource requires an entity ID (e.g. '1'): {}", resource);
        } else if resource.requires_chain_id() {
            bail!("Provided resource requires a chain ID (e.g. 'A'): {}", resource);
        } else if resource.requires_assembly_id() {
            bail!("Provided resource requires an assembly ID (e.g. '1'): {}", resource);
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
        format!(
            "{}/{}/{}",
            base_url,
            resource,
            pdb_id
            )
    };
    let response = Client::new()
        .get(rest_url)
        .send()?
        .json::<Value>()?;
    Ok(response)
}
