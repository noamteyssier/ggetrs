/// Module for functions
pub mod functions;

/// Module for handling result structs and containers
pub mod types;

/// Module for CLI
mod cli;

pub use cli::{launch_ncbi_query_ids, launch_ncbi_query_symbols, launch_ncbi_taxons};
pub use functions::{query_ids, query_symbols};
