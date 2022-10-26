use anyhow::{bail, Result};
use regex::Regex;

pub fn parse_qblast_info(text: &str, key: &str) -> Result<String> {
    let re = Regex::new(key)?;
    match re.find(text) {
        Some(m) => {
            let value = text
                .chars()
                // Not sure why I need this offset, but won't work without it
                .skip(m.start() + key.len() - 4)
                .take_while(|c| *c != '\n')
                .fold(String::new(), |mut s, c| {
                    s.push(c);
                    s
                });
            Ok(value)
        },
        None => bail!("Could not find query string")
    }
}

pub fn parse_rid(text: &str) -> Result<String> {
    if let Ok(value) = parse_qblast_info(text, "    RID = ") {
        Ok(value)
    } else {
        bail!("No RID found in response");
    }
}

pub fn parse_rtoe(text: &str) -> Result<usize> {
    if let Ok(value) = parse_qblast_info(text, "    RTOE = ") {
        Ok(value.parse()?)
    } else {
        bail!("No RTOE found in response");
    }
}
