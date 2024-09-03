use super::{string_homology, string_network};
use crate::{
    cli::{OutputArgs, StringHomologyArgs, StringNetworkArgs},
    utils::write_dataframe,
};
use anyhow::Result;

pub fn launch_string_network(args: &StringNetworkArgs, output: &OutputArgs) -> Result<()> {
    let mut dataframe = string_network(args)?;
    let output_handle = output.get_writer()?;
    write_dataframe(output_handle, &mut dataframe, output.format.into())?;
    Ok(())
}

pub fn launch_string_homology(args: &StringHomologyArgs, output: &OutputArgs) -> Result<()> {
    let mut dataframe = string_homology(args)?;
    let output_handle = output.get_writer()?;
    write_dataframe(output_handle, &mut dataframe, output.format.into())?;
    Ok(())
}
