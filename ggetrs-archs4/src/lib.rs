/// Module for functions
pub mod functions;

/// Module for handling result structs and containers
pub mod types;

/// module for launching commands
mod launch;

/// module for python interface
#[cfg(feature = "python")]
mod python;

pub use functions::{correlation, tissue};
pub use launch::{launch_archs4_correlation, launch_archs4_tissue};
pub use types::Species;

#[cfg(feature = "python")]
pub use python::python_archs4;
