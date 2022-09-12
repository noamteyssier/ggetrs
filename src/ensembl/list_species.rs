use anyhow::Result;
use ftp::FtpStream;
use super::DataType;

pub fn list_species(release: usize, datatype: &DataType) -> Result<Vec<String>> {
    let site = "ftp.ensembl.org:21";
    let mut stream = FtpStream::connect(site)?;
    stream.login("anonymous", "anonymous")?;
    stream.cwd("pub")?;

    let dirname = format!("release-{}/{}/", release, datatype.directory());
    let filelist = stream.nlst(Some(&dirname))?
        .iter()
        .map(|x| x.strip_prefix(&dirname).unwrap())
        .map(String::from)
        .collect::<Vec<String>>();

    Ok(filelist)
}


