use crate::cli::{StringHomologyArgs, StringMappingArgs, StringNetworkArgs};
use anyhow::Result;
use polars::prelude::*;
use reqwest::blocking::Client;
use serde_json::Value;
use std::io::Cursor;

fn string_api(url_extension: &str, data: &Value) -> Result<DataFrame> {
    let url = format!("https://string-db.org/api/json/{url_extension}");
    let response = Client::new().post(&url).form(data).send()?.text()?;
    let file = Cursor::new(response);
    let table = JsonReader::new(file).finish()?;
    Ok(table)
}

pub fn string_network(args: &StringNetworkArgs) -> Result<DataFrame> {
    let data = args.build_post();
    string_api("network", &data)
}

pub fn string_homology(args: &StringHomologyArgs) -> Result<DataFrame> {
    let data = args.build_post();
    string_api("homology", &data)
}

pub fn string_mapping(args: &StringMappingArgs) -> Result<DataFrame> {
    let data = args.build_post();
    string_api("get_string_ids", &data)
}
