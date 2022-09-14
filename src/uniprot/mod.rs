/// Submodule for uniprot queries
mod query;

/// Submodule for cli
mod submodule;

pub use query::query;
pub use submodule::launch_uniprot_query;
