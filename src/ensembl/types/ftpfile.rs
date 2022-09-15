use crate::constants::convert_mem_label;
use anyhow::Result;
use ftp::FtpStream;
use pyo3::{types::PyDict, PyResult, Python};
use serde::{Deserialize, Serialize};
use std::fmt;

/// A representation of a FTP file
#[derive(Serialize, Deserialize, Debug)]
pub struct FtpFile {
    pub url: String,
    pub ensembl_release: usize,
    pub release_date: String,
    pub release_time: String,
    pub bytes: String,
}
impl fmt::Display for FtpFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).expect("cannot serialize")
        )
    }
}
impl FtpFile {
    pub fn new(stream: &mut FtpStream, path: &str, release: usize) -> Result<Self> {
        let size = stream.size(path)?.unwrap();
        let modtime = stream.mdtm(path)?.unwrap();
        let url = format!("http://ftp.ensembl.org/pub/{}", path);
        Ok(Self {
            url,
            ensembl_release: release,
            release_date: modtime.date().to_string(),
            release_time: modtime.time().to_string(),
            bytes: convert_mem_label(size),
        })
    }

    pub fn as_pydict<'py>(&self, py: Python<'py>) -> PyResult<&'py PyDict> {
        let dict = PyDict::new(py);
        dict.set_item("url", &self.url)?;
        dict.set_item("ensembl_release", self.ensembl_release)?;
        dict.set_item("release_date", &self.release_date)?;
        dict.set_item("release_time", &self.release_time)?;
        dict.set_item("bytes", &self.bytes)?;
        Ok(dict)
    }
}
