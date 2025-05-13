/// Module for functions
pub mod functions;

/// Module for handling result structs and containers
pub mod types;

/// submodule for launching functions
mod launch;

/// submodule for python usage
#[cfg(feature = "python")]
mod python;

pub use functions::{add_list, enrich, get_libraries};
pub use launch::{launch_enrichr, launch_enrichr_list};

#[cfg(feature = "python")]
pub use python::{python_enrichr, python_enrichr_background};
