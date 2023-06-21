use crate::utils::download_multiple;

use super::{
    database, functions::lookup_id, list_species, lookup_symbol, reference, release, search,
    DataType,
};
use std::{fs::File, io::Write};

/// Main entrypoint for `Ensembl` description search
pub fn launch_ensembl_search(
    search_terms: &[String],
    database: &Option<String>,
    species: &str,
    db_type: &str,
    release: &usize,
    assembly: &str,
    output: &Option<String>,
) -> anyhow::Result<()> {
    let db_name = match database {
        Some(name) => name.clone(),
        None => format!("{}_{}_{}_{}", species, db_type, release, assembly),
    };
    let results = search(&db_name, search_terms)?;
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

/// Main entrypoint for `Ensembl` SQL database list
pub fn launch_ensembl_database(
    filter: &Option<String>,
    output: &Option<String>,
) -> anyhow::Result<()> {
    let results = database(filter)?;
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

/// Main entrypoint for `Ensembl` release version
pub fn launch_ensembl_release() -> anyhow::Result<()> {
    let result = release()?;
    println!("release: {}", result);
    Ok(())
}

/// Main entrypoint for `Ensembl` FTP query
pub fn launch_ensembl_reference(
    species: &str,
    release: usize,
    datatype: &[DataType],
    download: bool,
    output: &Option<String>,
) -> anyhow::Result<()> {
    let files = reference(species, release, datatype)?;

    let repr = serde_json::to_string_pretty(&files)?;
    match output {
        Some(path) => {
            if let Ok(mut writer) = File::create(path) {
                writeln!(writer, "{}", repr).expect("Unable to write to file");
            } else {
                println!("{}", repr);
            }
        }
        None => {
            println!("{}", repr);
        }
    }
    if download {
        eprintln!("Downloading {} files:", files.len());
        let urls = files.iter().map(|f| f.url.as_str()).collect::<Vec<&str>>();
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;
        rt.block_on(async move {
            download_multiple(&urls)
                .await
                .expect("Unable to download files");
        });
    }
    Ok(())
}

/// Main entrypoint for `Ensembl` FTP species list
pub fn launch_ensembl_list_species(
    release: usize,
    datatype: &DataType,
    output: &Option<String>,
) -> anyhow::Result<()> {
    let species = list_species(release, datatype)?;
    let repr = serde_json::to_string_pretty(&species)?;
    match output {
        Some(path) => {
            if let Ok(mut writer) = File::create(path) {
                writeln!(writer, "{}", repr).expect("Unable to write to file");
            } else {
                println!("{}", repr);
            }
        }
        None => {
            println!("{}", repr);
        }
    }
    Ok(())
}

/// Main entrypoint for `Ensembl` lookup id
pub fn launch_ensembl_lookup_id(
    ensembl_ids: &[String],
    names: bool,
    output: &Option<String>,
) -> anyhow::Result<()> {
    let results = lookup_id(ensembl_ids)?;

    let output_str = if names {
        ensembl_ids
            .iter()
            .filter_map(|x| results.get_symbol(x))
            .enumerate()
            .fold(String::new(), |mut s, (idx, i)| {
                if idx == 0 {
                    s.push_str(&format!("{i}"))
                } else {
                    s.push_str(&format!(" {i}"))
                }
                s
            })
    } else {
        format!("{}", results)
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

/// Main entrypoint for `Ensembl` lookup symbol
pub fn launch_ensembl_lookup_symbol(
    symbols: &[String],
    species: &str,
    ids: bool,
    output: &Option<String>,
) -> anyhow::Result<()> {
    let results = lookup_symbol(symbols, species)?;

    let output_str = if ids {
        symbols
            .iter()
            .filter_map(|x| results.get_id(x))
            .enumerate()
            .fold(String::new(), |mut s, (idx, i)| {
                if idx == 0 {
                    s.push_str(&format!("{i}"))
                } else {
                    s.push_str(&format!(" {i}"))
                }
                s
            })
    } else {
        format!("{}", results)
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
