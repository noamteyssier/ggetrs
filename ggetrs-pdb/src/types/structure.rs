use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
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
        write!(f, "{repr}")
    }
}
impl FromStr for PdbFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pdb" => Ok(Self::Pdb),
            "cif" => Ok(Self::Cif),
            _ => Err(format!("Invalid PDB format: {}", s)),
        }
    }
}

#[cfg(test)]
mod testing {
    use super::*;

    fn validate_enum(resource: PdbFormat, _expected: PdbFormat) {
        assert!(matches!(resource, _expected));
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
            validate_enum(PdbFormat::from_str(s).unwrap(), expected);
        }
    }

    #[test]
    fn pdb_structure_from_cif() {
        let examples = vec!["cif", "CIF", "cIf"];
        let expected = PdbFormat::Cif;
        for s in examples {
            validate_enum(PdbFormat::from_str(s).unwrap(), expected);
        }
    }
}
