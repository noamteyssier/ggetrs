use mysql::{Conn, OptsBuilder, Row, prelude::Queryable};
use anyhow::bail;
use serde::{Deserialize, Serialize};
use pyo3::{Python, PyResult, types::PyDict};
use std::fmt;

/// A unit struct container of [`SearchResult`]
#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResults ( Vec<SearchResult> );
impl fmt::Display for SearchResults {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).expect("cannot serialize"))
    }
}
impl SearchResults {
    pub fn as_pydict<'py>(&self, py: Python<'py>) -> PyResult<&'py PyDict> {
        let dict = PyDict::new(py);
        dict.set_item(
            "results", 
            self.0
                .iter()
                .map(|x| x.as_pydict(py).expect("could not create pydict"))
                .collect::<Vec<&PyDict>>()
        )?;
        Ok(dict)
    }
}

/// An individual search result generated from the `Ensembl` SQL query
#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResult {
    stable_id: String,
    display_label: String,
    ensembl_description: String,
    xref_description: String,
    biotype: String
}
impl fmt::Display for SearchResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).expect("cannot serialize"))
    }
}
impl SearchResult {
    /// Will parse the provided index into a string or return an empty string if it is null
    fn parse_index(row: &Row, index: usize) -> String {
        if let Some(Ok(value)) = row.get_opt::<String, usize>(index) {
            value
        } else {
            String::new()
        }
    }

    /// Generates from a [`Row`]
    pub fn from_row(row: Row) -> anyhow::Result<Self> {
        if row.is_empty() { bail!("empty row") }
        let stable_id = Self::parse_index(&row, 0);
        let display_label = Self::parse_index(&row, 1);
        let ensembl_description = Self::parse_index(&row, 2);
        let xref_description = Self::parse_index(&row, 3);
        let biotype = Self::parse_index(&row, 4);

        Ok(Self {
            stable_id, display_label, ensembl_description, xref_description, biotype
        })
    }

    pub fn as_pydict<'py>(&self, py: Python<'py>) -> PyResult<&'py PyDict> {
        let dict = PyDict::new(py);
        dict.set_item("stable_id", &self.stable_id)?;
        dict.set_item("display_label", &self.display_label)?;
        dict.set_item("ensembl_description", &self.ensembl_description)?;
        dict.set_item("xref_description", &self.xref_description)?;
        dict.set_item("biotype", &self.biotype)?;
        Ok(dict)
    }
}

/// Performs an individual search on a SQL connection for a provided search term
fn search_term(conn: &mut Conn, search_term: &str) -> anyhow::Result<SearchResults> {
    let query = build_search_query(search_term);
    let results = conn.query_map(query, |row| SearchResult::from_row(row).expect("unable to parse search results"))?;
    Ok(SearchResults(results))
}

/// Creates an SQL connection then iteratively performs searches for each provided term
pub fn search(db_name: &str, search_terms: &Vec<String>) -> anyhow::Result<SearchResults> {
    let opts = get_mysql_options(db_name);
    let mut conn = Conn::new(opts)?;
    let mut results = Vec::new();
    for term in search_terms.iter() {
        let mut term_results = search_term(&mut conn, term)?;
        results.append(&mut term_results.0 );
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
