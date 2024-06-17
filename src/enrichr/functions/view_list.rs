use super::{ENRICHR_URL, SPEEDRICHR_URL};
use crate::enrichr::types::ResponseViewList;
use reqwest::Result;

/// Performs a `GET` call to retrieve the genes within a `userListId`.
pub fn view_list(user_list_id: usize, speedrichr: bool) -> Result<ResponseViewList> {
    let url = if speedrichr {
        format!("{SPEEDRICHR_URL}/api/view?userListId={user_list_id}")
    } else {
        format!("{ENRICHR_URL}/view?userListId={user_list_id}")
    };
    reqwest::blocking::get(url)?.json::<ResponseViewList>()
}

#[cfg(test)]
mod testing {
    use super::view_list;
    use crate::enrichr::add_list;

    fn get_list_id() -> usize {
        let gene_list = ["AP2S1", "NSD1", "LDB1"]
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        let response = add_list(&gene_list, false).unwrap();
        response.user_list_id
    }

    fn get_list_id_with_background() -> usize {
        let gene_list = ["AP2S1", "NSD1", "LDB1"]
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        let response = add_list(&gene_list, true).unwrap();
        response.user_list_id
    }

    #[test]
    fn test_view_list() {
        let user_list_id = get_list_id();
        let response = view_list(user_list_id, false).unwrap();
        assert_eq!(response.genes.len(), 3);
        assert!(response.genes.contains(&"AP2S1".to_string()));
        assert!(response.genes.contains(&"NSD1".to_string()));
        assert!(response.genes.contains(&"LDB1".to_string()));
    }

    #[test]
    fn test_view_list_with_background() {
        let user_list_id = get_list_id_with_background();
        println!("{user_list_id:?}");
        let response = view_list(user_list_id, true).unwrap();
        println!("{response:#?}");
        assert_eq!(response.genes.len(), 3);
        assert!(response.genes.contains(&"AP2S1".to_string()));
        assert!(response.genes.contains(&"NSD1".to_string()));
        assert!(response.genes.contains(&"LDB1".to_string()));
    }
}
