use super::{
    functions::{resource_info, structure},
    types::{PdbFormat, PdbResource},
};
use anyhow::Result;
use std::{fs::File, io::Write};

/// main entrypoint for pdb structure
pub fn launch_pdb_structure(
    pdb_id: &str,
    header_only: bool,
    format: &PdbFormat,
    output: &Option<String>,
) -> Result<()> {
    let results = structure(pdb_id, header_only, format)?;
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

/// main entrypoint for pdb resource info
pub fn launch_pdb_resource(
    pdb_id: &str,
    resource: &PdbResource,
    identifier: &Option<String>,
    output: &Option<String>,
) -> Result<()> {
    let results = resource_info(pdb_id, resource, identifier)?;
    let repr = serde_json::to_string_pretty(&results)?;
    match output {
        Some(path) => {
            if let Ok(mut writer) = File::create(path) {
                write!(writer, "{}", repr)?;
            } else {
                print!("{}", repr)
            }
        }
        None => {
            print!("{}", repr)
        }
    }
    Ok(())
}
