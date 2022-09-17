use super::{functions::taxons, query_ids, query_symbols};
use anyhow::Result;
use std::{fs::File, io::Write};

pub fn launch_ncbi_taxons(query: &str, limit: usize, output: &Option<String>) -> Result<()> {
    let results = taxons(query, limit)?;
    match output {
        Some(path) => {
            if let Ok(mut writer) = File::create(path) {
                writeln!(writer, "{}", results).expect("Unable to write to file");
            } else {
                println!("{}", results);
            }
        }
        None => {
            println!("{}", results);
        }
    }
    Ok(())
}

pub fn launch_ncbi_query_ids(ids: &[usize], output: &Option<String>) -> Result<()> {
    let results = query_ids(ids)?;
    let repr = serde_json::to_string_pretty(&results)?;
    match output {
        Some(path) => {
            if let Ok(mut writer) = File::create(path) {
                writeln!(writer, "{}", repr).expect("Unable to write to file");
            } else {
                println!("{}", repr);
            }
        }
        None => {
            println!("{}", repr);
        }
    }
    Ok(())
}

pub fn launch_ncbi_query_symbols(
    symbols: &[String],
    taxon_id: usize,
    output: &Option<String>,
) -> Result<()> {
    let results = query_symbols(symbols, taxon_id)?;
    let repr = serde_json::to_string_pretty(&results)?;
    match output {
        Some(path) => {
            if let Ok(mut writer) = File::create(path) {
                writeln!(writer, "{}", repr).expect("Unable to write to file");
            } else {
                println!("{}", repr);
            }
        }
        None => {
            println!("{}", repr);
        }
    }
    Ok(())
}
