use anyhow::{bail, Result};
use clap::clap_derive::ArgEnum;

#[derive(Debug, Default, ArgEnum, Clone, Copy)]
pub enum BlastProgram {
    #[default]
    Blastn,
    Blastp,
    Blastx,
    Tblastn,
    Tblastx,
}
impl BlastProgram {
    pub fn to_string(&self) -> String {
        match &self {
            Self::Blastn => "blastn",
            Self::Blastp => "blastp",
            Self::Blastx => "blastx",
            Self::Tblastn => "tblastn",
            Self::Tblastx => "tblastx",
        }.to_string()
    }
    pub fn from_sequence(sequence: &str) -> Result<Self> {
        let sequence = sequence.to_uppercase();
        let mut known_characters = sequence
            .chars()
            .map(|c| match c {
                'A'|'C'|'G'|'T'|'R'|'N'|'D'|'Q'|'E'|'H'|'I'|'L'|'K'|'M'|'F'|'P'|'S'|'W'|'Y'|'V'|'B'|'Z' => Ok(c),
                _ => bail!("Unexpected character: {c}"),
            });

        let mut is_nucl = true;
        while let Some(potential_c) = known_characters.next() {
            match potential_c {
                Ok(c) => {
                    match c {
                        'A' | 'C' | 'G' | 'T' => {continue},
                        _ => {
                            is_nucl = false;
                            break;
                        }
                    }
                },
                Err(e) => return Err(e)
            }
        }
        if is_nucl {
            Ok(Self::Blastn)
        } else {
            Ok(Self::Blastp)
        }
    }
}

#[derive(Debug, Default, ArgEnum, Clone, Copy)]
pub enum BlastDatabase {
    #[default]
    Nt,
    Nr,
    RefseqRna,
    RefseqProtein,
    Swissprot,
    Pdbaa,
    Pdbnt,
}
impl BlastDatabase {
    pub fn to_string(&self) -> String {
        match &self {
            Self::Nt => "nt",
            Self::Nr => "nr",
            Self::RefseqRna=> "refseq_rna",
            Self::RefseqProtein => "refseq_protein",
            Self::Swissprot => "swissprot",
            Self::Pdbaa => "pdbaa",
            Self::Pdbnt => "pdbnt",
        }.to_string()
    }
    pub fn from_program(program: &BlastProgram) -> Self {
        match program {
            BlastProgram::Blastn => Self::Nt,
            BlastProgram::Blastp => Self::Nr,
            _ => Self::Nt,
        }
    }
}
