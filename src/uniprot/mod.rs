/// Module for functions
pub mod functions;

/// Module for handling result structs and containers
pub mod types;

/// Submodule for cli
mod submodule;

pub use functions::query;
pub use submodule::launch_uniprot_query;
pub use types::{UniprotInfo, UniprotInfoContainer};
