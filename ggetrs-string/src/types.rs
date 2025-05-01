use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug, Copy)]
pub enum StringNetworkType {
    /// Retrieves a network of physical interactions
    Physical,

    /// Retrieves a network of functional interactions
    Functional,
}
impl std::fmt::Display for StringNetworkType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Physical => write!(f, "physical"),
            Self::Functional => write!(f, "functional"),
        }
    }
}
