use crate::archs4::Species;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum ModArchS4 {
    /// Performs a gene-correlation analysis
    Correlate {
        /// Gene name to query for correlation
        #[clap(value_parser, required = true)]
        gene_name: String,

        /// number of values to recover
        #[clap(short, long, default_value = "100")]
        count: usize,

        /// output filepath to write to [default=stdout]
        #[clap(short, long)]
        output: Option<String>,
    },
    /// Perform a tissue-enrichment analysis
    Tissue {
        /// Gene name to query for tissue
        #[clap(value_parser, required = true)]
        gene_name: String,

        /// Species to use in query
        #[clap(short, long, default_value = "human")]
        species: Species,

        /// output filepath to write to [default=stdout]
        #[clap(short, long)]
        output: Option<String>,
    },
}
