use mysql::{prelude::Queryable, Conn, OptsBuilder};
use crate::ensembl::types::{ResponseDatabases, Database};

/// Queries all available databases on Ensembl SQL server
pub fn database(filter: &Option<String>) -> anyhow::Result<ResponseDatabases> {
    let opts = get_mysql_options();
    let mut conn = Conn::new(opts)?;
    let query = build_search_query(filter);
    let results: Vec<Database> = conn.query_map(query, Database)?;
    Ok(ResponseDatabases(results))
}

/// Generates mysql options
fn get_mysql_options() -> OptsBuilder {
    OptsBuilder::new()
        .ip_or_hostname(Some("ensembldb.ensembl.org"))
        .tcp_port(3306)
        .user(Some("anonymous"))
}

/// Generates the search query.
///
/// Searches through databases for a related token
fn build_search_query(search_term: &Option<String>) -> String {
    if let Some(token) = search_term {
        format!("SHOW databases LIKE '%{}%'", token)
    } else {
        String::from("SHOW databases")
    }
}

