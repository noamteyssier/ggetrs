use std::fmt;
use clap::clap_derive::ArgEnum;
use reqwest::{Result, blocking::Client};
use serde::Serialize;

#[derive(ArgEnum, Debug, Clone, Default)]
pub enum Species{
    #[default]
    Human,
    Mouse
}
impl fmt::Display for Species {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Human => "human",
            Self::Mouse => "mouse"
        })
    }
}

#[derive(Serialize, Debug)]
pub struct ResponseTissue {
    results: Vec<ResultTissue>
}
impl fmt::Display for ResponseTissue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).expect("cannot serialize"))
    }
}
impl ResponseTissue {
    pub fn from_str(response: &str) -> Self {
        let results = Self::parse_str(response);
        Self { results }
    }
    fn parse_str(response: &str) -> Vec<ResultTissue> {
        response
            .split('\n')
            .filter_map(|x| ResultTissue::from_line(x))
            .collect()
    }
}

#[derive(Serialize, Debug)]
pub struct ResultTissue {
    id: String,
    min: f64,
    q1: f64,
    median: f64,
    q3: f64,
    max: f64,
    color: String
}
impl fmt::Display for ResultTissue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).expect("cannot serialize"))
    }
}
impl ResultTissue {

    fn parse_float(record: Option<&str>) -> Option<f64> {
        match record {
            Some(value) => match value.parse::<f64>() {
                Ok(x) => Some(x),
                Err(_) => None
            },
            None => None
        }
    }

    pub fn from_line(line: &str) -> Option<Self> {
        let mut records = line.split(',');
        let id = match records.next() {
            Some(value) => value.to_string(),
            None => return None
        };
        let min = match Self::parse_float(records.next()) {
            Some(value) => value,
            None => return None
        };
        let q1 = match Self::parse_float(records.next()) {
            Some(value) => value,
            None => return None
        };
        let median = match Self::parse_float(records.next()) {
            Some(value) => value,
            None => return None
        };
        let q3 = match Self::parse_float(records.next()) {
            Some(value) => value,
            None => return None
        };
        let max = match Self::parse_float(records.next()) {
            Some(value) => value,
            None => return None
        };
        let color = match records.next() {
            Some(value) => value.to_string(),
            None => return None
        };
        Some(Self {
            id, min, q1, median, q3, max, color
        })
    }

}

/// Returns the tissue-specific expression of a provided gene_name
pub fn tissue(gene_name: &str, species: &Species) -> Result<ResponseTissue> {
    let client = Client::new();

    let url = format!(
        "https://maayanlab.cloud/archs4/search/loadExpressionTissue.php?{}",
        format!("search={}&species={}&type=tissue", gene_name, species)
        );

    let raw_response = client.post(url)
        .header("Content-Type", "application/json")
        .send()?
        .text()?;

    let response = ResponseTissue::from_str(&raw_response);

    Ok(response)
}
