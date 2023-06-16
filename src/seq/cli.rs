use super::sequence;
use crate::uniprot::functions::query;
use anyhow::Result;
use std::{fs::File, io::Write};

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
