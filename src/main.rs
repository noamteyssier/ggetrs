use clap::{Parser, Subcommand};
use ggetrs::{
    RequestError, 
    enrichr::launch_enrich, 
    archs4::{launch_archs4_correlation, launch_archs4_tissue, Species},
    ensembl::{launch_ensembl_search, launch_ensembl_database, launch_ensembl_release, launch_ensembl_reference, DataType, ENSEMBL_RELEASE_STR, launch_ensembl_list_species}
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
    ARCHS4(ModArchS4),

    /// Searches through descriptions on ENSEMBL
    Search {
        /// Search terms to query
        #[clap(value_parser, min_values=1, required=true)]
        search_terms: Vec<String>,

        /// database
        #[clap(short, long, value_parser)]
        database: Option<String>,

        /// species used in database
        #[clap(short, long, value_parser, default_value="homo_sapiens")]
        species: String,

        /// database type specied by Ensembl
        #[clap(short='t', long, value_parser, default_value="core")]
        db_type: String,

        /// release number to use for database
        #[clap(short, long, value_parser, default_value=ENSEMBL_RELEASE_STR)]
        release: usize,

        /// assembly to use for species
        #[clap(short, long, value_parser, default_value="38")]
        assembly: String,

        /// optional filepath to write output to [default=stdout]
        #[clap(short, long, value_parser)]
        output: Option<String>,
    },

    /// Queries information from Ensembl
    #[clap(subcommand)]
    Ensembl(ModEnsembl)
}

#[derive(Subcommand)]
enum ModArchS4{
    /// Performs a gene-correlation analysis
    Correlate {
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

#[derive(Subcommand)]
enum ModEnsembl {
    /// Searches through descriptions on ENSEMBL
    Search {
        /// Search terms to query
        #[clap(value_parser, min_values=1, required=true)]
        search_terms: Vec<String>,
        
        /// database
        #[clap(short, long, value_parser)]
        database: Option<String>,
        
        /// species used in database
        #[clap(short, long, value_parser, default_value="homo_sapiens")]
        species: String,
        
        /// database type specied by Ensembl
        #[clap(short='t', long, value_parser, default_value="core")]
        db_type: String,
        
        /// release number to use for database
        #[clap(short, long, value_parser, default_value=ENSEMBL_RELEASE_STR)]
        release: usize,
        
        /// assembly to use for species
        #[clap(short, long, value_parser, default_value="38")]
        assembly: String,
        
        /// optional filepath to write output to [default=stdout]
        #[clap(short, long, value_parser)]
        output: Option<String>,
    },

    /// Prints all available databases on Ensembl's SQL database
    Database {
        /// Provides a substring filter to only return databases which contain the substring
        #[clap(short, long, value_parser)]
        filter: Option<String>,
        
        /// optional filepath to write output to [default=stdout]
        #[clap(short, long, value_parser)]
        output: Option<String>,
    },

    /// Retrieves the latest ensembl release version
    Release,

    /// Retrieves reference files from Ensembl FTP site
    Ref {
        /// Species to query data for
        #[clap(short, long, value_parser, default_value="homo_sapiens")]
        species: String,

        /// Release to use - will default to latest release
        #[clap(short, long, value_parser, default_value=ENSEMBL_RELEASE_STR)]
        release: usize,
        
        /// Datatype to query for, provided as a comma-separated list (example: cdna,dna,gtf)
        #[clap(short, long, value_enum, value_parser, value_delimiter=',', min_values=1, required=true)]
        datatype: Vec<DataType>,
        
        /// Optional filepath to write output to [default=stdout]
        #[clap(short, long, value_parser)]
        output: Option<String>,
    },

    /// Retrieves the list of species from ENSEMBL FTP site
    Species {
        /// Release to use - will default to latest release
        #[clap(short, long, value_parser, default_value=ENSEMBL_RELEASE_STR)]
        release: usize,
        
        /// Optional filepath to write output to [default=stdout]
        #[clap(short, long, value_parser)]
        output: Option<String>,
        
        /// Datatype to query species list
        #[clap(short, long, value_enum, default_value="dna")]
        datatype: DataType,
    }
}

fn main() -> Result<(), RequestError> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Enrichr { library, gene_list, output } => {
            launch_enrich(library, gene_list, output)?;
        },
        Commands::ARCHS4(sub) => match sub {
            ModArchS4::Correlate { gene_name, count, output } => {
                launch_archs4_correlation(gene_name, *count, output)?;
            },
            ModArchS4::Tissue { gene_name, species, output } => {
                launch_archs4_tissue(gene_name, species, output)?;
            }
        },
        Commands::Search { search_terms, database, species, db_type, release, assembly, output } => {
            launch_ensembl_search(search_terms, database, species, db_type, release, assembly, output)?;
        },
        Commands::Ensembl(sub) => match sub {
            ModEnsembl::Search { search_terms, database, species, db_type, release, assembly, output } => {
                launch_ensembl_search(search_terms, database, species, db_type, release, assembly, output)?;
            },
            ModEnsembl::Database { filter, output } => {
                launch_ensembl_database(filter, output)?;
            },
            ModEnsembl::Release => {
                launch_ensembl_release()?;
            },
            ModEnsembl::Ref { species, release, datatype, output } => {
                launch_ensembl_reference(species, *release, datatype, output)?;
            },
            ModEnsembl::Species { release, datatype, output} => {
                launch_ensembl_list_species(*release, datatype, output)?;
            }
        }
    };

    Ok(())
}
