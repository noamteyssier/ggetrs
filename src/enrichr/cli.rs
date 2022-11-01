use std::fs::File;
use std::io::Write;

use super::{add_list, enrich, get_libraries};
use reqwest::Result;

/// Main entrypoint to launching the `enrich` function for `Enrichr`
pub fn launch_enrichr(library: &str, gene_list: &[String], output: &Option<String>) -> Result<()> {
    let add_list = add_list(gene_list)?;
    let results = enrich(add_list.user_list_id, library)?;
    match output {
        Some(path) => {
            if let Ok(mut writer) = File::create(path) {
                writeln!(writer, "{}", results).expect("Unable to write to file");
            } else {
                println!("{}", results);
            }
        }
        None => {
            println!("{}", results);
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
            format!("{}", libs.minimal())
        } else {
            format!("{}", libs)
        }
    };

    match output {
        Some(path) => {
            if let Ok(mut writer) = File::create(path) {
                writeln!(writer, "{}", output_str).expect("Unable to write to file");
            } else {
                println!("{}", output_str);
            }
        }
        None => {
            println!("{}", output_str);
        }
    }

    Ok(())
}
