use crate::archs4::types::{Correlations, ResultCorrelation};
use anyhow::{bail, Result};
use reqwest::blocking::Client;
use std::collections::HashMap;

/// Queries the most correlated genes for a provided gene.
///
/// Uses the ARCHS4 correlation API
pub fn correlation(gene_name: &str, count: usize) -> Result<Correlations> {
    let client = Client::new();
    let url = "https://maayanlab.cloud/matrixapi/coltop";
    let map = build_query(gene_name, count);
    let correlations = client
        .post(url)
        .json(&map)
        .send()?
        .json::<ResultCorrelation>();
    match correlations {
        Err(_) => bail!(format!(
            "Could not parse response for symbol: {}",
            gene_name
        )),
        Ok(rc) => Ok(rc.into()),
    }
}

/// Builds the `HashMap` to be converted to a JSON
fn build_query(gene_name: &str, count: usize) -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert(String::from("id"), gene_name.to_string());
    map.insert(String::from("count"), format!("{}", count));
    map
}

#[cfg(test)]
mod testing {
    use super::correlation;

    #[test]
    fn test_correlation() {
        let symbol = "AP2S1";
        let count = 5;
        let results = correlation(symbol, count).unwrap();
        assert_eq!(results.correlations.len(), count);
        assert_eq!(results.correlations[0].gene_symbol, "AP2S1");
        assert_eq!(results.correlations[1].gene_symbol, "ASNA1");
        assert_eq!(results.correlations[2].gene_symbol, "MRPL28");
        assert_eq!(results.correlations[3].gene_symbol, "SSNA1");
        assert_eq!(results.correlations[4].gene_symbol, "COX8A");
    }

    #[test]
    fn test_correlation_nonsense_query() {
        let symbol = "AJIWDJOAWD";
        let count = 5;
        let results = correlation(symbol, count);
        assert!(results.is_err());
    }
}
