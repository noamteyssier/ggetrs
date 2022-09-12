use anyhow::Result;
use clap::clap_derive::ValueEnum;
use ftp::FtpStream;
use serde::{Deserialize, Serialize};
use crate::constants::convert_mem_label;
use pyo3::{Python, PyResult, types::PyDict};
use std::fmt;

/// A representation of a FTP file
#[derive(Serialize, Deserialize, Debug)]
pub struct FtpFile {
    url: String,
    ensembl_release: usize,
    release_date: String,
    release_time: String,
    bytes: String
}
impl fmt::Display for FtpFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).expect("cannot serialize"))
    }
}
impl FtpFile {
    pub fn new(stream: &mut FtpStream, path: &str, release: usize) -> Result<Self> {
        let size = stream.size(&path)?.unwrap();
        let modtime = stream.mdtm(&path)?.unwrap();
        let url = format!("http://ftp.ensembl.org/pub/{}", path);
        Ok(Self {
            url,
            ensembl_release: release,
            release_date: modtime.date().to_string(),
            release_time: modtime.time().to_string(),
            bytes: convert_mem_label(size)
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

/// The different data types present within the Ensembl FTP
#[derive(ValueEnum, Clone, Debug)]
pub enum DataType {
    CDNA, CDS, DNA, GFF3,
    GTF, NCRNA, PEP,
}
impl DataType {
    pub fn directory(&self) -> &str {
        match self {
            Self::GTF => "gtf",
            Self::GFF3 => "gff3",
            _ => "fasta"
        }
    }

    pub fn subdirectory(&self) -> Option<&str> {
        match self {
            Self::CDNA => Some("cdna"),
            Self::CDS => Some("cds"),
            Self::DNA => Some("dna"),
            Self::NCRNA => Some("ncrna"),
            Self::PEP => Some("pep"),
            _ => None
        }
    }

    pub fn expected_substring(&self, release: usize) -> Vec<String> {
        match self {
            Self::CDNA => vec![".cdna.all.fa"].iter().map(|x| x.to_string()).collect(),
            Self::CDS => vec![".cds.all.fa"].iter().map(|x| x.to_string()).collect(),
            Self::DNA => vec![".dna.primary_assembly.fa", ".dna.toplevel.fa"].iter().map(|x| x.to_string()).collect(),
            Self::GFF3 => vec![format!("{}.gff3.gz", release)],
            Self::GTF => vec![format!("{}.gtf.gz", release)],
            Self::NCRNA => vec![".ncrna.fa"].iter().map(|x| x.to_string()).collect(),
            Self::PEP => vec![".pep.all.fa"].iter().map(|x| x.to_string()).collect(),
        }
    }
}

/// Searches through filelists to recover an expected file format
pub fn find_data(filelist: &Vec<String>, release: usize, datatype: &DataType) -> Option<String> {
    for substring in datatype.expected_substring(release) {
        match filelist.iter().filter(|x| x.contains(&substring)).next() {
            Some(s) => return Some(s.clone()),
            None => {}
        }
    }
    None
}

/// Recovers a specific datatype from Ensembl FTP
pub fn show_data(stream: &mut FtpStream, species: &str, release: usize, datatype: &DataType) -> Result<Option<String>> {
    let dirname = match datatype.subdirectory() {
        Some(subdir) => format!("release-{}/{}/{}/{}/", release, datatype.directory(), species, subdir),
        None => format!("release-{}/{}/{}/", release, datatype.directory(), species)
    };
    let filelist = stream.nlst(Some(&dirname))?;
    let filename = find_data(&filelist, release, datatype);
    Ok(filename)
}


/// Queries a set of datatypes from Ensembl FTP
pub fn reference(species: &str, release: usize, datatype: &[DataType]) -> Result<Vec<FtpFile>> {
    let site = "ftp.ensembl.org:21";
    let mut stream = FtpStream::connect(site)?;
    stream.login("anonymous", "anonymous")?;
    stream.cwd("pub")?;

    let filepaths = datatype.iter()
            .filter_map(|d| show_data(&mut stream, species, release, d).expect("could not query ftp"))
            .collect::<Vec<String>>();
    Ok(filepaths.iter()
            .map(|path| FtpFile::new(&mut stream, path, release).expect("Could not represent file path"))
            .collect())
    
}

#[cfg(test)]
mod testing {
    use super::{DataType, reference};

    #[test]
    pub fn test_reference_single() {
        let species = "homo_sapiens";
        let release = 107;
        let datatype = vec![DataType::DNA];
        let results = reference(species, release, &datatype).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].ensembl_release, 107);
        assert_eq!(results[0].url, "http://ftp.ensembl.org/pub/release-107/fasta/homo_sapiens/dna/Homo_sapiens.GRCh38.dna.primary_assembly.fa.gz");
    }

    #[test]
    pub fn test_reference_multi() {
        let species = "homo_sapiens";
        let release = 107;
        let datatype = vec![DataType::DNA, DataType::GTF];
        let results = reference(species, release, &datatype).unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].ensembl_release, 107);
        assert_eq!(results[0].url, "http://ftp.ensembl.org/pub/release-107/fasta/homo_sapiens/dna/Homo_sapiens.GRCh38.dna.primary_assembly.fa.gz");
        assert_eq!(results[1].ensembl_release, 107);
        assert_eq!(results[1].url, "http://ftp.ensembl.org/pub/release-107/gtf/homo_sapiens/Homo_sapiens.GRCh38.107.gtf.gz");
    }
}
