use anyhow::Result;
use super::{types::{BlastProgram, BlastDatabase}, functions::blast};

pub fn launch_blast(query: &str, program: &Option<BlastProgram>, database: &Option<BlastDatabase>, limit: usize, expect: f64, low_comp_filter: bool, megablast: bool) -> Result<()> {
    let result = blast(query, program, database, limit, expect, low_comp_filter, megablast)?;
    println!("{:#?}", result);
    Ok(())
}
