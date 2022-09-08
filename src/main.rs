use clap::{Parser, Subcommand};
use ggetrs::{enrichr::launch_enrich, RequestError};

// fn default_gene_list() -> Vec<String> {
//     let gene_list = vec![
//         "PHF14", "RBM3", "MSL1", "PHF21A", "ARL10", "INSR", "JADE2", "P2RX7",
//         "LINC00662", "CCDC101", "PPM1B", "KANSL1L", "CRYZL1", "ANAPC16", "TMCC1",
//         "CDH8", "RBM11", "CNPY2", "HSPA1L", "CUL2", "PLBD2", "LARP7", "TECPR2", 
//         "ZNF302", "CUX1", "MOB2", "CYTH2", "SEC22C", "EIF4E3", "ROBO2",
//         "ADAMTS9-AS2", "CXXC1", "LINC01314", "ATF7", "ATP5F1"
//     ];
//     gene_list.iter().map(|x| x.to_string()).collect()
// }

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
