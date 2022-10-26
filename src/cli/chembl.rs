use clap::Subcommand;

#[derive(Subcommand)]
pub enum ModChembl {
    /// Queries chemical activity for a provided item
    Activity {
        /// Query to retrieve bioactivity
        #[clap(value_parser, required = true)]
        query: String,

        /// Number of results to return
        #[clap(short, long, default_value = "500")]
        limit: usize,

        /// Optional filepath to write output to [default=stdout]
        #[clap(short, long)]
        output: Option<String>,
    },
}
