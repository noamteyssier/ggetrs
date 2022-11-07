use super::{ModArchS4, ModChembl, ModEnrichr, ModEnsembl, ModNcbi, ModPdb, ModUcsc, ModUniprot};
use crate::{
    blast::types::{BlastDatabase, BlastProgram},
    ensembl::ENSEMBL_RELEASE_STR,
};
use clap::{Parser, Subcommand};
use clap_complete::Shell;

#[derive(Parser)]
#[clap(author, version, about)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Perform an enrichment analysis on a list of genes using Enrichr.
    #[clap(subcommand)]
    Enrichr(ModEnrichr),

    /// Queries gene-specific information using ARCHS4
    #[clap(subcommand)]
    ARCHS4(ModArchS4),

    /// Performs a BLAST query for a given sequence
    Blast {
        /// query sequence to BLAST
        #[clap(value_parser, required = true)]
        query: String,

        /// blast program to use
        #[clap(short, long)]
        program: Option<BlastProgram>,

        /// blast database to use
        #[clap(short, long)]
        database: Option<BlastDatabase>,

        /// Number of hits to return
        #[clap(short, long, default_value = "50")]
        limit: usize,

        /// Minimum expected value to consider
        #[clap(short, long, default_value = "10.0")]
        expect: f64,

        /// Whether to use a complexity filter (default = false)
        #[clap(short = 'f', long)]
        low_comp_filter: bool,

        /// Whether to use MEGABLAST (default = true)
        #[clap(short, long)]
        megablast: bool,

        /// optional filepath to write output to [default=stdout]
        #[clap(short, long)]
        output: Option<String>,
    },

    /// Queries information from Chembl Chemical Database
    #[clap(subcommand)]
    Chembl(ModChembl),

    /// Searches through descriptions on ENSEMBL
    Search {
        /// Search terms to query
        #[clap(value_parser, required = true)]
        search_terms: Vec<String>,

        /// Ensembl database to perform search in
        #[clap(short, long)]
        database: Option<String>,

        /// species used in database
        #[clap(short, long, default_value = "homo_sapiens")]
        species: String,

        /// database type specied by Ensembl
        #[clap(short = 't', long, default_value = "core")]
        db_type: String,

        /// release number to use for database
        #[clap(short, long, default_value=ENSEMBL_RELEASE_STR)]
        release: usize,

        /// assembly to use for species
        #[clap(short, long, default_value = "38")]
        assembly: String,

        /// optional filepath to write output to [default=stdout]
        #[clap(short, long)]
        output: Option<String>,
    },

    /// Queries symbols or Ensembl IDs across multiple databases and aggregates results
    Info {
        /// Search terms to query (Gene symbols of Ensembl IDs)
        #[clap(required = true)]
        search_terms: Vec<String>,

        /// Species name to use: currently this MUST match the taxon_id
        #[clap(short, long, default_value = "homo_sapiens")]
        species: String,

        /// Taxon ID to use: currently this MUST match the species
        #[clap(short, long, default_value = "9606")]
        taxon_id: usize,

        /// optional filepath to write output to [default=stdout]
        #[clap(short, long)]
        output: Option<String>,
    },

    /// Queries sequences from ensembl and UniProt
    Seq {
        /// Search terms to query (can be Ensembl IDs or Gene Symbols)
        #[clap(value_parser, required = true)]
        search_terms: Vec<String>,

        /// Return the amino acid sequence instead of nucleotide sequence.
        #[clap(short, long, action)]
        translate: bool,

        /// Species/alias to specify
        #[clap(short, long, default_value = "homo_sapiens")]
        species: Option<String>,

        /// optional filepath to write output to [default=stdout]
        #[clap(short, long)]
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

    /// Set up autocomplete for various shells
    Autocomplete {
        /// Shell to generate autocompletions for
        #[clap(short, long)]
        shell: Shell,
    },
}
