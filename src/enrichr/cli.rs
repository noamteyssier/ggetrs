use std::fs::File;
use std::io::Write;

use super::{add_list, enrich};
use reqwest::Result;

/// Main entrypoint to launching the `enrich` function for `Enrichr`
pub fn launch_enrich(library: &str, gene_list: &[String], output: &Option<String>) -> Result<()> {
    let add_list = add_list(gene_list)?;
    let results = enrich(add_list.user_list_id, library)?;
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
