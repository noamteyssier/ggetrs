use clap::Subcommand;

#[derive(Subcommand)]
pub enum ModEnrichr {
    Enrichr {
        /// any database listed at: https://maayanlab.cloud/Enrichr/#libraries
        /// some shorthands include: pathway, transcription, ontology, diseases_drugs, celltypes,
        /// and kinase_interactions.
        #[clap(short, long)]
        library: String,

        /// list of gene symbols to perform enrichment analysis on.
        #[clap(value_parser, required = true)]
        gene_list: Vec<String>,

        /// optional filepath to write output to [default=stdout]
        #[clap(short, long)]
        output: Option<String>,
    },

    /// List all available libraries and their descriptions
    List {
        /// Return library names in plaintext
        #[clap(short, long)]
        minimal: bool,

        /// List the categorization of libraries
        #[clap(short = 't', long)]
        list_categories: bool,

        /// Filter to a category ID
        #[clap(short, long)]
        category: Option<usize>,

        /// optional filepath to write output to [default=stdout]
        #[clap(short, long)]
        output: Option<String>,
    },
}
