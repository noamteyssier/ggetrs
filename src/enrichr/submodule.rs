use std::fs::File;
use std::io::Write;

use reqwest::Error;
use super::{add_list, enrich};

pub async fn launch_enrich(library: &str, gene_list: &[String], output: &Option<String>) -> Result<(), Error> {
    let add_list = add_list(&gene_list).await?;
    let results = enrich(add_list.user_list_id, &library).await?;
    match output {
        Some(path) => {
            if let Ok(mut writer) = File::create(path) {
                writeln!(writer, "{}", results).expect("Unable to write to file");
            } else {
                println!("{}", results);
            }
        },
        None => {
            println!("{}", results);
        }
    }
    Ok(())
}
