use mysql::{OptsBuilder, Conn, prelude::Queryable};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseDatabases(Vec<Database>);
impl fmt::Display for ResponseDatabases {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).expect("cannot serialize"))
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Database(String);
impl fmt::Display for Database {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).expect("cannot serialize"))
    }
}

/// Queries all available databases on Ensembl SQL server
pub fn database(filter: &Option<String>) -> anyhow::Result<ResponseDatabases> {
    let opts = get_mysql_options();
    let mut conn = Conn::new(opts)?;
    let query = build_search_query(filter);
    let results: Vec<Database> = conn.query_map(query, |x: String| Database(x))?;
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
        format!(
            "SHOW databases LIKE '%{}%'", token
        )
    } else {
        String::from("SHOW databases")
    }
}
