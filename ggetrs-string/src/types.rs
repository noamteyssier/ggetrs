use std::str::FromStr;

#[derive(Clone, Debug, Copy)]
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
impl FromStr for StringNetworkType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "physical" => Ok(Self::Physical),
            "functional" => Ok(Self::Functional),
            _ => Err(format!("Invalid network type: {}", s)),
        }
    }
}
