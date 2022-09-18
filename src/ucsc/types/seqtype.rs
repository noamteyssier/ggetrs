use clap::ValueEnum;
use std::fmt;

/// The available sequence types for BLAT
#[derive(ValueEnum, Clone, Debug)]
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
        write!(f, "{}", repr)
    }
}
