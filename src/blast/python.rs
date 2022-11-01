use super::{
    functions::blast,
    types::{BlastDatabase, BlastProgram},
};
use anyhow::{bail, Result};
use clap::ValueEnum;
use pyo3::{
    pyfunction,
    types::{IntoPyDict, PyDict},
    Python,
};

#[pyfunction(name = "blast")]
#[pyo3(
    text_signature = "(query, program = None, database = None, limit = 50, expect = 10.0, low_comp_filter = False, megablast = True)"
)]
pub fn python_blast<'py>(
    py: Python<'py>,
    query: &str,
    program: Option<String>,
    database: Option<String>,
    limit: Option<usize>,
    expect: Option<f64>,
    low_comp_filter: Option<bool>,
    megablast: Option<bool>,
) -> Result<&'py PyDict> {
    let program = match program {
        Some(program_str) => {
            if let Ok(s) = BlastProgram::from_str(&program_str, true) {
                Some(s)
            } else {
                bail!("Could not assign blast program from input")
            }
        }
        None => None,
    };

    let database = match database {
        Some(database_str) => {
            if let Ok(s) = BlastDatabase::from_str(&database_str, true) {
                Some(s)
            } else {
                bail!("Could not assign blast database from input")
            }
        }
        None => None,
    };

    let limit = limit.unwrap_or(50);
    let expect = expect.unwrap_or(10.0);
    let low_comp_filter = low_comp_filter.unwrap_or(false);
    let megablast = megablast.unwrap_or(true);
    let response = blast(
        query,
        &program,
        &database,
        limit,
        expect,
        low_comp_filter,
        megablast,
    )?;
    Ok(response.into_py_dict(py))
}
