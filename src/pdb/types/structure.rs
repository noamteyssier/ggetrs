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

#[cfg(test)]
mod testing {
    use super::PdbFormat;
    use clap::ValueEnum;

    fn validate_enum(resource: PdbFormat, _expected: PdbFormat) {
        assert!(matches!(resource, _expected))
    }

    #[test]
    fn pdb_structure_repr() {
        assert_eq!(PdbFormat::Pdb.to_string(), String::from("pdb"));
        assert_eq!(PdbFormat::Cif.to_string(), String::from("cif"));
    }

    #[test]
    fn pdb_structure_from_pdb() {
        let examples = vec!["pdb", "PDB", "pDb"];
        let expected = PdbFormat::Pdb;
        for s in examples {
            validate_enum(PdbFormat::from_str(s, true).unwrap(), expected.clone());
        }
    }

    #[test]
    fn pdb_structure_from_cif() {
        let examples = vec!["cif", "CIF", "cIf"];
        let expected = PdbFormat::Cif;
        for s in examples {
            validate_enum(PdbFormat::from_str(s, true).unwrap(), expected.clone());
        }
    }
}
