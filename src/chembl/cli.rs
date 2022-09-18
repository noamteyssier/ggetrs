use anyhow::Result;
use std::{fs::File, io::Write};
use super::functions::activity;

pub fn launch_chembl_activity(query: &str, limit: usize, output: &Option<String>) -> Result<()> {
    let results = activity(query, limit)?;
    match output {
        Some(path) => {
            if let Ok(mut writer) = File::create(path) {
                writeln!(writer, "{}", results)?;
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
