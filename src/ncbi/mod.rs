/// Module for handling result structs and containers
pub mod structs;

/// Module for querying NCBI
mod query;

/// Module for CLI
mod submodule;

pub use query::{query_ncbi_ids, query_ncbi_symbols};
pub use submodule::{launch_query_ncbi_ids, launch_query_ncbi_symbols};
