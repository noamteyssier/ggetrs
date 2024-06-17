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
        "https://genome.ucsc.edu/cgi-bin/hgBlat?userSeq={sequence}&type={seqtype}&db={db_name}&output=json",
    );
    let response = Client::new().get(url).send()?;

    let response_json: Value = match response.json() {
        Ok(json) => json,
        Err(_) => {
            bail!(
                "Bad response from UCSC Genome Browser. Check Database Name: {}",
                db_name
            );
        }
    };

    let br = BlatResults::from_value(&response_json);
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
        let seqtype = SeqType::Dna; // Corrected to match the enum variant
        let db_name = "blahblahblah";
        let response = blat(sequence, &seqtype, db_name);

        // Ensure that the response is an error
        assert!(response.is_err());

        // Check that the error message matches the expected message
        if let Err(e) = response {
            assert_eq!(
                e.to_string(),
                "Bad response from UCSC Genome Browser. Check Database Name: blahblahblah"
            );
        }
    }

    #[test]
    fn test_blat_wrong_seqtype() {
        let sequence = "MIRFILIQNRAGKTRLAKWYMQFDDDEKQK";
        let seqtype = SeqType::Dna;
        let db_name = "hg38";
        let response = blat(sequence, &seqtype, db_name);
        assert!(response.is_err());
    }
}
