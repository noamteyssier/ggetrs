use std::fs::File;
use std::io::Write;

use anyhow::Result;

use super::{functions::blat, types::SeqType};

/// Main entrypoint for ucsc blat function
pub fn launch_ucsc_blat(
    sequence: &str,
    seqtype: &SeqType,
    db_name: &str,
    output: &Option<String>,
) -> Result<()> {
    let results = blat(sequence, seqtype, db_name)?;
    match output {
        Some(path) => {
            if let Ok(mut writer) = File::create(path) {
                write!(writer, "{results}")?;
            } else {
                print!("{results}");
            }
        }
        None => {
            print!("{results}");
        }
    }
    Ok(())
}
