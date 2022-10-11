use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResultSeqContainer(pub Vec<ResultSeq>);

impl ResultSeqContainer{
    pub fn to_fasta(&self) -> String{
        self.0
        .iter()
        .map(|x| x.to_fasta())
        .fold(String::new(), |acc, x| acc + &x)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResultSeq {
    pub seq: String,
    pub desc: String,
    pub id: String,
}

impl ResultSeq{
    pub fn to_fasta(&self) -> String {
        format!(">{} {}\n{}\n", self.id, self.desc, self.seq)
    }
}