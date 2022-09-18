use crate::pdb::types::{PdbFormat, PdbResource};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum ModPdb {
    /// Retrieves pdb structure for a provided ID
    Structure {
        /// PDB id to retrieve structure
        #[clap(value_parser, min_values = 1, max_values = 1, required = true)]
        pdb_id: String,

        /// Retrieve only the PDB header
        #[clap(short = 'm', long, action)]
        header_only: bool,

        /// Specify the structure format
        #[clap(short, long, value_parser, default_value = "pdb")]
        format: PdbFormat,

        /// Optional filepath to write output to [default=stdout]
        #[clap(short, long, value_parser)]
        output: Option<String>,
    },

    /// Retrieves pdb information for a provided ID and resource
    Info {
        /// PDB Id to request info
        #[clap(value_parser, min_values = 1, max_values = 1, required = true)]
        pdb_id: String,

        /// Specify the structure format
        #[clap(short, long, value_parser, default_value = "entry")]
        resource: PdbResource,

        /// Specifies the Entry or Chain Identifier
        #[clap(short, long, value_parser)]
        identifier: Option<String>,

        /// Optional filepath to write output to [default=stdout]
        #[clap(short, long, value_parser)]
        output: Option<String>,
    },
}
