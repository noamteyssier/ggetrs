use super::query;
use anyhow::Result;
use std::{fs::File, io::Write};

pub fn launch_uniprot_query(
    search_terms: &[String],
    freeform: bool,
    taxon: &Option<usize>,
    output: &Option<String>,
) -> Result<()> {
    let results = query(search_terms, freeform, taxon)?;
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
