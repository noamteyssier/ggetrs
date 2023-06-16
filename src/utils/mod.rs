pub mod autocomplete;
mod download;
mod fasta;
pub mod parsing;
mod ping;
pub use download::download_multiple;
pub use fasta::{FastaRecord, FastaRecords};
pub use ping::ping;
