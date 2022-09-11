use mysql::{Conn, OptsBuilder, Row, prelude::Queryable};
use anyhow::bail;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize ,Debug)]
pub struct SearchResults ( Vec<SearchResult> );
impl fmt::Display for SearchResults {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).expect("cannot serialize"))
    }
}

#[derive(Serialize, Deserialize ,Debug)]
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
}

fn search_term(conn: &mut Conn, search_term: &str) -> anyhow::Result<SearchResults> {
    let query = build_search_query(search_term);
    let results = conn.query_map(query, |row| SearchResult::from_row(row).expect("unable to parse search results"))?;
    Ok(SearchResults(results))
}

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

fn get_mysql_options(db_name: &str) -> OptsBuilder {
    OptsBuilder::new()
        .ip_or_hostname(Some("ensembldb.ensembl.org"))
        .tcp_port(3306)
        .user(Some("anonymous"))
        .db_name(Some(db_name))
}

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

