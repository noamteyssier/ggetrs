use crate::archs4::types::{ResponseTissue, Species};
use reqwest::{blocking::Client, Result};

/// Returns the tissue-specific expression of a provided `gene_name`
pub fn tissue(gene_name: &str, species: &Species) -> Result<ResponseTissue> {
    let client = Client::new();
    let query_string = format!("search={}&species={}&type=tissue", gene_name, species);

    let url = format!(
        "https://maayanlab.cloud/archs4/search/loadExpressionTissue.php?{}",
        query_string
    );

    let raw_response = client
        .post(url)
        .header("Content-Type", "application/json")
        .send()?
        .text()?;

    let response = ResponseTissue::from_str(&raw_response);

    Ok(response)
}

#[cfg(test)]
mod testing {
    use super::tissue;
    use crate::archs4::Species;

    #[test]
    fn test_tissue() {
        let symbol = "AP2S1";
        let species = Species::Human;
        let results = tissue(symbol, &species).unwrap();
        assert!(results.results.len() > 1);
        assert_eq!(
            results.results[0].id,
            "System.Connective Tissue.Bone marrow.BONE MARROW"
        );
    }

    #[test]
    fn test_tissue_nonsense_query() {
        let symbol = "asiodaoiwdasd";
        let species = Species::Human;
        let results = tissue(symbol, &species).unwrap();
        assert_eq!(results.results.len(), 0);
    }
}
