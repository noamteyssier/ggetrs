use anyhow::Result;
use clap::clap_derive::ArgEnum;
use ftp::FtpStream;
use serde::{Deserialize, Serialize};
use crate::constants::convert_mem_label;
use std::fmt;

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
    pub fn new(stream: &mut FtpStream, path: String, release: usize) -> Result<Self> {
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
}

#[derive(ArgEnum, Clone, Debug)]
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

pub fn find_data(filelist: &Vec<String>, release: usize, datatype: &DataType) -> Option<String> {
    for substring in datatype.expected_substring(release) {
        match filelist.iter().filter(|x| x.contains(&substring)).next() {
            Some(s) => return Some(s.clone()),
            None => {}
        }
    }
    None
}

pub fn show_data(stream: &mut FtpStream, species: &str, release: usize, datatype: &DataType) -> Result<Option<String>> {
    let dirname = match datatype.subdirectory() {
        Some(subdir) => format!("release-{}/{}/{}/{}/", release, datatype.directory(), species, subdir),
        None => format!("release-{}/{}/{}/", release, datatype.directory(), species)
    };
    let filelist = stream.nlst(Some(&dirname))?;
    let filename = find_data(&filelist, release, datatype);
    Ok(filename)
}


pub fn reference(species: &str, release: usize, datatype: &DataType) -> Result<Option<FtpFile>> {
    let site = "ftp.ensembl.org:21";
    let mut stream = FtpStream::connect(site)?;
    stream.login("anonymous", "anonymous")?;
    stream.cwd("pub")?;

    let result = show_data(&mut stream, species, release, &datatype)?;
    if let Some(path) = result {
        let file = FtpFile::new(&mut stream, path, release)?;
        Ok(Some(file))
    } else {
        Ok(None)
    }

}
