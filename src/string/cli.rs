use super::{
    string_annotations, string_enrichment, string_homology, string_interactions, string_mapping,
    string_network, string_ppi_enrichment,
};
use crate::{
    cli::{
        OutputArgs, StringFunctionalAnnotationArgs, StringFunctionalEnrichmentArgs,
        StringHomologyArgs, StringInteractionsArgs, StringMappingArgs, StringNetworkArgs,
        StringPpiEnrichmentArgs,
    },
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

pub fn launch_string_mapping(args: &StringMappingArgs, output: &OutputArgs) -> Result<()> {
    let mut dataframe = string_mapping(args)?;
    let output_handle = output.get_writer()?;
    write_dataframe(output_handle, &mut dataframe, output.format.into())?;
    Ok(())
}

pub fn launch_string_interactions(
    args: &StringInteractionsArgs,
    output: &OutputArgs,
) -> Result<()> {
    let mut dataframe = string_interactions(args)?;
    let output_handle = output.get_writer()?;
    write_dataframe(output_handle, &mut dataframe, output.format.into())?;
    Ok(())
}

pub fn launch_string_enrichment(
    args: &StringFunctionalEnrichmentArgs,
    output: &OutputArgs,
) -> Result<()> {
    let mut dataframe = string_enrichment(args)?;
    let output_handle = output.get_writer()?;
    write_dataframe(output_handle, &mut dataframe, output.format.into())?;
    Ok(())
}

pub fn launch_string_annotations(
    args: &StringFunctionalAnnotationArgs,
    output: &OutputArgs,
) -> Result<()> {
    let mut dataframe = string_annotations(args)?;
    let output_handle = output.get_writer()?;
    write_dataframe(output_handle, &mut dataframe, output.format.into())?;
    Ok(())
}

pub fn launch_string_ppi_enrichment(
    args: &StringPpiEnrichmentArgs,
    output: &OutputArgs,
) -> Result<()> {
    let mut dataframe = string_ppi_enrichment(args)?;
    let output_handle = output.get_writer()?;
    write_dataframe(output_handle, &mut dataframe, output.format.into())?;
    Ok(())
}
