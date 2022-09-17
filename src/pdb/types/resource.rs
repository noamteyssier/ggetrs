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
            Self::BranchedEntity | Self::NonpolymerEntity | Self::PolymerEntity | Self::Uniprot => true,
            _ => false
        }
    }
    pub fn requires_chain_id(&self) -> bool {
        match self {
            Self::BranchedEntityInstance | Self::NonpolymerEntityInstance | Self::PolymerEntityInstance => true,
            _ => false
        }
    }
    pub fn requires_assembly_id(&self) -> bool {
        match self {
            Self::Assembly => true,
            _ => false
        }
    }
    pub fn requires_identifier(&self) -> bool {
        match self {
            Self::Entry | Self::Pubmed => false,
            _ => true
        }
    }
}

