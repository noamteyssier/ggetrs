use super::{string_network, StringNetworkType};
use crate::utils::{match_output, write_dataframe, OutputFormat};
use anyhow::Result;

pub fn launch_string_network(
    identifiers: &[String],
    species: usize,
    required_score: Option<f64>,
    network_type: StringNetworkType,
    add_nodes: Option<usize>,
    keep_input_name: bool,
    output: Option<String>,
    format: OutputFormat,
) -> Result<()> {
    let mut dataframe = string_network(
        identifiers,
        species,
        required_score,
        network_type,
        add_nodes,
        keep_input_name,
    )?;
    let output_handle = match_output(output)?;
    write_dataframe(output_handle, &mut dataframe, format.into())?;
    Ok(())
}
