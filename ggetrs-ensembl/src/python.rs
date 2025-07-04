use std::str::FromStr;

use anyhow::{Result, bail};
use pyo3::{
    Bound, PyResult, Python, pyfunction,
    types::{PyDict, PyModule, PyModuleMethods},
    wrap_pyfunction,
};

use super::{DataType, ENSEMBL_RELEASE, database, list_species, reference, release, search};

#[pyfunction(name = "search")]
#[allow(clippy::needless_pass_by_value)]
#[pyo3(signature = (search_terms, database = None, species = None, db_type = None, release = None, assembly = None))]
pub fn python_ensembl_search<'py>(
    py: Python<'py>,
    search_terms: Vec<String>,
    database: Option<String>,
    species: Option<&str>,
    db_type: Option<&str>,
    release: Option<usize>,
    assembly: Option<&str>,
) -> Result<Bound<'py, PyDict>> {
    if search_terms.is_empty() {
        bail!("Must pass in more than one search term!");
    } else if search_terms[0].len() == 1 {
        bail!("Must pass in search terms as a list!");
    }
    let db_name = if let Some(name) = database {
        name
    } else {
        let species = species.unwrap_or("homo_sapiens");
        let db_type = db_type.unwrap_or("core");
        let release = release.unwrap_or(107);
        let assembly = assembly.unwrap_or("38");
        format!("{species}_{db_type}_{release}_{assembly}")
    };
    let results = search(&db_name, &search_terms)?;
    results.as_pydict(py)
}

#[pyfunction(name = "database")]
#[must_use]
#[allow(clippy::needless_pass_by_value)]
#[pyo3(signature = (filter = None))]
pub fn python_ensembl_database(_py: Python, filter: Option<String>) -> Vec<String> {
    let results = database(&filter).expect("Could not query ensembl SQL");
    results.as_vec()
}

#[pyfunction(name = "release")]
pub fn python_ensembl_release(_py: Python) -> usize {
    release().expect("Could not query ensembl release number")
}

#[pyfunction(name = "reference")]
#[pyo3(signature = (species = None, release = None, datatype = None))]
pub fn python_ensembl_reference<'py>(
    py: Python<'py>,
    species: Option<&str>,
    release: Option<usize>,
    datatype: Option<Vec<String>>,
) -> Result<Vec<Bound<'py, PyDict>>> {
    let species = species.unwrap_or("homo_sapiens");
    let release = release.unwrap_or(ENSEMBL_RELEASE);
    let datatype = match datatype {
        Some(datatype) => {
            if datatype.is_empty() {
                bail!("Must pass in at least one datatype!");
            } else if datatype[0].len() == 1 {
                bail!("Must pass in datatypes as a list!");
            }
            datatype
                .iter()
                .map(|x| DataType::from_str(x).expect("Could not represent provided datatypes"))
                .collect::<Vec<DataType>>()
        }
        None => {
            vec![DataType::DNA]
        }
    };

    let results = reference(species, release, &datatype)
        .expect("Could not query FTP")
        .iter()
        .map(|x| x.as_pydict(py).expect("could not create dictionary"))
        .collect();

    Ok(results)
}

#[pyfunction(name = "species")]
#[pyo3(signature = (release = None, datatype = None))]
pub fn python_ensembl_species(
    _py: Python,
    release: Option<usize>,
    datatype: Option<String>,
) -> Vec<String> {
    let datatype = datatype.map_or(DataType::DNA, |x| {
        DataType::from_str(&x).expect("Unexpected datatype provided")
    });
    let release = match release {
        Some(x) => x,
        None => ENSEMBL_RELEASE,
    };
    list_species(release, &datatype).expect("Could not query species FTP")
}

pub fn python_ensembl(py: Python<'_>, module: &Bound<'_, PyModule>) -> PyResult<()> {
    let submodule = PyModule::new(py, "ensembl")?;
    submodule.add_function(wrap_pyfunction!(python_ensembl_search, module)?)?;
    submodule.add_function(wrap_pyfunction!(python_ensembl_database, module)?)?;
    submodule.add_function(wrap_pyfunction!(python_ensembl_release, module)?)?;
    submodule.add_function(wrap_pyfunction!(python_ensembl_reference, module)?)?;
    submodule.add_function(wrap_pyfunction!(python_ensembl_species, module)?)?;
    module.add_submodule(&submodule)?;
    Ok(())
}
