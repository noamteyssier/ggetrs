use super::{
    functions::blast,
    types::{BlastDatabase, BlastProgram},
};
use anyhow::Result;
use std::{fs::File, io::Write};

pub fn launch_blast(
    query: &str,
    program: &Option<BlastProgram>,
    database: &Option<BlastDatabase>,
    limit: usize,
    expect: f64,
    low_comp_filter: bool,
    megablast: bool,
    output: &Option<String>,
) -> Result<()> {
    let results = blast(
        query,
        program,
        database,
        limit,
        expect,
        low_comp_filter,
        megablast,
    )?;
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
