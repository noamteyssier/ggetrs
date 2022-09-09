use reqwest::Result;
use std::{io::Write, fs::File};

use super::correlation;

/// Main entrypoint to launching the `correlation` function for `ARCHS4`
pub fn launch_archs4_correlation(gene_name: &str, count: usize, output: &Option<String>) -> Result<()> {
    let results = correlation(gene_name, count)?;
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
