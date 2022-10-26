use clap::Subcommand;

#[derive(Subcommand)]
pub enum ModNcbi {
    /// Retrieves taxon information from NCBI from a query string
    Taxons {
        /// taxon name to query
        #[clap(required = true)]
        query: String,

        /// number of search results to return
        #[clap(short, long, default_value = "5")]
        limit: usize,

        /// optional filepath to write output to [default=stdout]
        #[clap(short, long)]
        output: Option<String>,
    },

    /// Retrieves information for a list of IDs
    QueryIds {
        /// NCBI ids to query
        #[clap(required = true)]
        ids: Vec<usize>,

        /// optional filepath to write output to [default=stdout]
        #[clap(short, long)]
        output: Option<String>,
    },

    /// Retrieves information for a list of symbols (must provide taxon)
    QuerySymbols {
        /// NCBI ids to query
        #[clap(required = true)]
        symbols: Vec<String>,

        /// Taxon ID (human: 9606, mouse: 10090)
        #[clap(short, long, default_value = "9606")]
        taxon_id: usize,

        /// optional filepath to write output to [default=stdout]
        #[clap(short, long)]
        output: Option<String>,
    },
}
