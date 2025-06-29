use std::{fmt, str::FromStr};

#[cfg(feature = "python")]
use pyo3::{
    Bound, PyResult, Python,
    types::{PyDict, PyDictMethods},
};
use serde::Serialize;

/// The currently supported species for tissue expression in `ARCHS4`
#[derive(Debug, Clone, Default)]
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
impl FromStr for Species {
    type Err = anyhow::Error;

    fn from_str(species: &str) -> Result<Self, Self::Err> {
        match species.to_lowercase().as_str() {
            "human" => Ok(Self::Human),
            "mouse" => Ok(Self::Mouse),
            _ => Err(anyhow::anyhow!("invalid species")),
        }
    }
}
impl Species {
    pub fn from_str(species: &str) -> Option<Self> {
        match species {
            "human" => Some(Self::Human),
            "mouse" => Some(Self::Mouse),
            _ => None,
        }
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
    #[cfg(feature = "python")]
    pub fn as_pydict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item(
            "tissues",
            self.results
                .iter()
                .map(|x| x.as_pydict(py).expect("could not create pydict"))
                .collect::<Vec<Bound<'py, PyDict>>>(),
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
        let min = Self::parse_float(records.next())?;
        let q1 = Self::parse_float(records.next())?;
        let median = Self::parse_float(records.next())?;
        let q3 = Self::parse_float(records.next())?;
        let max = Self::parse_float(records.next())?;
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

    #[cfg(feature = "python")]
    pub fn as_pydict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
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
