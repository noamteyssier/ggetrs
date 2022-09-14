use anyhow::Result;
use std::{io::Write, fs::File};
use super::query;

pub fn launch_uniprot_query(search_terms: &Vec<String>, taxon: &Option<usize>, output: &Option<String>) -> Result<()> {
    let results = query(search_terms, taxon)?;
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
