use crate::ucsc::types::{BlatResults, SeqType};
use anyhow::{bail, Result};
use reqwest::blocking::Client;
use serde_json::Value;

/// Queries an input sequence for location within the genome
pub fn blat(sequence: &str, seqtype: &SeqType, db_name: &str) -> Result<BlatResults> {
    if sequence.len() < 20 {
        bail!("Input sequence must be greater than 20 nucleotides/residues")
    }
    let url = format!(
        "https://genome.ucsc.edu/cgi-bin/hgBlat?userSeq={}&type={}&db={}&output=json",
        sequence, seqtype, db_name
    );
    let response = Client::new().get(url).send()?.json::<Value>()?;
    let br = BlatResults::from_value(&response);
    Ok(br)
}

#[cfg(test)]
mod testing {
    use super::blat;
    use crate::ucsc::types::SeqType;

    #[test]
    fn test_blat_short_seq() {
        let sequence = "ACTG";
        let seqtype = SeqType::Dna;
        let db_name = "hg38";
        let response = blat(sequence, &seqtype, db_name);
        assert!(response.is_err());
    }

    #[test]
    fn test_blat() {
        let sequence = "AGTGGTACATGCAGTTTGATGATGATGAGAAACAGAAGCT";
        let seqtype = SeqType::Dna;
        let db_name = "hg38";
        let response = blat(sequence, &seqtype, db_name).unwrap();
        assert_eq!(response.0[0].matches, 40);
        assert_eq!(response.0.len(), 3);
    }

    #[test]
    fn test_blat_protein() {
        let sequence = "MIRFILIQNRAGKTRLAKWYMQFDDDEKQK";
        let seqtype = SeqType::Protein;
        let db_name = "hg38";
        let response = blat(sequence, &seqtype, db_name).unwrap();
        assert_eq!(response.0[0].matches, 29);
        assert_eq!(response.0.len(), 1);
    }

    #[test]
    fn test_blat_missing_db() {
        let sequence = "AGTGGTACATGCAGTTTGATGATGATGAGAAACAGAAGCT";
        let seqtype = SeqType::Dna;
        let db_name = "blahblahblah";
        let response = blat(sequence, &seqtype, db_name).unwrap();
        assert_eq!(response.0[0].matches, 40);
        assert_eq!(response.0.len(), 3);
    }

    #[test]
    fn test_blat_wrong_seqtype() {
        let sequence = "MIRFILIQNRAGKTRLAKWYMQFDDDEKQK";
        let seqtype = SeqType::Dna;
        let db_name = "hg38";
        let response = blat(sequence, &seqtype, db_name);
        assert!(response.is_err())
    }
}
