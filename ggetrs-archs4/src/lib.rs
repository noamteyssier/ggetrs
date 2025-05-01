/// Module for functions
pub mod functions;

/// Module for handling result structs and containers
pub mod types;

/// module for command line interface
mod cli;

/// module for python interface
mod python;

pub use cli::{launch_archs4_correlation, launch_archs4_tissue};
pub use functions::{correlation, tissue};
pub use python::python_archs4;
pub use types::Species;
