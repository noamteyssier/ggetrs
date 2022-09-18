use anyhow::Result;
use reqwest::blocking::Client;
use crate::chembl::types::ActivityResponse;

pub fn activity(query: &str, limit: usize) -> Result<ActivityResponse> {
    let url = format!(
        "https://www.ebi.ac.uk/chembl/api/data/activity/search?q={}&limit={}",
        query,
        limit
        );
    let response = Client::new()
        .get(url)
        .header("accept", "application/json")
        .send()?
        .json::<ActivityResponse>()?;
    Ok(response)
}
