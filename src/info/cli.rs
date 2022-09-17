use super::info;
use anyhow::Result;
use std::{fs::File, io::Write};

/// Main entrypoint for info query across multiple databases
pub fn launch_info(
    search_terms: &[String],
    species: &str,
    taxon_id: usize,
    output: &Option<String>,
) -> Result<()> {
    let results = info(search_terms, species, taxon_id)?;
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
