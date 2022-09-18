use clap::Subcommand;

#[derive(Subcommand)]
pub enum ModUniprot {
    /// Searches through descriptions on Uniprot
    Query {
        /// Search terms to query
        #[clap(value_parser, min_values = 1, required = true)]
        search_terms: Vec<String>,

        /// Taxon to filter results (human: 9606, mouse: 10090)
        #[clap(short, long, value_parser)]
        taxon: Option<usize>,

        /// optional filepath to write output to [default=stdout]
        #[clap(short, long, value_parser)]
        output: Option<String>,
    },
}
