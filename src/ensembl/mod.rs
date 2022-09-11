mod release;

/// submodule for free-form search
mod search;

/// submodule for command line usage
mod submodule;

/// submodule for python usage
mod python;

pub use search::search;
pub use submodule::launch_ensembl_search;
pub use python::python_ensembl_search;
