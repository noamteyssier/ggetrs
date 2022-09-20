use crate::{ensembl::types::DataType, utils::ping};
use anyhow::{bail, Result};
use ftp::FtpStream;

/// List all available species for the provided release and datatype
pub fn list_species(release: usize, datatype: &DataType) -> Result<Vec<String>> {
    let site = "ftp.ensembl.org:21";
    if !ping("https://ftp.ensembl.org", 3) {
        bail!("Ensembl ftp site is inaccessible. Try again later")
    }
    let mut stream = FtpStream::connect(site)?;
    stream.login("anonymous", "anonymous")?;
    stream.cwd("pub")?;

    let dirname = format!("release-{}/{}/", release, datatype.directory());
    let filelist = stream
        .nlst(Some(&dirname))?
        .iter()
        .map(|x| x.strip_prefix(&dirname).unwrap())
        .map(String::from)
        .collect::<Vec<String>>();

    Ok(filelist)
}

#[cfg(test)]
mod testing {
    use super::list_species;
    use crate::{
        ensembl::{DataType, ENSEMBL_RELEASE},
        utils::ping,
    };

    #[test]
    fn test_list_species() {
        if ping("ftp.ensembl.org", 3) {
            let release = ENSEMBL_RELEASE;
            let datatype = DataType::DNA;
            let response = list_species(release, &datatype).unwrap();
            assert!(response.len() > 1);
        } else {
            // ensembl ftp is currently down - skip check
            assert!(true)
        }
    }
}
