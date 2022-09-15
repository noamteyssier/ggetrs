use clap::ValueEnum;

/// The different data types present within the Ensembl FTP
#[derive(ValueEnum, Clone, Debug)]
pub enum DataType {
    CDNA,
    CDS,
    DNA,
    GFF3,
    GTF,
    NCRNA,
    PEP,
}
impl DataType {
    #[must_use]
    pub fn directory(&self) -> &str {
        match self {
            Self::GTF => "gtf",
            Self::GFF3 => "gff3",
            _ => "fasta",
        }
    }

    #[must_use]
    pub fn subdirectory(&self) -> Option<&str> {
        match self {
            Self::CDNA => Some("cdna"),
            Self::CDS => Some("cds"),
            Self::DNA => Some("dna"),
            Self::NCRNA => Some("ncrna"),
            Self::PEP => Some("pep"),
            _ => None,
        }
    }

    #[must_use]
    pub fn expected_substring(&self, release: usize) -> Vec<String> {
        match self {
            Self::CDNA => vec![".cdna.all.fa"]
                .iter()
                .map(|x| (*x).to_string())
                .collect(),
            Self::CDS => vec![".cds.all.fa"]
                .iter()
                .map(|x| (*x).to_string())
                .collect(),
            Self::DNA => vec![".dna.primary_assembly.fa", ".dna.toplevel.fa"]
                .iter()
                .map(|x| (*x).to_string())
                .collect(),
            Self::GFF3 => vec![format!("{}.gff3.gz", release)],
            Self::GTF => vec![format!("{}.gtf.gz", release)],
            Self::NCRNA => vec![".ncrna.fa"].iter().map(|x| (*x).to_string()).collect(),
            Self::PEP => vec![".pep.all.fa"]
                .iter()
                .map(|x| (*x).to_string())
                .collect(),
        }
    }
}
