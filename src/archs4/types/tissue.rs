use clap::clap_derive::ValueEnum;
use pyo3::{types::PyDict, PyResult, Python};
use serde::Serialize;
use std::fmt;

/// The currently supported species for tissue expression in `ARCHS4`
#[derive(ValueEnum, Debug, Clone, Default)]
pub enum Species {
    #[default]
    Human,
    Mouse,
}
impl fmt::Display for Species {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Human => "human",
                Self::Mouse => "mouse",
            }
        )
    }
}

/// A struct to hold the responses from tissue expression
#[derive(Serialize, Debug)]
pub struct ResponseTissue {
    pub results: Vec<ResultTissue>,
}
impl fmt::Display for ResponseTissue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).expect("cannot serialize")
        )
    }
}
impl ResponseTissue {
    #[must_use]
    pub fn from_str(response: &str) -> Self {
        let results = Self::parse_str(response);
        Self { results }
    }
    fn parse_str(response: &str) -> Vec<ResultTissue> {
        response
            .split('\n')
            .filter_map(ResultTissue::from_line)
            .collect()
    }
    pub fn as_pydict<'py>(&self, py: Python<'py>) -> PyResult<&'py PyDict> {
        let dict = PyDict::new(py);
        dict.set_item(
            "tissues",
            self.results
                .iter()
                .map(|x| x.as_pydict(py).expect("could not create pydict"))
                .collect::<Vec<&PyDict>>(),
        )?;
        Ok(dict)
    }
}

/// Individual tissue responses
#[derive(Serialize, Debug)]
pub struct ResultTissue {
    pub id: String,
    pub min: f64,
    pub q1: f64,
    pub median: f64,
    pub q3: f64,
    pub max: f64,
    pub color: String,
}
impl fmt::Display for ResultTissue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).expect("cannot serialize")
        )
    }
}
impl ResultTissue {
    fn parse_float(record: Option<&str>) -> Option<f64> {
        match record {
            Some(value) => match value.parse::<f64>() {
                Ok(x) => Some(x),
                Err(_) => None,
            },
            None => None,
        }
    }

    #[must_use]
    pub fn from_line(line: &str) -> Option<Self> {
        let mut records = line.split(',');
        let id = match records.next() {
            Some(value) => value.to_string(),
            None => return None,
        };
        let min = match Self::parse_float(records.next()) {
            Some(value) => value,
            None => return None,
        };
        let q1 = match Self::parse_float(records.next()) {
            Some(value) => value,
            None => return None,
        };
        let median = match Self::parse_float(records.next()) {
            Some(value) => value,
            None => return None,
        };
        let q3 = match Self::parse_float(records.next()) {
            Some(value) => value,
            None => return None,
        };
        let max = match Self::parse_float(records.next()) {
            Some(value) => value,
            None => return None,
        };
        let color = match records.next() {
            Some(value) => value.to_string(),
            None => return None,
        };
        Some(Self {
            id,
            min,
            q1,
            median,
            q3,
            max,
            color,
        })
    }

    pub fn as_pydict<'py>(&self, py: Python<'py>) -> PyResult<&'py PyDict> {
        let dict = PyDict::new(py);
        dict.set_item("id", &self.id)?;
        dict.set_item("min", self.min)?;
        dict.set_item("q1", self.q1)?;
        dict.set_item("median", self.median)?;
        dict.set_item("q3", self.q3)?;
        dict.set_item("max", self.max)?;
        dict.set_item("color", &self.color)?;
        Ok(dict)
    }
}
