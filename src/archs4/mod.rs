/// module for querying most correlated genes from a query gene
mod correlation;

/// module for querying the tissues where a query gene is expressed
mod tissue;

/// module for command line interface
mod submodule;

/// module for python interface
mod python;

pub use correlation::correlation;
pub use tissue::{Species, tissue};
pub use submodule::{launch_archs4_correlation, launch_archs4_tissue};
pub use python::python_archs4;
