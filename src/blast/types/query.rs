use crate::blast::{
    functions::{parse_rid, parse_rtoe, parse_status},
    types::BlastResult,
};
use anyhow::Result;
use chrono::Utc;
use reqwest::blocking::Client;

use super::{BlastDatabase, BlastProgram, BlastStatus};

#[derive(Debug, Default)]
pub struct BlastQuery {
    program: BlastProgram,
    database: BlastDatabase,
    query: String,
    limit: usize,
    expect: f64,
    low_comp_filter: bool,
    megablast: bool,
    rid: String,
    rtoe: usize,
}
impl BlastQuery {
    pub fn new(
        program: BlastProgram,
        database: BlastDatabase,
        query: &str,
        limit: usize,
        expect: f64,
        low_comp_filter: bool,
        megablast: bool,
    ) -> Result<Self> {
        let mut query = Self {
            program,
            database,
            query: query.to_string(),
            limit,
            expect,
            low_comp_filter,
            megablast,
            rid: String::new(),
            rtoe: 0,
        };
        query.launch_query()?;
        Ok(query)
    }
    #[allow(unused)]
    pub fn rid(&self) -> &str {
        &self.rid
    }
    #[allow(unused)]
    pub fn rtoe(&self) -> usize {
        self.rtoe
    }
    fn launch_query(&mut self) -> Result<()> {
        let url = "https://blast.ncbi.nlm.nih.gov/blast/Blast.cgi";
        let put_url = format!(
            "{url}?CMD=Put&PROGRAM={}&DATABASE={}&FILTER={}&EXPECT={}&MEGABLAST={}&LIMIT={}&HITLIST_SIZE={}&QUERY={}",
            self.program.to_string(),
            self.database.to_string(),
            if self.low_comp_filter {"T"} else {"F"},
            self.expect,
            if self.megablast {"on"} else {"off"},
            self.limit,
            self.limit,
            self.query,
        );
        // println!("query: {}", put_url);
        let response = Client::new()
            .get(put_url)
            .header("User-Agent", format!("ggetrs_{}", Utc::now().to_rfc3339()))
            .send()?
            .text()?;
        self.rid = parse_rid(&response)?;
        self.rtoe = parse_rtoe(&response)?;
        Ok(())
    }

    pub fn status(&self) -> Result<BlastStatus> {
        let url = "https://blast.ncbi.nlm.nih.gov/Blast.cgi?CMD=Get";
        let poll_url = format!("{url}&RID={}&FORMAT_OBJECT=SearchInfo", self.rid);
        let response = Client::new()
            .get(poll_url)
            .header("User-Agent", format!("ggetrs_{}", Utc::now().to_rfc3339()))
            .send()?
            .text()?;
        parse_status(&response)
    }

    pub fn get(&self) -> Result<BlastResult> {
        let url = "https://blast.ncbi.nlm.nih.gov/Blast.cgi?CMD=Get";
        let poll_url = format!("{url}&RID={}&FORMAT_TYPE=XML", self.rid);
        let response = Client::new()
            .get(poll_url)
            .header("User-Agent", format!("ggetrs_{}", Utc::now().to_rfc3339()))
            .send()?
            .text()?;
        let output = serde_xml_rs::from_str(&response)?;
        let result = BlastResult::from_blast_output(&output, &self.query);
        Ok(result)
    }
}

#[cfg(test)]
mod testing {
    use crate::blast::types::{BlastDatabase, BlastProgram};

    use super::BlastQuery;

    #[test]
    fn test_blast_query() {
        let sequence = "ATACTCAGTCACACAAGCCATAGCAGGAAACAGCGAGCTTGCAGCCTCACCGACGAGTCTCAACTAAAAGGGACTCCCGGAGCTAGGGGTGGGGACTCGGCCTCACACAGTGAGTGCCGG";
        let program = BlastProgram::from_sequence(&sequence).unwrap();
        let database = BlastDatabase::from_program(&program);
        let query = BlastQuery::new(program, database, sequence, 1, 10.0, false, true).unwrap();
        assert!(!query.rid.is_empty());
        assert!(!query.rtoe != 0);
    }

    #[test]
    fn test_blast_status() {
        let sequence = "ATACTCAGTCACACAAGCCATAGCAGGAAACAGCGAGCTTGCAGCCTCACCGACGAGTCTCAACTAAAAGGGACTCCCGGAGCTAGGGGTGGGGACTCGGCCTCACACAGTGAGTGCCGG";
        let program = BlastProgram::from_sequence(&sequence).unwrap();
        let database = BlastDatabase::from_program(&program);
        let query = BlastQuery::new(program, database, sequence, 1, 10.0, false, true).unwrap();
        query.status().unwrap();
    }
}
