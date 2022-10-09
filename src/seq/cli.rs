use anyhow::Result;
use super::sequence;
use crate::uniprot::functions::query_seq;

pub fn launch_seq(ensembl_ids: &Vec<String>, &transcribe: &bool) -> Result<()> {

    if transcribe {
        let result = query_seq(ensembl_ids, &None)?;
        println!("{}", result.to_fasta());
    }
    else {
        let result = sequence(ensembl_ids)?;
        println!("{}", result.to_fasta());
    }
    Ok(())
}