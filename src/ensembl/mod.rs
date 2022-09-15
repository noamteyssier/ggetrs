/// Module for functions
pub mod functions;

/// Module for handling result structs and containers
pub mod types;

/// submodule for free-form search
mod search;

/// submodule for retrieving the latest release
mod release;

/// submodule for command line usage
mod submodule;

/// submodule for python usage
mod python;

/// submodule for constants shared between multiple functions
mod constants;

pub use functions::{database, list_species, reference};
pub use types::{DataType, FtpFile};
pub use constants::{ENSEMBL_RELEASE, ENSEMBL_RELEASE_STR};
pub use python::{python_ensembl, python_ensembl_database, python_ensembl_search};
pub use release::release;
pub use search::search;
pub use submodule::{
    launch_ensembl_database, launch_ensembl_list_species, launch_ensembl_reference,
    launch_ensembl_release, launch_ensembl_search,
};
