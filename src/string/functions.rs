use std::io::Cursor;

use super::StringNetworkType;
use anyhow::Result;
use polars::prelude::*;
use reqwest::blocking::Client;
use serde_json::{json, Value};

fn build_json_post_request(
    identifiers: &[String],
    species: usize,
    required_score: Option<f64>,
    network_type: StringNetworkType,
    add_nodes: Option<usize>,
    keep_input_name: bool,
) -> Value {
    let mut data = json!({
        "identifiers": identifiers.join("%0d"),
        "species": species,
        "network_type": network_type.to_string(),
        "caller_identity": "ggetrs",
    });
    data["show_query_node_labels"] = if keep_input_name { json!(1) } else { json!(0) };
    if let Some(score) = required_score {
        data["required_score"] = json!(score);
    }
    if let Some(nodes) = add_nodes {
        data["add_nodes"] = json!(nodes);
    }
    data
}

pub fn string_network(
    identifiers: &[String],
    species: usize,
    required_score: Option<f64>,
    network_type: StringNetworkType,
    add_nodes: Option<usize>,
    keep_input_name: bool,
) -> Result<DataFrame> {
    let url = "https://string-db.org/api/json/network";
    let data = build_json_post_request(
        identifiers,
        species,
        required_score,
        network_type,
        add_nodes,
        keep_input_name,
    );
    let response = Client::new().post(url).form(&data).send()?.text()?;
    let file = Cursor::new(response);
    let table = JsonReader::new(file).finish()?;
    Ok(table)
}
