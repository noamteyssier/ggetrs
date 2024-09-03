use std::io::Write;

use crate::utils::OutputFormat;
use crate::{string::StringNetworkType, utils::match_output};
use anyhow::Result;
use clap::{Parser, Subcommand};
use serde_json::{json, Value};

#[derive(Subcommand)]
pub enum ModString {
    /// Retrieves string network for a collection of genes
    Network {
        #[clap(flatten)]
        args: StringNetworkArgs,

        #[clap(flatten)]
        output: OutputArgs,
    },
}

#[derive(Debug, Clone, Parser)]
#[clap(next_help_heading = "Output Arguments")]
pub struct OutputArgs {
    /// Optional filepath to write output to [default=stdout]
    #[clap(short, long)]
    pub output: Option<String>,

    /// The output format to use
    #[clap(short, long, default_value = "tsv")]
    pub format: OutputFormat,
}
impl OutputArgs {
    pub fn get_writer(&self) -> Result<Box<dyn Write>> {
        match_output(self.output.clone())
    }
}

#[derive(Debug, Clone, Parser)]
#[clap(next_help_heading = "STRING Network Arguments")]
pub struct StringNetworkArgs {
    /// List of genes to retrieve network for
    #[clap(required = true)]
    pub identifiers: Vec<String>,

    /// Species to retrieve network for (NCBI taxonomy ID)
    #[clap(short, long, default_value = "9606")]
    pub species: usize,

    /// threshold of significance to include a interaction, a number between 0 and 1000 (default depends on the network)
    #[clap(short, long)]
    pub required_score: Option<f64>,

    /// The type of network to retrieve
    #[clap(short, long, default_value = "functional")]
    pub network_type: StringNetworkType,

    /// adds a number of proteins with to the network based on their confidence score
    #[clap(short, long)]
    pub add_nodes: Option<usize>,

    /// when available use submitted names in the preferredName column when true
    #[clap(short = 'q', long)]
    pub show_query_node_labels: bool,

    /// identifier of the caller to provide to the server
    #[clap(short, long, default_value = "ggetrs")]
    pub caller_identity: String,
}
impl StringNetworkArgs {
    #[must_use]
    pub fn build_post(&self) -> Value {
        let mut data = json!({
            "identifiers": self.identifiers.join("%0d"),
            "species": self.species,
            "network_type": self.network_type.to_string(),
            "caller_identity": self.caller_identity,
        });
        data["show_query_node_labels"] = if self.show_query_node_labels {
            json!(1)
        } else {
            json!(0)
        };
        if let Some(score) = self.required_score {
            data["required_score"] = json!(score);
        }
        if let Some(nodes) = self.add_nodes {
            data["add_nodes"] = json!(nodes);
        }
        data
    }
}
