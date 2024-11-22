use crate::cli::{
    StringFunctionalAnnotationArgs, StringFunctionalEnrichmentArgs, StringHomologyArgs,
    StringInteractionsArgs, StringMappingArgs, StringNetworkArgs, StringPpiEnrichmentArgs,
};
use anyhow::Result;
use polars_core::prelude::*;
use polars_io::prelude::*;
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

pub fn string_enrichment(args: &StringFunctionalEnrichmentArgs) -> Result<DataFrame> {
    let data = args.build_post();
    string_api_tsv("enrichment", &data)
}

pub fn string_annotations(args: &StringFunctionalAnnotationArgs) -> Result<DataFrame> {
    let data = args.build_post();
    string_api_tsv("functional_annotation", &data)
}

pub fn string_ppi_enrichment(args: &StringPpiEnrichmentArgs) -> Result<DataFrame> {
    let data = args.build_post();
    string_api("ppi_enrichment", &data)
}

#[cfg(test)]
mod testing {
    use super::*;

    fn identifiers() -> Vec<String> {
        vec!["RFX3".to_string(), "RFX2".to_string()]
    }

    #[test]
    fn test_string_network() -> Result<()> {
        let args = StringNetworkArgs::builder()
            .identifiers(identifiers())
            .build();
        let network = string_network(&args)?;
        let expected_column_names = vec![
            "stringId_A",
            "stringId_B",
            "preferredName_A",
            "preferredName_B",
            "ncbiTaxonId",
            "score",
            "nscore",
            "fscore",
            "pscore",
            "ascore",
            "escore",
            "dscore",
            "tscore",
        ];
        assert_eq!(network.get_column_names(), expected_column_names);
        Ok(())
    }

    #[test]
    fn test_string_homology() -> Result<()> {
        let args = StringHomologyArgs::builder()
            .identifiers(identifiers())
            .build();
        let homology = string_homology(&args)?;
        let expected_column_names = vec![
            "ncbiTaxonId_A",
            "stringId_A",
            "ncbiTaxonId_B",
            "stringId_B",
            "bitscore",
        ];
        assert_eq!(homology.get_column_names(), expected_column_names);
        Ok(())
    }

    #[test]
    fn test_string_map_ids() -> Result<()> {
        let args = StringMappingArgs::builder()
            .identifiers(identifiers())
            .build();
        let mapping = string_mapping(&args)?;
        let expected_column_names = vec![
            "queryIndex",
            "queryItem",
            "stringId",
            "ncbiTaxonId",
            "taxonName",
            "preferredName",
            "annotation",
        ];
        assert_eq!(mapping.get_column_names(), expected_column_names);
        Ok(())
    }

    #[test]
    fn test_string_interactions() -> Result<()> {
        let args = StringInteractionsArgs::builder()
            .identifiers(identifiers())
            .build();
        let interactions = string_interactions(&args)?;
        let expected_column_names = vec![
            "stringId_A",
            "stringId_B",
            "preferredName_A",
            "preferredName_B",
            "ncbiTaxonId",
            "score",
            "nscore",
            "fscore",
            "pscore",
            "ascore",
            "escore",
            "dscore",
            "tscore",
        ];
        assert_eq!(interactions.get_column_names(), expected_column_names);
        Ok(())
    }

    #[test]
    fn test_string_enrichment() -> Result<()> {
        let args = StringFunctionalEnrichmentArgs::builder()
            .identifiers(identifiers())
            .build();
        let enrichment = string_enrichment(&args)?;
        let expected_column_names = vec![
            "category",
            "term",
            "number_of_genes",
            "number_of_genes_in_background",
            "ncbiTaxonId",
            "inputGenes",
            "preferredNames",
            "p_value",
            "fdr",
            "description",
        ];
        assert_eq!(enrichment.get_column_names(), expected_column_names);
        Ok(())
    }

    #[test]
    fn test_string_annotations() -> Result<()> {
        let args = StringFunctionalAnnotationArgs::builder()
            .identifiers(identifiers())
            .build();
        let annotations = string_annotations(&args)?;
        let expected_column_names = vec![
            "category",
            "term",
            "number_of_genes",
            "ratio_in_set",
            "ncbiTaxonId",
            "inputGenes",
            "preferredNames",
            "description",
        ];
        assert_eq!(annotations.get_column_names(), expected_column_names);
        Ok(())
    }

    #[test]
    fn test_string_ppi_enrichment() -> Result<()> {
        let args = StringPpiEnrichmentArgs::builder()
            .identifiers(identifiers())
            .build();
        let enrichment = string_ppi_enrichment(&args)?;
        let expected_column_names = vec![
            "number_of_nodes",
            "number_of_edges",
            "average_node_degree",
            "local_clustering_coefficient",
            "expected_number_of_edges",
            "p_value",
        ];
        assert_eq!(enrichment.get_column_names(), expected_column_names);
        Ok(())
    }
}
