use std::fs::File;
use std::io::Write;

use super::{add_list, enrich, functions::add_background, get_libraries};
use anyhow::{bail, Result};

/// Main entrypoint to launching the `enrich` function for `Enrichr`
pub fn launch_enrichr(
    library: &str,
    background: &Option<Vec<String>>,
    gene_list: &[String],
    output: &Option<String>,
) -> Result<()> {
    // entrypoint for enrichr with background
    let results = if let Some(background_set) = background {
        let background_id = add_background(background_set)?;
        let add_list = add_list(gene_list, true)?;
        let missing_genes = gene_list
            .iter()
            .filter(|x| !background_set.contains(x))
            .collect::<Vec<&String>>();
        if !missing_genes.is_empty() {
            eprintln!(
                "The following genes were not found in the background set: {missing_genes:?}",
            );
            bail!("Unable to find all genes in background set");
        }
        enrich(
            add_list.user_list_id,
            library,
            Some(&background_id.backgroundid),
        )?
    // standard entrypoint for enrichr
    } else {
        let add_list = add_list(gene_list, false)?;
        enrich(add_list.user_list_id, library, None)?
    };

    match output {
        Some(path) => {
            if let Ok(mut writer) = File::create(path) {
                writeln!(writer, "{results}").expect("Unable to write to file");
            } else {
                println!("{results}");
            }
        }
        None => {
            println!("{results}");
        }
    }
    Ok(())
}

/// Main entrypoint for listing all available libraries in `Enrichr`
pub fn launch_enrichr_list(
    minimal: bool,
    list_categories: bool,
    category: &Option<usize>,
    output: &Option<String>,
) -> Result<()> {
    let libraries = get_libraries()?;

    let output_str = if list_categories {
        format!("{}", libraries.categories())
    } else {
        let libs = if let Some(cid) = category {
            libraries.filter_categories(*cid)
        } else {
            libraries.libraries()
        };
        if minimal {
            libs.minimal().to_string()
        } else {
            format!("{libs}")
        }
    };

    match output {
        Some(path) => {
            if let Ok(mut writer) = File::create(path) {
                writeln!(writer, "{output_str}").expect("Unable to write to file");
            } else {
                println!("{output_str}");
            }
        }
        None => {
            println!("{output_str}");
        }
    }

    Ok(())
}
