use clap::Subcommand;

#[derive(Subcommand)]
pub enum ModUniprot {
    /// Searches through descriptions on Uniprot
    Query {
        /// Search terms to query
        #[clap(required = true)]
        search_terms: Vec<String>,

        /// Taxon to filter results (human: 9606, mouse: 10090)
        #[clap(short, long)]
        taxon: Option<usize>,

        /// optional filepath to write output to [default=stdout]
        #[clap(short, long)]
        output: Option<String>,
    },
}
