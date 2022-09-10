use clap::{Parser, Subcommand};
use ggetrs::{
    enrichr::launch_enrich, 
    archs4::{launch_archs4_correlation, launch_archs4_tissue, Species},
    RequestError
};

#[derive(Parser)]
#[clap(author, version, about)]
#[clap(propagate_version = true)]
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
    },

    /// Queries gene-specific information using ARCHS4
    #[clap(subcommand)]
    ARCHS4(ModArchS4)
}

#[derive(Subcommand)]
enum ModArchS4{
    /// Performs a gene-correlation analysis
    Correlation {
        /// Gene name to query for correlation
        #[clap(value_parser, required=true)]
        gene_name: String,
        /// number of values to recover
        #[clap(short, long, value_parser, default_value="100")]
        count: usize,
        /// output filepath to write to [default=stdout]
        #[clap(short, long, value_parser)]
        output: Option<String>,
    },
    /// Perform a tissue-enrichment analysis
    Tissue {
        /// Gene name to query for tissue
        #[clap(value_parser, required=true)]
        gene_name: String,
        /// number of values to recover
        #[clap(short, long, value_parser, default_value="human")]
        species: Species,
        /// output filepath to write to [default=stdout]
        #[clap(short, long, value_parser)]
        output: Option<String>,
    }
}

fn main() -> Result<(), RequestError> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Enrichr { library, gene_list, output } => {
            launch_enrich(library, gene_list, output)?;
        },
        Commands::ARCHS4(sub) => match sub {
            ModArchS4::Correlation { gene_name, count, output } => {
                launch_archs4_correlation(gene_name, *count, output)?;
            },
            ModArchS4::Tissue { gene_name, species, output } => {
                launch_archs4_tissue(gene_name, species, output)?;
            }
        }
    };

    Ok(())
}
