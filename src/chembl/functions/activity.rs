use crate::chembl::types::ActivityResponse;
use anyhow::Result;
use reqwest::blocking::Client;

/// Queries information from Chembl given a single query
pub fn activity(query: &str, limit: usize) -> Result<ActivityResponse> {
    let url = format!(
        "https://www.ebi.ac.uk/chembl/api/data/activity/search?q={}&limit={}",
        query, limit
    );
    let response = Client::new()
        .get(url)
        .header("accept", "application/json")
        .send()?
        .json::<ActivityResponse>()?;
    Ok(response)
}

#[cfg(test)]
mod testing {
    use super::activity;

    #[test]
    fn test_chembl_activity() {
        let query = "AP2S1";
        let limit = 1;
        let response = activity(query, limit).unwrap();
        assert_eq!(response.activities.len(), 1);
        assert_eq!(response.activities[0].activity_id, 18905243);
    }

    #[test]
    fn test_chembl_activity_nonsense_query() {
        let query = "AIWDJAIWDAW";
        let limit = 1;
        let response = activity(query, limit).unwrap();
        assert_eq!(response.activities.len(), 0);
    }
}
