use crate::ensembl::types::{SearchResult, SearchResults};
use anyhow::Result;
use mysql::{prelude::Queryable, Conn, OptsBuilder};

/// Performs an individual search on a SQL connection for a provided search term
fn search_term(conn: &mut Conn, search_term: &str) -> Result<SearchResults> {
    let query = build_search_query(search_term);
    let results = conn.query_map(query, |row| {
        SearchResult::from_row(&row).expect("unable to parse search results")
    })?;
    Ok(SearchResults(results))
}

/// Creates an SQL connection then iteratively performs searches for each provided term
pub fn search(db_name: &str, search_terms: &[String]) -> Result<SearchResults> {
    let opts = get_mysql_options(db_name);
    let mut conn = Conn::new(opts)?;
    let mut results = Vec::new();
    for term in search_terms.iter() {
        let mut term_results = search_term(&mut conn, term)?;
        results.append(&mut term_results.0);
    }
    Ok(SearchResults(results))
}

/// Generates mysql options
fn get_mysql_options(db_name: &str) -> OptsBuilder {
    OptsBuilder::new()
        .ip_or_hostname(Some("ensembldb.ensembl.org"))
        .tcp_port(3306)
        .user(Some("anonymous"))
        .db_name(Some(db_name))
}

/// Generates the search query.
///
/// Searches through all the descriptions and display labels for the provided search term
fn build_search_query(search_term: &str) -> String {
    format!(
        "SELECT gene.stable_id, xref.display_label, gene.description, xref.description, gene.biotype
        FROM gene
        LEFT JOIN xref ON gene.display_xref_id = xref.xref_id
        WHERE (gene.description LIKE '%{}%' OR xref.description LIKE '%{}%' OR xref.display_label LIKE '%{}%')",
        search_term,
        search_term,
        search_term
    )
}

#[cfg(test)]
mod testing {
    use super::search;

    #[test]
    fn test_search() {
        let db_name = "homo_sapiens_core_107_38";
        let search_terms = vec!["AP2S1".to_string()];
        let results = search(db_name, &search_terms).unwrap();
        assert_eq!(results.0.len(), 1);
        assert_eq!(results.0[0].display_label, "AP2S1");
    }
}
