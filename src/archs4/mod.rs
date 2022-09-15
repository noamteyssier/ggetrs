mod functions;
mod types;

/// module for querying the tissues where a query gene is expressed
mod tissue;

/// module for command line interface
mod submodule;

/// module for python interface
mod python;

pub use functions::correlation;
pub use python::python_archs4;
pub use submodule::{launch_archs4_correlation, launch_archs4_tissue};
pub use tissue::{tissue, Species};
