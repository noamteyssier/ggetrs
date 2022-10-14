use anyhow::Result;
use super::sequence;
use crate::uniprot::functions::query;
use std::{fs::File, io::Write};

pub fn launch_seq(ensembl_ids: &Vec<String>, &transcribe: &bool, output: &Option<String>) -> Result<()> {

    if transcribe {
        let results = query(ensembl_ids, &None)?;
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
    else {
        let results = sequence(ensembl_ids)?;
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