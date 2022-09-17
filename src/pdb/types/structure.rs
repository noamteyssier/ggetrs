use clap::ValueEnum;
use std::fmt;

#[derive(ValueEnum, Debug, Clone)]
pub enum PdbFormat {
    Pdb,
    Cif,
}
impl fmt::Display for PdbFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match self {
            Self::Pdb => "pdb",
            Self::Cif => "cif",
        };
        write!(f, "{}", repr)
    }
}
