use crate::cli::StringNetworkArgs;
use anyhow::Result;
use polars::prelude::*;
use reqwest::blocking::Client;
use std::io::Cursor;

pub fn string_network(args: &StringNetworkArgs) -> Result<DataFrame> {
    let url = "https://string-db.org/api/json/network";
    let data = args.build_post();
    let response = Client::new().post(url).form(&data).send()?.text()?;
    let file = Cursor::new(response);
    let table = JsonReader::new(file).finish()?;
    Ok(table)
}
