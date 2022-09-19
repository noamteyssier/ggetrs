use crate::ensembl::types::{Database, ResponseDatabases};
use anyhow::bail;
use mysql::{prelude::Queryable, Conn, OptsBuilder};

/// Queries all available databases on Ensembl SQL server
pub fn database(filter: &Option<String>) -> anyhow::Result<ResponseDatabases> {
    let opts = get_mysql_options();
    let mut conn = Conn::new(opts)?;
    let query = build_search_query(filter);
    let results: Vec<Database> = conn.query_map(query, Database)?;
    if results.len() > 0 {
        Ok(ResponseDatabases(results))
    } else {
        match filter {
            Some(f) => bail!(format!("No databases found with filter: {}", f)),
            None => bail!("No databases found"),
        }
    }
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

#[cfg(test)]
mod testing {
    use super::database;

    #[test]
    fn test_ensembl_database_full() {
        let filter = None;
        let response = database(&filter);
        assert!(response.is_ok())
    }

    #[test]
    fn test_ensembl_database_specific() {
        let filter = Some("homo_sapiens_core_107_38".to_string());
        let response = database(&filter);
        assert!(response.is_ok())
    }

    #[test]
    fn test_ensembl_database_nonsense() {
        let filter = Some("AIWJDIOANDNIWND".to_string());
        let response = database(&filter);
        assert!(response.is_err())
    }
}
