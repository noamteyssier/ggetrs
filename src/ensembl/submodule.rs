use super::search;
use std::{io::Write, fs::File};

pub fn launch_ensembl_search(
        search_terms: &Vec<String>, 
        species: &str, 
        db_type: &str, 
        release: &usize, 
        assembly: &str,
        output: &Option<String>) -> anyhow::Result<()> 
{
    let db_name = format!("{}_{}_{}_{}", species, db_type, release, assembly);
    let results = search(&db_name, search_terms)?;
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
