use clap::Subcommand;

#[derive(Subcommand)]
pub enum ModChembl {
    /// Queries chemical activity for a provided item
    Activity {
        /// Query to retrieve bioactivity
        #[clap(value_parser, min_values = 1, max_values = 1, required = true)]
        query: String,

        /// Number of results to return
        #[clap(short, long, value_parser, default_value = "500")]
        limit: usize,

        /// Optional filepath to write output to [default=stdout]
        #[clap(short, long, value_parser)]
        output: Option<String>,
    },
}
