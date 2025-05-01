mod cli;
mod functions;
mod types;

#[cfg(feature = "python")]
mod python;

pub use cli::launch_seq;
pub use functions::sequence;
pub use types::{ResultSeq, ResultSeqContainer};

#[cfg(feature = "python")]
pub use python::python_seq;
