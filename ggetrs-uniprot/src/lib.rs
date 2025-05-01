/// Module for functions
pub mod functions;

/// Module for handling result structs and containers
pub mod types;

/// Submodule for cli
mod cli;

/// Submodule for utilities
pub mod utils;

pub use cli::launch_uniprot_query;
pub use functions::{async_query_uniprot, async_query_uniprot_multiple, query};
pub use types::{UniprotInfo, UniprotInfoContainer};
