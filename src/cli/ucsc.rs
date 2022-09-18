use clap::Subcommand;
use crate::ucsc::types::SeqType;

#[derive(Subcommand)]
pub enum ModUcsc {
    /// Performs a BLAT sequence search on a provided database
    Blat {
        /// PDB Id to request info
        #[clap(value_parser, min_values = 1, max_values = 1, required = true)]
        sequence: String,

        /// Specify the structure format
        #[clap(short, long, value_parser, default_value = "dna")]
        seqtype: SeqType,

        /// Specifies the database name to query
        #[clap(short, long, value_parser, default_value = "hg38")]
        db_name: String,

        /// Optional filepath to write output to [default=stdout]
        #[clap(short, long, value_parser)]
        output: Option<String>,
    },
}
