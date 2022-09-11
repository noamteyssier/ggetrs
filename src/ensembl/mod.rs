mod release;

/// submodule for free-form search
mod search;

/// submodule for command line usage
mod submodule;

pub use search::search;
pub use submodule::launch_ensembl_search;
