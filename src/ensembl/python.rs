use clap::ArgEnum;
use pyo3::{
    pyfunction, Python, PyResult, wrap_pyfunction,
    types::{PyDict, PyModule}
};
use super::{search, database, release, DataType, reference, ENSEMBL_RELEASE, list_species};


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
        Some(name) => name,
        None => {
            let species = species.unwrap_or("homo_sapiens");
            let db_type = db_type.unwrap_or("core");
            let release = release.unwrap_or(107);
            let assembly = assembly.unwrap_or("38");
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

#[pyfunction(name="release")]
pub fn python_ensembl_release<'py>(_py: Python<'py>) -> PyResult<usize> {
    let results = release().expect("Could not query ensembl release number");
    Ok(results)
}

#[pyfunction(name="reference")]
pub fn python_ensembl_reference<'py>(
        py: Python<'py>, 
        species: Option<&str>, 
        release: Option<usize>, 
        datatype: Option<Vec<String>>) -> PyResult<Vec<&'py PyDict>> 
{
    let species = species.unwrap_or("homo_sapiens");

    let release = match release {
        Some(x) => x,
        None => ENSEMBL_RELEASE
    };

    let datatype = match datatype {
        Some(datatype) => {
            datatype
                .iter()
                .map(|x| DataType::from_str(x, true).expect("Could not represent provided datatypes"))
                .collect::<Vec<DataType>>()
        },
        None => {
            vec![DataType::DNA]
        }
    };

    Ok(reference(species, release, &datatype).expect("Could not query FTP")
        .iter()
        .map(|x| x.as_pydict(py).expect("could not create dictionary"))
        .collect()
    )
}

#[pyfunction(name="species")]
pub fn python_ensembl_species<'py>(
        _py: Python<'py>,
        release: Option<usize>,
        datatype: Option<String>) -> PyResult<Vec<String>> 
{
    let datatype = datatype
        .map_or(
            DataType::DNA, |x| 
            DataType::from_str(&x, true)
                .expect("Unexpected datatype provided")
        );
    let release = match release {
        Some(x) => x,
        None => ENSEMBL_RELEASE
    };
    let results = list_species(release, &datatype).expect("Could not query species FTP");
    Ok(results)
}

pub fn python_ensembl(py: Python<'_>, module: &PyModule) -> PyResult<()> {
    let submodule = PyModule::new(py, "ensembl")?;
    submodule.add_function(wrap_pyfunction!(python_ensembl_search, module)?)?;
    submodule.add_function(wrap_pyfunction!(python_ensembl_database, module)?)?;
    submodule.add_function(wrap_pyfunction!(python_ensembl_release, module)?)?;
    submodule.add_function(wrap_pyfunction!(python_ensembl_reference, module)?)?;
    submodule.add_function(wrap_pyfunction!(python_ensembl_species, module)?)?;
    module.add_submodule(submodule)?;
    Ok(())
}
