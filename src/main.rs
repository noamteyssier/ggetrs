use clap::{Parser, Subcommand};
use ggetrs::{enrichr::launch_enrich, RequestError};

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Perform an enrichment analysis on a list of genes using Enrichr.
    Enrichr {
        /// any database listed at: https://maayanlab.cloud/Enrichr/#libraries
        #[clap(short, long, value_parser)]
        library: String,
        /// optional filepath to write output to [default=stdout]
        #[clap(short, long, value_parser)]
        output: Option<String>,
        /// list of gene symbols to perform enrichment analysis on.
        #[clap(value_parser, min_values=1, required=true)]
        gene_list: Vec<String>,
    }
}

#[tokio::main]
async fn main() -> Result<(), RequestError> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Enrichr { library, gene_list, output } => {
            launch_enrich(library, gene_list, output).await?;
        },
    };

    Ok(())
}
