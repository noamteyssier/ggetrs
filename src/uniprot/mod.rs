/// Module for functions
pub mod functions;

/// Module for handling result structs and containers
pub mod types;

/// Submodule for cli
mod cli;

pub use cli::launch_uniprot_query;
pub use functions::query;
pub use types::{UniprotInfo, UniprotInfoContainer};
