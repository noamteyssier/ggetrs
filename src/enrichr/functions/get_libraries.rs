use crate::enrichr::types::ResponseLibraries;
use reqwest::Result;

/// Performs a `GET` call to retrieve the known libraries of `Enrichr`
pub fn get_libraries() -> Result<ResponseLibraries> {
    let url = "https://maayanlab.cloud/Enrichr/datasetStatistics";
    reqwest::blocking::get(url)?.json::<ResponseLibraries>()
}

#[cfg(test)]
mod testing {
    use super::get_libraries;

    #[test]
    fn test_get_libraries() {
        let libraries = get_libraries().unwrap();
        assert!(libraries.statistics.len() > 1);
    }
}
