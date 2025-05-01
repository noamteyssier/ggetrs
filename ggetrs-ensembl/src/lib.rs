/// Module for functions
pub mod functions;

/// Module for handling result structs and containers
pub mod types;

/// submodule for command line usage
mod cli;

/// submodule for utilities
mod utils;

/// submodule for constants shared between multiple functions
mod constants;

/// submodule for python usage
#[cfg(feature = "python")]
mod python;

pub use cli::{
    launch_ensembl_database, launch_ensembl_list_species, launch_ensembl_lookup_id,
    launch_ensembl_lookup_symbol, launch_ensembl_reference, launch_ensembl_release,
    launch_ensembl_search,
};
pub use constants::{ENSEMBL_RELEASE, ENSEMBL_RELEASE_STR};
pub use functions::{
    async_lookup_symbol, database, list_species, lookup_id, lookup_symbol, reference, release,
    search,
};
pub use types::{DataType, FtpFile};

#[cfg(feature = "python")]
pub use python::{python_ensembl, python_ensembl_database, python_ensembl_search};
