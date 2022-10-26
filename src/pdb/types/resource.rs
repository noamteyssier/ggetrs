use clap::ValueEnum;
use std::fmt;

#[derive(ValueEnum, Debug, Clone)]
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
        write!(f, "{}", repr)
    }
}
impl PdbResource {
    pub fn requires_entity_id(&self) -> bool {
        match self {
            Self::BranchedEntity | Self::NonpolymerEntity | Self::PolymerEntity | Self::Uniprot => {
                true
            }
            _ => false,
        }
    }
    pub fn requires_chain_id(&self) -> bool {
        match self {
            Self::BranchedEntityInstance
            | Self::NonpolymerEntityInstance
            | Self::PolymerEntityInstance => true,
            _ => false,
        }
    }
    pub fn requires_assembly_id(&self) -> bool {
        match self {
            Self::Assembly => true,
            _ => false,
        }
    }
    pub fn requires_identifier(&self) -> bool {
        match self {
            Self::Entry | Self::Pubmed => false,
            _ => true,
        }
    }
}

#[cfg(test)]
mod testing {
    use super::PdbResource;
    use clap::ValueEnum;

    fn validate_enum(resource: PdbResource, _expected: PdbResource) {
        assert!(matches!(resource, _expected))
    }

    #[test]
    fn pdb_resource_from_entry() {
        let examples = vec!["entry", "Entry", "ENTRY", "EnTrY"];
        let expected = PdbResource::Entry;
        for s in examples {
            validate_enum(PdbResource::from_str(s, true).unwrap(), expected.clone());
        }
    }

    #[test]
    fn pdb_resource_from_pubmed() {
        let examples = vec!["pubmed", "Pubmed", "PUBMED", "pUbMeD"];
        let expected = PdbResource::Pubmed;
        for s in examples {
            validate_enum(PdbResource::from_str(s, true).unwrap(), expected.clone());
        }
    }

    #[test]
    fn pdb_resource_from_assembly() {
        let examples = vec!["assembly", "Assembly", "ASSEMBLY", "aSsEmBlY"];
        let expected = PdbResource::Assembly;
        for s in examples {
            validate_enum(PdbResource::from_str(s, true).unwrap(), expected.clone());
        }
    }
}
