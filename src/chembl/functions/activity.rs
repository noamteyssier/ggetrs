use anyhow::Result;
use reqwest::blocking::Client;
use serde_json::Value;

pub fn activity(query: &str) -> Result<()> {
    let url = format!(
        "https://www.ebi.ac.uk/chembl/api/data/activity/search?q={}",
        query
        );
    let response = Client::new()
        .get(url)
        .header("accept", "application/json")
        .send()?
        .json::<Value>();
    println!("{:#?}", response);
    Ok(())
}
