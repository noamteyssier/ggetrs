use crate::string::StringNetworkType;
use crate::utils::OutputFormat;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum ModString {
    /// Retrieves string network for a collection of genes
    Network {
        /// List of genes to retrieve network for
        #[clap(required = true)]
        genes: Vec<String>,

        /// Optional filepath to write output to [default=stdout]
        #[clap(short, long)]
        output: Option<String>,

        /// Species to retrieve network for (NCBI taxonomy ID)
        #[clap(short, long, default_value = "9606")]
        species: usize,

        /// threshold of significance to include a interaction, a number between 0 and 1000 (default depends on the network)
        #[clap(short, long)]
        threshold: Option<f64>,

        /// The type of network to retrieve
        #[clap(short, long, default_value = "functional")]
        network_type: StringNetworkType,

        /// adds a number of proteins with to the network based on their confidence score
        #[clap(short, long)]
        add_nodes: Option<usize>,

        /// when available use submitted names in the preferredName column when true
        #[clap(short, long)]
        keep_input_name: bool,

        /// The output format to use
        #[clap(short, long, default_value = "tsv")]
        format: OutputFormat,
    },
}
