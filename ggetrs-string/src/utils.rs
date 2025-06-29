use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::str::FromStr;

use anyhow::Result;
use polars_core::prelude::*;
use polars_io::prelude::*;

/// Match the output to either a file or stdout
pub fn match_output(path: Option<String>) -> Result<Box<dyn Write>> {
    if let Some(path) = path {
        let file = File::create(path)?;
        let buffer = BufWriter::new(file);
        Ok(Box::new(buffer))
    } else {
        let file = std::io::stdout();
        let buffer = BufWriter::new(file);
        Ok(Box::new(buffer))
    }
}

pub enum WriteConfig {
    CSV { delimiter: u8, header: bool },
    JSON { format: JsonFormat },
}

pub fn write_dataframe<W: Write>(
    output: W,
    dataframe: &mut DataFrame,
    config: WriteConfig,
) -> Result<(), PolarsError> {
    match config {
        WriteConfig::CSV { delimiter, header } => CsvWriter::new(output)
            .with_separator(delimiter)
            .include_header(header)
            .finish(dataframe),
        WriteConfig::JSON { format } => JsonWriter::new(output)
            .with_json_format(format)
            .finish(dataframe),
    }
}

#[derive(Clone, Copy, Debug)]
pub enum OutputFormat {
    Csv,
    Tsv,
    Json,
    Ndjson,
}
impl From<OutputFormat> for WriteConfig {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Csv => WriteConfig::CSV {
                delimiter: b',',
                header: true,
            },
            OutputFormat::Tsv => WriteConfig::CSV {
                delimiter: b'\t',
                header: true,
            },
            OutputFormat::Json => WriteConfig::JSON {
                format: JsonFormat::Json,
            },
            OutputFormat::Ndjson => WriteConfig::JSON {
                format: JsonFormat::JsonLines,
            },
        }
    }
}
impl FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "csv" => Ok(Self::Csv),
            "tsv" => Ok(Self::Tsv),
            "json" => Ok(Self::Json),
            "ndjson" => Ok(Self::Ndjson),
            _ => Err(format!("Invalid output format: {}", s)),
        }
    }
}
