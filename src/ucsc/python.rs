use super::{functions::blat, types::SeqType};
use anyhow::{bail, Result};
use clap::ValueEnum;
use pyo3::{
    pyfunction,
    types::{PyList, PyModule},
    wrap_pyfunction, PyResult, Python,
};

#[pyfunction(name = "blat")]
#[pyo3(text_signature = "(sequence, seqtype = 'dna', db_name = 'hg38')")]
pub fn python_ucsc_blat<'py>(
    py: Python<'py>,
    sequence: &str,
    seqtype: Option<String>,
    db_name: Option<String>,
) -> Result<&'py PyList> {
    // match provided seqtype against known `SeqType`
    let seqtype = if let Some(st) = seqtype {
        if let Ok(s) = SeqType::from_str(&st, true) {
            s
        } else {
            bail!(format!(
                "Could not interpret provided seqtype: {st}. Please select from {:?}",
                vec!["dna", "protein", "translated_dna", "translated_rna"]
            ))
        }
    } else {
        SeqType::Dna
    };

    // Unwrap `db_name` or assign hg38 default
    let db_name = db_name.unwrap_or("hg38".to_string());

    // perform BLAT
    let results = blat(sequence, &seqtype, &db_name)?;

    // convert to list and return
    let list = results.as_pylist(py)?;

    Ok(list)
}

pub fn python_ucsc(py: Python<'_>, module: &PyModule) -> PyResult<()> {
    let submodule = PyModule::new(py, "ucsc")?;
    submodule.add_function(wrap_pyfunction!(python_ucsc_blat, module)?)?;
    module.add_submodule(submodule)?;
    Ok(())
}
