use anyhow::{bail, Result};
use chrono::Local;
use crate::blast::types::{BlastStatus, BlastProgram, BlastDatabase, BlastQuery, BlastResult};

pub fn blast(
    query: &str, 
    program: &Option<BlastProgram>, 
    database: &Option<BlastDatabase>, 
    limit: usize, 
    expect: f64, 
    low_comp_filter: bool, 
    megablast: bool
) -> Result<BlastResult> {
    let program = match program {
        Some(p) => *p,
        None => BlastProgram::from_sequence(query)?
    };
    let database = match database {
        Some(d) => *d,
        None => BlastDatabase::from_program(&program)
    };
    let query = BlastQuery::new(program, database, query, limit, expect, low_comp_filter, megablast)?;
    eprintln!("{:#?}", query);

    let mut idx = 0;
    eprintln!("[{}] Request ID: {} :: Estimated Time: {}", Local::now().to_rfc2822(), query.rid(), query.rtoe());
    std::thread::sleep(std::time::Duration::from_secs(5));
    loop {
        idx += 1;
        eprint!("[{}] Request ID: {} :: Loop {idx}: ", Local::now().to_rfc2822(), query.rid());
        match query.status()? {
            BlastStatus::Waiting => {
                eprintln!("Waiting (Will Poll again in 1 min)");
                std::thread::sleep(std::time::Duration::from_secs(60));
                continue
            },
            BlastStatus::Ready => {
                eprintln!("Ready");
                break
            },
            BlastStatus::Unknown => {
                eprintln!("Unknown");
                bail!("Status is unknown - request likely failed" )
            }
        }
    }
    query.get()
}
