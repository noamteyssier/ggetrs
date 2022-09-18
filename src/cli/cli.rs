use clap::{Parser, AppSettings, Subcommand};
use crate::ensembl::ENSEMBL_RELEASE_STR;
use super::{ModArchS4, ModEnsembl, ModNcbi, ModPdb, ModUcsc, ModUniprot};

#[derive(Parser)]
#[clap(author, version, about)]
#[clap(propagate_version = true)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Perform an enrichment analysis on a list of genes using Enrichr.
    Enrichr {
        /// any database listed at: https://maayanlab.cloud/Enrichr/#libraries
        #[clap(short, long, value_parser)]
        library: String,

        /// optional filepath to write output to [default=stdout]
        #[clap(short, long, value_parser)]
        output: Option<String>,

        /// list of gene symbols to perform enrichment analysis on.
        #[clap(value_parser, min_values = 1, required = true)]
        gene_list: Vec<String>,
    },

    /// Queries gene-specific information using ARCHS4
    #[clap(subcommand)]
    ARCHS4(ModArchS4),

    /// Searches through descriptions on ENSEMBL
    Search {
        /// Search terms to query
        #[clap(value_parser, min_values = 1, required = true)]
        search_terms: Vec<String>,

        /// database
        #[clap(short, long, value_parser)]
        database: Option<String>,

        /// species used in database
        #[clap(short, long, value_parser, default_value = "homo_sapiens")]
        species: String,

        /// database type specied by Ensembl
        #[clap(short = 't', long, value_parser, default_value = "core")]
        db_type: String,

        /// release number to use for database
        #[clap(short, long, value_parser, default_value=ENSEMBL_RELEASE_STR)]
        release: usize,

        /// assembly to use for species
        #[clap(short, long, value_parser, default_value = "38")]
        assembly: String,

        /// optional filepath to write output to [default=stdout]
        #[clap(short, long, value_parser)]
        output: Option<String>,
    },

    /// Queries symbols or Ensembl IDs across multiple databases and aggregates results
    Info {
        /// Search terms to query
        #[clap(value_parser, min_values = 1, required = true)]
        search_terms: Vec<String>,

        /// Taxon ID to use: currently this MUST match the taxon_id
        #[clap(short, long, value_parser, default_value = "homo_sapiens")]
        species: String,

        /// Taxon ID to use: currently this MUST match the species
        #[clap(short, long, value_parser, default_value = "9606")]
        taxon_id: usize,

        /// optional filepath to write output to [default=stdout]
        #[clap(short, long, value_parser)]
        output: Option<String>,
    },

    /// Queries information from Ensembl
    #[clap(subcommand)]
    Ensembl(ModEnsembl),

    /// Queries information from Uniprot
    #[clap(subcommand)]
    Uniprot(ModUniprot),

    /// Queries information from NCBI
    #[clap(subcommand)]
    Ncbi(ModNcbi),

    /// Retrieves structures and information from RCSB PDB
    #[clap(subcommand)]
    Pdb(ModPdb),

    /// Retrieves information from UCSC Genome Browser
    #[clap(subcommand)]
    Ucsc(ModUcsc),
}
