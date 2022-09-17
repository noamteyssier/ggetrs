use anyhow::Result;
use super::functions::structure;
use std::{fs::File, io::Write};

/// main entrypoint for pdb structure
pub fn launch_pdb_structure(pdb_id: &str, header_only: bool, output: &Option<String>) -> Result<()> {
    let results = structure(pdb_id, header_only)?;
    if let Some(pdb_text) = results {
        match output {
            Some(path) => {
                if let Ok(mut writer) = File::create(path) {
                    write!(writer, "{}", pdb_text).expect("Unable to write to file");
                } else {
                    print!("{}", pdb_text);
                }
            }
            None => {
                print!("{}", pdb_text);
            }
        }
    } else {
        eprintln!("No PDB record found: {}", pdb_id);
    }
    Ok(())
}
