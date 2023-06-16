pub mod autocomplete;
mod fasta;
pub mod parsing;
mod ping;
mod download;
pub use fasta::{FastaRecord, FastaRecords};
pub use ping::ping;
pub use download::download_multiple;
