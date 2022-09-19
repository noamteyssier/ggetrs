use anyhow::Result;
use crate::ensembl::types::Releases;
use reqwest::blocking::Client;

/// Returns the most recent release from Ensembl REST API
pub fn release() -> Result<usize> {
    let client = Client::new();
    let url = "https://rest.ensembl.org/info/data";
    let results = client
        .get(url)
        .header("content-type", "application/json")
        .send()?
        .json::<Releases>()?;
    Ok(results.max())
}

#[cfg(test)]
mod testing {
    use super::release;

    #[test]
    fn test_ensembl_release() {
        let release = release().unwrap();
        assert_eq!(release, 107);
    }
}
