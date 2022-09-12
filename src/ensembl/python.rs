use pyo3::{
    pyfunction, Python, PyResult, wrap_pyfunction,
    types::{PyDict, PyModule}
};
use super::{search, database};


#[pyfunction(name="search")]
pub fn python_ensembl_search<'py>(
        py: Python<'py>, 
        search_terms: Vec<String>, 
        database: Option<String>,
        species: Option<&str>, 
        db_type: Option<&str>, 
        release: Option<usize>, 
        assembly: Option<&str>) -> PyResult<&'py PyDict> 
{
    let db_name = match database {
        Some(name) => name.clone(),
        None => {
            let species = match species { Some(x) => x, None => "homo_sapiens" };
            let db_type = match db_type { Some(x) => x, None => "core" };
            let release = match release { Some(x) => x, None => 107 };
            let assembly = match assembly { Some(x) => x, None => "38" };
            format!("{}_{}_{}_{}", species, db_type, release, assembly)
        }
    };
    let results = search(&db_name, &search_terms).expect("Unable to query ensembl search");
    results.as_pydict(py)
}

#[pyfunction(name="database")]
pub fn python_ensembl_database<'py>(_py: Python<'py>, filter: Option<String>) -> PyResult<Vec<String>> {
    let results = database(&filter).expect("Could not query ensembl SQL");
    Ok(results.as_vec())
}

pub fn python_ensembl(py: Python<'_>, module: &PyModule) -> PyResult<()> {
    let submodule = PyModule::new(py, "ensembl")?;
    submodule.add_function(wrap_pyfunction!(python_ensembl_search, module)?)?;
    submodule.add_function(wrap_pyfunction!(python_ensembl_database, module)?)?;
    module.add_submodule(submodule)?;
    Ok(())
}
