mod cli;
pub use cli::launch_seq;
mod types;
pub use types::{ResultSeq, ResultSeqContainer};
mod functions;
pub use functions::sequence;