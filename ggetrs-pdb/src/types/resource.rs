use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum PdbResource {
    Entry,
    Pubmed,
    Assembly,
    BranchedEntity,
    NonpolymerEntity,
    PolymerEntity,
    Uniprot,
    BranchedEntityInstance,
    PolymerEntityInstance,
    NonpolymerEntityInstance,
}
impl fmt::Display for PdbResource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match self {
            Self::Entry => "entry",
            Self::Pubmed => "pubmed",
            Self::Assembly => "assembly",
            Self::BranchedEntity => "branched_entity",
            Self::NonpolymerEntity => "nonpolymer_entity",
            Self::PolymerEntity => "polymer_entity",
            Self::Uniprot => "uniprot",
            Self::BranchedEntityInstance => "branched_entity_instance",
            Self::PolymerEntityInstance => "polymer_entity_instance",
            Self::NonpolymerEntityInstance => "nonpolymer_entity_instance",
        };
        write!(f, "{repr}")
    }
}
impl FromStr for PdbResource {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "entry" => Ok(Self::Entry),
            "pubmed" => Ok(Self::Pubmed),
            "assembly" => Ok(Self::Assembly),
            "branched_entity" => Ok(Self::BranchedEntity),
            "nonpolymer_entity" => Ok(Self::NonpolymerEntity),
            "polymer_entity" => Ok(Self::PolymerEntity),
            "uniprot" => Ok(Self::Uniprot),
            "branched_entity_instance" => Ok(Self::BranchedEntityInstance),
            "polymer_entity_instance" => Ok(Self::PolymerEntityInstance),
            "nonpolymer_entity_instance" => Ok(Self::NonpolymerEntityInstance),
            _ => Err(format!("Invalid PDB resource: {}", s)),
        }
    }
}
impl PdbResource {
    #[must_use]
    pub fn requires_entity_id(&self) -> bool {
        matches!(
            self,
            Self::BranchedEntityInstance
                | Self::NonpolymerEntityInstance
                | Self::PolymerEntityInstance
                | Self::Uniprot
        )
    }
    #[must_use]
    pub fn requires_chain_id(&self) -> bool {
        matches!(
            self,
            Self::BranchedEntityInstance
                | Self::NonpolymerEntityInstance
                | Self::PolymerEntityInstance
        )
    }
    #[must_use]
    pub fn requires_assembly_id(&self) -> bool {
        matches!(self, Self::Assembly)
    }
    #[must_use]
    pub fn requires_identifier(&self) -> bool {
        matches!(self, Self::Assembly)
    }
}

#[cfg(test)]
mod testing {
    use super::*;

    fn validate_enum(resource: PdbResource, _expected: PdbResource) {
        assert!(matches!(resource, _expected));
    }

    #[test]
    fn pdb_resource_from_entry() {
        let examples = vec!["entry", "Entry", "ENTRY", "EnTrY"];
        let expected = PdbResource::Entry;
        for s in examples {
            validate_enum(PdbResource::from_str(s).unwrap(), expected);
        }
    }

    #[test]
    fn pdb_resource_from_pubmed() {
        let examples = vec!["pubmed", "Pubmed", "PUBMED", "pUbMeD"];
        let expected = PdbResource::Pubmed;
        for s in examples {
            validate_enum(PdbResource::from_str(s).unwrap(), expected);
        }
    }

    #[test]
    fn pdb_resource_from_assembly() {
        let examples = vec!["assembly", "Assembly", "ASSEMBLY", "aSsEmBlY"];
        let expected = PdbResource::Assembly;
        for s in examples {
            validate_enum(PdbResource::from_str(s).unwrap(), expected);
        }
    }
}
