use std::fmt;
use std::str::FromStr;

/// The available sequence types for BLAT
#[derive(Clone, Debug)]
pub enum SeqType {
    Dna,
    Protein,
    TranslatedRna,
    TranslatedDna,
}
impl fmt::Display for SeqType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match self {
            Self::Dna => "dna",
            Self::Protein => "protein",
            Self::TranslatedDna => "translated%20dna",
            Self::TranslatedRna => "translated%20rna",
        };
        write!(f, "{repr}")
    }
}
impl FromStr for SeqType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "dna" => Ok(Self::Dna),
            "protein" => Ok(Self::Protein),
            "tdna" => Ok(Self::TranslatedDna),
            "trna" => Ok(Self::TranslatedRna),
            _ => Err(format!("Invalid sequence type: {}", s)),
        }
    }
}
