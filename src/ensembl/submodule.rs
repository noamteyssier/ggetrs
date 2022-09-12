use super::{search, database};
use std::{io::Write, fs::File};

/// Main entrypoint for `Ensembl` description search
pub fn launch_ensembl_search(
        search_terms: &Vec<String>, 
        database: &Option<String>,
        species: &str, 
        db_type: &str, 
        release: &usize, 
        assembly: &str,
        output: &Option<String>) -> anyhow::Result<()> 
{
    let db_name = match database {
        Some(name) => name.clone(),
        None => format!("{}_{}_{}_{}", species, db_type, release, assembly)
    };
    let results = search(&db_name, search_terms)?;
    match output {
        Some(path) => {
            if let Ok(mut writer) = File::create(path) {
                writeln!(writer, "{}", results).expect("Unable to write to file");
            } else {
                println!("{}", results);
            }
        },
        None => {
            println!("{}", results);
        }
    }
    Ok(())
}

/// Main entrypoint for `Ensembl` SQL database list
pub fn launch_ensembl_database(filter: &Option<String>, output: &Option<String>) -> anyhow::Result<()> {
    let results = database(filter)?;
    match output {
        Some(path) => {
            if let Ok(mut writer) = File::create(path) {
                writeln!(writer, "{}", results).expect("Unable to write to file");
            } else {
                println!("{}", results);
            }
        },
        None => {
            println!("{}", results);
        }
    }
    Ok(())
}
