/// Module for functions
pub mod functions;

/// Module for handling result structs and containers
pub mod types;

/// Module for utilities
mod utils;

/// Module for launching functions
mod launch;

pub use functions::{query_ids, query_symbols};
pub use launch::{launch_ncbi_query_ids, launch_ncbi_query_symbols, launch_ncbi_taxons};
