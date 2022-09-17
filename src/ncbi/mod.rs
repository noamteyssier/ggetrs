/// Module for functions
pub mod functions;

/// Module for handling result structs and containers
pub mod types;

/// Module for CLI
mod submodule;

pub use functions::{query_ids, query_symbols};
pub use submodule::{launch_ncbi_query_ids, launch_ncbi_query_symbols, launch_ncbi_taxons};
