use anyhow::{Result, bail};
use ftp::FtpStream;

use crate::{
    types::{DataType, FtpFile},
    utils::ping,
};

/// Queries a set of datatypes from Ensembl FTP
pub fn reference(species: &str, release: usize, datatype: &[DataType]) -> Result<Vec<FtpFile>> {
    let site = "ftp.ensembl.org:21";
    if !ping("https://ftp.ensembl.org", 3) {
        bail!("Ensembl ftp site is inaccessible. Try again later")
    }
    let mut stream = FtpStream::connect(site)?;
    stream.login("anonymous", "anonymous")?;
    stream.cwd("pub")?;

    let filepaths = datatype
        .iter()
        .filter_map(|d| show_data(&mut stream, species, release, d).expect("could not query ftp"))
        .collect::<Vec<String>>();
    Ok(filepaths
        .iter()
        .map(|path| {
            FtpFile::new(&mut stream, path, release).expect("Could not represent file path")
        })
        .collect())
}

/// Searches through filelists to recover an expected file format
fn find_data(filelist: &[String], release: usize, datatype: &DataType) -> Option<String> {
    for substring in datatype.expected_substring(release) {
        if let Some(s) = filelist.iter().find(|x| x.contains(&substring)) {
            return Some(s.clone());
        }
    }
    None
}

/// Recovers a specific datatype from Ensembl FTP
fn show_data(
    stream: &mut FtpStream,
    species: &str,
    release: usize,
    datatype: &DataType,
) -> Result<Option<String>> {
    let dirname = if let Some(subdir) = datatype.subdirectory() {
        format!(
            "release-{release}/{}/{species}/{subdir}/",
            datatype.directory(),
        )
    } else {
        format!("release-{release}/{}/{species}/", datatype.directory())
    };
    let filelist = stream.nlst(Some(&dirname))?;
    let filename = find_data(&filelist, release, datatype);
    Ok(filename)
}

#[cfg(test)]
mod testing {
    use crate::utils::ping;

    use super::{DataType, reference};

    #[test]
    pub fn test_reference_single() {
        if ping("ftp.ensembl.org", 3) {
            let species = "homo_sapiens";
            let release = 107;
            let datatype = vec![DataType::DNA];
            let results = reference(species, release, &datatype).unwrap();
            assert_eq!(results.len(), 1);
            assert_eq!(results[0].ensembl_release, 107);
            assert_eq!(
                results[0].url,
                "http://ftp.ensembl.org/pub/release-107/fasta/homo_sapiens/dna/Homo_sapiens.GRCh38.dna.primary_assembly.fa.gz"
            );
        }
    }

    #[test]
    pub fn test_reference_multi() {
        if ping("ftp.ensembl.org", 3) {
            let species = "homo_sapiens";
            let release = 107;
            let datatype = vec![DataType::DNA, DataType::GTF];
            let results = reference(species, release, &datatype).unwrap();
            assert_eq!(results.len(), 2);
            assert_eq!(results[0].ensembl_release, 107);
            assert_eq!(
                results[0].url,
                "http://ftp.ensembl.org/pub/release-107/fasta/homo_sapiens/dna/Homo_sapiens.GRCh38.dna.primary_assembly.fa.gz"
            );
            assert_eq!(results[1].ensembl_release, 107);
            assert_eq!(
                results[1].url,
                "http://ftp.ensembl.org/pub/release-107/gtf/homo_sapiens/Homo_sapiens.GRCh38.107.gtf.gz"
            );
        }
    }
}
