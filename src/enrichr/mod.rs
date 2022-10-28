/// Module for functions
pub mod functions;

/// Module for handling result structs and containers
pub mod types;

/// submodule for command line usage
mod cli;

/// submodule for python usage
mod python;

pub use cli::{launch_enrichr, launch_enrichr_list};
pub use functions::{add_list, enrich, get_libraries};
pub use python::python_enrichr;
