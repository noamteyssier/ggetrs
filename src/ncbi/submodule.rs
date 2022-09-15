use super::{query_ncbi_ids, query_ncbi_symbols};
use anyhow::Result;
use std::{fs::File, io::Write};

pub fn launch_query_ncbi_ids(ids: &[usize], output: &Option<String>) -> Result<()> {
    let results = query_ncbi_ids(ids)?;
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

pub fn launch_query_ncbi_symbols(
    symbols: &[String],
    taxon_id: usize,
    output: &Option<String>,
) -> Result<()> {
    let results = query_ncbi_symbols(symbols, taxon_id)?;
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
