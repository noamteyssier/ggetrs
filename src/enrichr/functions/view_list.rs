use crate::enrichr::types::ResponseViewList;
use reqwest::Result;

/// Performs a `GET` call to retrieve the genes within a `userListId`.
pub fn view_list(user_list_id: usize) -> Result<ResponseViewList> {
    let url = format!(
        "https://maayanlab.cloud/Enrichr/view?userListId={}",
        user_list_id
    );
    reqwest::blocking::get(&url)?.json::<ResponseViewList>()
}

#[cfg(test)]
mod testing {
    use super::view_list;
    use crate::enrichr::add_list;

    fn get_list_id() -> usize {
        let gene_list = vec!["AP2S1", "NSD1", "LDB1"]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let response = add_list(&gene_list).unwrap();
        response.user_list_id
    }

    #[test]
    fn test_view_list() {
        let user_list_id = get_list_id();
        let response = view_list(user_list_id).unwrap();
        assert_eq!(response.genes.len(), 3);
        assert!(response.genes.contains(&"AP2S1".to_string()));
        assert!(response.genes.contains(&"NSD1".to_string()));
        assert!(response.genes.contains(&"LDB1".to_string()));
    }
}
