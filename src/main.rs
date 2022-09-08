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
    Enrichr {
        #[clap(short, long, value_parser)]
        library: String,
        #[clap(short, long, value_parser, min_values=1, required=true)]
        gene_list: Vec<String>,
        #[clap(short, long, value_parser)]
        output: Option<String>
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
