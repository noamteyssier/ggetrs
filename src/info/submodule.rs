use anyhow::Result;
use super::info;

pub fn launch_info(search_terms: &[String], species: &str, taxon_id: usize) -> Result<()> {
    info(search_terms, species, taxon_id)?;
    Ok(())
}
