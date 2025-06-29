use std::fs::File;
use std::io::Write;

use anyhow::Result;

use super::info;

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
                writeln!(writer, "{results}").expect("Unable to write to file");
            } else {
                println!("{results}");
            }
        }
        None => {
            println!("{results}");
        }
    }
    Ok(())
}
