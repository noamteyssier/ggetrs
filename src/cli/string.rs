use crate::{
    string::StringNetworkType,
    utils::{match_output, OutputFormat},
};
use anyhow::Result;
use bon::{builder, Builder};
use clap::{Parser, Subcommand};
use serde_json::{json, Value};
use std::io::Write;

#[derive(Subcommand)]
pub enum ModString {
    /// Maps common protein names, synonyms and UniProt identifiers into STRING identifiers
    MapIds {
        #[clap(flatten)]
        args: StringMappingArgs,

        #[clap(flatten)]
        output: OutputArgs,
    },
    /// Retrieves the network interactions for your input protein(s) in various text based formats
    Network {
        #[clap(flatten)]
        args: StringNetworkArgs,

        #[clap(flatten)]
        output: OutputArgs,
    },
    /// Retrieve the protein similarity scores between the input proteins
    Homology {
        #[clap(flatten)]
        args: StringHomologyArgs,

        #[clap(flatten)]
        output: OutputArgs,
    },
    /// Gets all the STRING interaction partners of your proteins
    Interactions {
        #[clap(flatten)]
        args: StringInteractionsArgs,

        #[clap(flatten)]
        output: OutputArgs,
    },
    /// Performs the enrichment analysis of your set of proteins for the Gene Ontology, KEGG pathways, UniProt Keywords, PubMed publications, Pfam, InterPro and SMART domains.
    Enrichment {
        #[clap(flatten)]
        args: StringFunctionalEnrichmentArgs,

        #[clap(flatten)]
        output: OutputArgs,
    },
    /// Gets the functional annotation (Gene Ontology, UniProt Keywords, PFAM, INTERPRO and SMART domains) of your list of proteins.
    Annotations {
        #[clap(flatten)]
        args: StringFunctionalAnnotationArgs,

        #[clap(flatten)]
        output: OutputArgs,
    },
    /// Tests if your network has more interactions than expected
    PpiEnrichment {
        #[clap(flatten)]
        args: StringPpiEnrichmentArgs,

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

#[derive(Debug, Clone, Parser, Builder)]
#[clap(next_help_heading = "STRING Network Arguments")]
/// Retrieves the network interactions for your input protein(s) in various text based formats
pub struct StringNetworkArgs {
    /// List of genes to retrieve network for
    #[clap(required = true)]
    pub identifiers: Vec<String>,

    /// Species to retrieve network for (NCBI taxonomy ID)
    #[clap(short, long, default_value = "9606")]
    #[builder(default = 9606)]
    pub species: usize,

    /// threshold of significance to include a interaction, a number between 0 and 1000 (default depends on the network)
    #[clap(short, long)]
    pub required_score: Option<f64>,

    /// The type of network to retrieve
    #[clap(short, long, default_value = "functional")]
    #[builder(default = StringNetworkType::Functional)]
    pub network_type: StringNetworkType,

    /// adds a number of proteins with to the network based on their confidence score
    #[clap(short, long)]
    pub add_nodes: Option<usize>,

    /// when available use submitted names in the preferredName column when true
    #[clap(short = 'q', long)]
    #[builder(default = false)]
    pub show_query_node_labels: bool,

    /// identifier of the caller to provide to the server
    #[clap(short, long, default_value = "ggetrs")]
    #[builder(default = "ggetrs".to_string())]
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

#[derive(Debug, Clone, Parser, Builder)]
#[clap(next_help_heading = "STRING Homology Arguments")]
pub struct StringHomologyArgs {
    /// List of genes to retrieve network for
    #[clap(required = true)]
    pub identifiers: Vec<String>,

    /// Species to retrieve network for (NCBI taxonomy ID)
    #[clap(short, long, default_value = "9606")]
    #[builder(default = 9606)]
    pub species: usize,

    /// identifier of the caller to provide to the server
    #[clap(short, long, default_value = "ggetrs")]
    #[builder(default = "ggetrs".to_string())]
    pub caller_identity: String,
}
impl StringHomologyArgs {
    #[must_use]
    pub fn build_post(&self) -> Value {
        json!({
            "identifiers": self.identifiers.join("%0d"),
            "species": self.species,
            "caller_identity": self.caller_identity,
        })
    }
}

#[derive(Debug, Clone, Parser, Builder)]
#[clap(next_help_heading = "STRING Mapping Identifiers Arguments")]
pub struct StringMappingArgs {
    /// List of genes to retrieve network for
    #[clap(required = true)]
    pub identifiers: Vec<String>,

    /// insert column with your input identifier
    #[clap(short, long)]
    #[builder(default = false)]
    pub echo_query: bool,

    /// limits the number of matches per query identifier (best matches come first)
    #[clap(short, long)]
    pub limit: Option<usize>,

    /// Species to retrieve network for (NCBI taxonomy ID)
    #[clap(short, long, default_value = "9606")]
    #[builder(default = 9606)]
    pub species: usize,

    /// identifier of the caller to provide to the server
    #[clap(short, long, default_value = "ggetrs")]
    #[builder(default = "ggetrs".to_string())]
    pub caller_identity: String,
}
impl StringMappingArgs {
    #[must_use]
    pub fn build_post(&self) -> Value {
        let mut data = json!({
            "identifiers": self.identifiers.join("%0d"),
            "echo_query": self.echo_query,
            "species": self.species,
            "caller_identity": self.caller_identity,
        });
        if let Some(limit) = self.limit {
            data["limit"] = json!(limit);
        }
        data
    }
}

/// Gets all the STRING interaction partners of your proteins
#[derive(Debug, Clone, Parser, Builder)]
#[clap(next_help_heading = "STRING Interactions Arguments")]
pub struct StringInteractionsArgs {
    /// List of genes to retrieve network for
    #[clap(required = true)]
    pub identifiers: Vec<String>,

    /// Species to retrieve network for (NCBI taxonomy ID)
    #[clap(short, long, default_value = "9606")]
    #[builder(default = 9606)]
    pub species: usize,

    /// limits the number of interaction partners retrieved per protein (most confident interactions come first)
    /// default set by the server (usually 10)
    #[clap(short, long)]
    pub limit: Option<usize>,

    /// threshold of significance to include a interaction, a number between 0 and 1000 (default depends on the network)
    #[clap(short, long)]
    pub required_score: Option<f64>,

    /// The type of network to retrieve
    #[clap(short, long, default_value = "functional")]
    #[builder(default = StringNetworkType::Functional)]
    pub network_type: StringNetworkType,

    /// identifier of the caller to provide to the server
    #[clap(short, long, default_value = "ggetrs")]
    #[builder(default = "ggetrs".to_string())]
    pub caller_identity: String,
}
impl StringInteractionsArgs {
    #[must_use]
    pub fn build_post(&self) -> Value {
        let mut data = json!({
            "identifiers": self.identifiers.join("%0d"),
            "species": self.species,
            "network_type": self.network_type.to_string(),
            "caller_identity": self.caller_identity,
        });
        if let Some(limit) = self.limit {
            data["limit"] = json!(limit);
        }
        if let Some(score) = self.required_score {
            data["required_score"] = json!(score);
        }
        data
    }
}

/// Performs the enrichment analysis of your set of proteins for the Gene Ontology, KEGG pathways, UniProt Keywords, PubMed publications, Pfam, InterPro and SMART domains.
#[derive(Debug, Clone, Parser, Builder)]
#[clap(next_help_heading = "STRING Functional Enrichment Arguments")]
pub struct StringFunctionalEnrichmentArgs {
    /// List of genes to retrieve network for
    #[clap(required = true)]
    pub identifiers: Vec<String>,

    /// Background list of genes to compare against (Only STRING identifiers are accepted)
    /// If not provided, the whole genome of the species is used
    #[clap(short, long)]
    pub background: Option<Vec<String>>,

    /// Species to retrieve network for (NCBI taxonomy ID)
    #[clap(short, long, default_value = "9606")]
    #[builder(default = 9606)]
    pub species: usize,

    /// identifier of the caller to provide to the server
    #[clap(short, long, default_value = "ggetrs")]
    #[builder(default = "ggetrs".to_string())]
    pub caller_identity: String,
}
impl StringFunctionalEnrichmentArgs {
    #[must_use]
    pub fn build_post(&self) -> Value {
        let mut data = json!({
            "identifiers": self.identifiers.join("%0d"),
            "species": self.species,
            "caller_identity": self.caller_identity,
        });
        if let Some(background) = &self.background {
            data["background"] = json!(background.join("%0d"));
        }
        data
    }
}

/// Gets the functional annotation (Gene Ontology, UniProt Keywords, PFAM, INTERPRO and SMART domains) of your list of proteins.
#[derive(Debug, Clone, Parser, Builder)]
#[clap(next_help_heading = "STRING Functional Annotations Arguments")]
pub struct StringFunctionalAnnotationArgs {
    /// List of genes to retrieve network for
    #[clap(required = true)]
    pub identifiers: Vec<String>,

    /// Species to retrieve network for (NCBI taxonomy ID)
    #[clap(short, long, default_value = "9606")]
    #[builder(default = 9606)]
    pub species: usize,

    /// Return PubMed annotations in addition to other categories
    #[clap(short = 'p', long)]
    #[builder(default = false)]
    pub allow_pubmed: bool,

    /// Only return PubMed annotations
    #[clap(short = 'P', long)]
    #[builder(default = false)]
    pub only_pubmed: bool,

    /// identifier of the caller to provide to the server
    #[clap(short, long, default_value = "ggetrs")]
    #[builder(default = "ggetrs".to_string())]
    pub caller_identity: String,
}
impl StringFunctionalAnnotationArgs {
    #[must_use]
    pub fn build_post(&self) -> Value {
        json!({
            "identifiers": self.identifiers.join("%0d"),
            "species": self.species,
            "caller_identity": self.caller_identity,
            "allow_pubmed": self.allow_pubmed,
            "only_pubmed": self.only_pubmed,
        })
    }
}

/// Tests if your network has more interactions than expected
#[derive(Debug, Clone, Parser, Builder)]
#[clap(next_help_heading = "STRING PPI Enrichment Arguments")]
pub struct StringPpiEnrichmentArgs {
    /// List of genes to retrieve network for
    #[clap(required = true)]
    pub identifiers: Vec<String>,

    /// Species to retrieve network for (NCBI taxonomy ID)
    #[clap(short, long, default_value = "9606")]
    #[builder(default = 9606)]
    pub species: usize,

    /// threshold of significance to include a interaction, a number between 0 and 1000 (default depends on the network)
    #[clap(short, long)]
    pub required_score: Option<f64>,

    /// using this parameter you can specify the background proteome of your experiment. Only STRING identifiers will be recognised (each must be seperated by "%0d") e.g. '7227.FBpp0077451%0d7227.FBpp0074373'. You can map STRING identifiers using mapping identifiers method.
    #[clap(short, long)]
    pub background: Option<Vec<String>>,

    /// identifier of the caller to provide to the server
    #[clap(short, long, default_value = "ggetrs")]
    #[builder(default = "ggetrs".to_string())]
    pub caller_identity: String,
}
impl StringPpiEnrichmentArgs {
    #[must_use]
    pub fn build_post(&self) -> Value {
        let mut data = json!({
            "identifiers": self.identifiers.join("%0d"),
            "species": self.species,
            "caller_identity": self.caller_identity,
        });
        if let Some(score) = self.required_score {
            data["required_score"] = json!(score);
        }
        if let Some(background) = &self.background {
            data["background"] = json!(background.join("%0d"));
        }
        data
    }
}
