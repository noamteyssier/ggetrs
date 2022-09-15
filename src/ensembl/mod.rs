/// Module for functions
pub mod functions;

/// Module for handling result structs and containers
pub mod types;

/// submodule for command line usage
mod submodule;

/// submodule for python usage
mod python;

/// submodule for constants shared between multiple functions
mod constants;

pub use constants::{ENSEMBL_RELEASE, ENSEMBL_RELEASE_STR};
pub use functions::{database, list_species, reference, release, search};
pub use python::{python_ensembl, python_ensembl_database, python_ensembl_search};
pub use submodule::{
    launch_ensembl_database, launch_ensembl_list_species, launch_ensembl_reference,
    launch_ensembl_release, launch_ensembl_search,
};
pub use types::{DataType, FtpFile};
