use anyhow::Result;

use super::functions::activity;

pub fn launch_chembl_activity(query: &str, output: &Option<String>) -> Result<()> {
    activity(query)?;
    Ok(())
}
