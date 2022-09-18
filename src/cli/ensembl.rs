use crate::ensembl::{DataType, ENSEMBL_RELEASE_STR};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum ModEnsembl {
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

    /// Prints all available databases on Ensembl's SQL database
    Database {
        /// Provides a substring filter to only return databases which contain the substring
        #[clap(short, long, value_parser)]
        filter: Option<String>,

        /// optional filepath to write output to [default=stdout]
        #[clap(short, long, value_parser)]
        output: Option<String>,
    },

    /// Lookup information for genes/transcripts providing ensembl ids
    LookupId {
        /// Ensembl IDS to query
        #[clap(value_parser, min_values = 1, required = true)]
        ensembl_ids: Vec<String>,

        /// optional filepath to write output to [default=stdout]
        #[clap(short, long, value_parser)]
        output: Option<String>,
    },

    /// Lookup information for genes/transcripts providing symbols and species
    LookupSymbol {
        /// Gene symbols to query
        #[clap(value_parser, min_values = 1, required = true)]
        symbols: Vec<String>,

        /// Species/alias to specify
        #[clap(short, long, value_parser, default_value = "homo_sapiens")]
        species: String,

        /// optional filepath to write output to [default=stdout]
        #[clap(short, long, value_parser)]
        output: Option<String>,
    },

    /// Retrieves the latest ensembl release version
    Release,

    /// Retrieves reference files from Ensembl FTP site
    Ref {
        /// Species to query data for
        #[clap(short, long, value_parser, default_value = "homo_sapiens")]
        species: String,

        /// Release to use - will default to latest release
        #[clap(short, long, value_parser, default_value=ENSEMBL_RELEASE_STR)]
        release: usize,

        /// Datatype to query for, provided as a comma-separated list (example: cdna,dna,gtf)
        #[clap(
            short,
            long,
            value_enum,
            value_parser,
            value_delimiter = ',',
            min_values = 1,
            required = true
        )]
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
        #[clap(short, long, value_enum, default_value = "dna")]
        datatype: DataType,
    },
}
