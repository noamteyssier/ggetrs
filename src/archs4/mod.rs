/// module for querying most correlated genes from a query gene
mod correlation;

/// module for command line interface
mod submodule;

pub use correlation::correlation;
pub use submodule::launch_archs4_correlation;
