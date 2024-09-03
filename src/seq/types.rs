use serde::{Deserialize, Serialize};

use crate::utils::{FastaRecord, FastaRecords};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResultSeqContainer(pub Vec<ResultSeq>);
impl ResultSeqContainer {
    #[must_use]
    pub fn to_fasta(&self) -> String {
        self.0
            .iter()
            .map(ResultSeq::to_fasta)
            .fold(String::new(), |mut acc, x| {
                acc.push_str(&x);
                acc
            })
    }
    #[must_use]
    pub fn fasta_records(&self) -> FastaRecords {
        let records = self.0.iter().map(ResultSeq::as_fasta).collect();
        FastaRecords(records)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResultSeq {
    pub id: String,
    pub desc: String,
    pub seq: String,
}

impl ResultSeq {
    #[must_use]
    pub fn to_fasta(&self) -> String {
        format!("{}", self.as_fasta())
    }
    #[must_use]
    pub fn as_fasta(&self) -> FastaRecord {
        FastaRecord::new(&format!("{} {}", self.id, self.desc), &self.seq)
    }
}
