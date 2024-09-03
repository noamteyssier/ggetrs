use crate::cli::{
    StringFunctionalEnrichmentArgs, StringHomologyArgs, StringInteractionsArgs, StringMappingArgs,
    StringNetworkArgs,
};
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

fn string_api_tsv(url_extension: &str, data: &Value) -> Result<DataFrame> {
    let url = format!("https://string-db.org/api/tsv/{url_extension}");
    let response = Client::new().post(&url).form(data).send()?.text()?;
    let file = Cursor::new(response);
    let table = CsvReadOptions::default()
        .with_parse_options(CsvParseOptions::default().with_separator(b'\t'))
        .with_has_header(true)
        .into_reader_with_file_handle(file)
        .finish()?;
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

pub fn string_interactions(args: &StringInteractionsArgs) -> Result<DataFrame> {
    let data = args.build_post();
    string_api("interaction_partners", &data)
}

pub fn string_functional_enrichment(args: &StringFunctionalEnrichmentArgs) -> Result<DataFrame> {
    let data = args.build_post();
    string_api_tsv("enrichment", &data)
}
