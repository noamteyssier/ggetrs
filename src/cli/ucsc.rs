use crate::ucsc::types::SeqType;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum ModUcsc {
    /// Performs a BLAT sequence search on a provided database
    Blat {
        /// Query sequence for BLAT search
        #[clap(required = true)]
        sequence: String,

        /// Specify the structure format
        #[clap(short, long, default_value = "dna")]
        seqtype: SeqType,

        /// Specifies the database name to query
        #[clap(short, long, default_value = "hg38")]
        db_name: String,

        /// Optional filepath to write output to [default=stdout]
        #[clap(short, long)]
        output: Option<String>,
    },
}
