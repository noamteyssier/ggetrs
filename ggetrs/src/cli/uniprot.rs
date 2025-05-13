use clap::Subcommand;

#[derive(Subcommand)]
pub enum ModUniprot {
    /// Searches through descriptions on Uniprot
    Query {
        /// Search terms to query
        #[clap(required = true)]
        search_terms: Vec<String>,

        /// Freeform search as opposed to gene-focused searches
        #[clap(short, long)]
        freeform: bool,

        /// Taxon to filter results (human: 9606, mouse: 10090)
        #[clap(short, long)]
        taxon: Option<usize>,

        /// optional filepath to write output to [default=stdout]
        #[clap(short, long)]
        output: Option<String>,
    },
}
