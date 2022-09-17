use anyhow::{Result, bail};
use reqwest::blocking::Client;
use serde_json::Value;
use crate::ucsc::types::{SeqType, BlatResults};

/// Queries an input sequence for location within the genome
pub fn blat(sequence: &str, seqtype: &SeqType, db_name: &str) -> Result<BlatResults> {
    if sequence.len() < 20 {
        bail!("Input sequence must be greater than 20 nucleotides/residues")
    }
    let url = format!(
        "https://genome.ucsc.edu/cgi-bin/hgBlat?userSeq={}&type={}&db={}&output=json",
        sequence,
        seqtype,
        db_name 
        );
    let response = Client::new()
        .get(url)
        .send()?
        .json::<Value>()?;
    let br = BlatResults::from_value(&response);
    Ok(br)
}
