use std::str::FromStr;

/// The different data types present within the Ensembl FTP
#[derive(Clone, Debug)]
pub enum DataType {
    CDNA,
    CDS,
    DNA,
    GFF3,
    GTF,
    NCRNA,
    PEP,
}
impl FromStr for DataType {
    type Err = anyhow::Error;

    fn from_str(datatype: &str) -> Result<Self, Self::Err> {
        match datatype.to_lowercase().as_str() {
            "cdna" => Ok(Self::CDNA),
            "cds" => Ok(Self::CDS),
            "dna" => Ok(Self::DNA),
            "gff3" => Ok(Self::GFF3),
            "gtf" => Ok(Self::GTF),
            "ncrna" => Ok(Self::NCRNA),
            "pep" => Ok(Self::PEP),
            _ => Err(anyhow::anyhow!("invalid datatype")),
        }
    }
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
            Self::CDNA => [".cdna.all.fa"].iter().map(|x| (*x).to_string()).collect(),
            Self::CDS => [".cds.all.fa"].iter().map(|x| (*x).to_string()).collect(),
            Self::DNA => [".dna.primary_assembly.fa", ".dna.toplevel.fa"]
                .iter()
                .map(|x| (*x).to_string())
                .collect(),
            Self::GFF3 => vec![format!("{}.gff3.gz", release)],
            Self::GTF => vec![format!("{}.gtf.gz", release)],
            Self::NCRNA => [".ncrna.fa"].iter().map(|x| (*x).to_string()).collect(),
            Self::PEP => [".pep.all.fa"].iter().map(|x| (*x).to_string()).collect(),
        }
    }
}
