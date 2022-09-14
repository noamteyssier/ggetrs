/// Submodule for uniprot queries
mod query;

/// Container for uniprot query results
mod uniprotinfo;

/// Submodule for cli
mod submodule;

pub use query::query;
pub use uniprotinfo::{UniprotInfo, UniprotInfoContainer};
pub use submodule::launch_uniprot_query;
