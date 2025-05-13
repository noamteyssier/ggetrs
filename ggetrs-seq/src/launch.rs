use std::fs::File;
use std::io::Write;

use anyhow::Result;

use super::sequence;
use ggetrs_uniprot::functions::query;

pub fn launch_seq(
    search_terms: &Vec<String>,
    translate: bool,
    species: &Option<String>,
    output: &Option<String>,
) -> Result<()> {
    if translate {
        let results = query(search_terms, false, &None)?;
        match output {
            Some(path) => {
                if let Ok(mut writer) = File::create(path) {
                    write!(writer, "{}", results.to_fasta()).expect("Unable to write to file");
                } else {
                    print!("{}", results.to_fasta());
                }
            }
            None => {
                print!("{}", results.to_fasta());
            }
        }
    } else {
        let results = sequence(search_terms, species)?;
        match output {
            Some(path) => {
                if let Ok(mut writer) = File::create(path) {
                    write!(writer, "{}", results.to_fasta()).expect("Unable to write to file");
                } else {
                    print!("{}", results.to_fasta());
                }
            }
            None => {
                print!("{}", results.to_fasta());
            }
        }
    }
    Ok(())
}
