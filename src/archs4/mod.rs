/// Module for functions
pub mod functions;

/// Module for handling result structs and containers
pub mod types;

/// module for command line interface
mod submodule;

/// module for python interface
mod python;

pub use functions::{correlation, tissue};
pub use types::Species;
pub use python::python_archs4;
pub use submodule::{launch_archs4_correlation, launch_archs4_tissue};
