use super::{correlation, tissue, Species};
use anyhow::Result;
use std::{fs::File, io::Write};

/// Main entrypoint to launching the `correlation` function for `ARCHS4`
pub fn launch_archs4_correlation(
    gene_name: &str,
    count: usize,
    output: &Option<String>,
) -> Result<()> {
    let results = correlation(gene_name, count)?;
    match output {
        Some(path) => {
            if let Ok(mut writer) = File::create(path) {
                writeln!(writer, "{}", results)?;
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

/// Main entrypoint to launching the `tissue` function for `ARCHS4`
pub fn launch_archs4_tissue(
    gene_name: &str,
    species: &Species,
    output: &Option<String>,
) -> Result<()> {
    let results = tissue(gene_name, species)?;
    match output {
        Some(path) => {
            if let Ok(mut writer) = File::create(path) {
                writeln!(writer, "{}", results)?;
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
