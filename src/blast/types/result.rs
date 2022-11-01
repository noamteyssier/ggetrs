use pyo3::types::{IntoPyDict, PyDict};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct BlastResult {
    query: String,
    results: Vec<BlastHit>,
}
impl fmt::Display for BlastResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).expect("cannot serialize")
        )
    }
}
impl IntoPyDict for BlastResult {
    fn into_py_dict(self, py: pyo3::Python<'_>) -> &PyDict {
        let map = PyDict::new(py);
        map.set_item("query", self.query).unwrap();
        map.set_item(
            "results",
            self.results
                .iter()
                .map(|x| x.clone())
                .map(|x| x.into_py_dict(py))
                .collect::<Vec<&PyDict>>(),
        )
        .unwrap();
        map
    }
}
impl BlastResult {
    pub fn from_blast_output(output: &BlastOutput, query: &str) -> Self {
        Self {
            results: output
                .blast_output_iterations
                .iterations
                .iteration_hits
                .hits
                .iter()
                .map(|x| BlastHit::from_hit(x))
                .collect(),
            query: query.to_string(),
        }
    }
    pub fn query(&self) -> &str {
        &self.query
    }
    pub fn results(&self) -> &Vec<BlastHit> {
        &self.results
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlastHit {
    pub num: usize,
    pub id: String,
    pub definition: String,
    pub accession: String,
    pub length: usize,
    pub bit_score: f64,
    pub score: usize,
    pub evalue: f64,
    pub gap_opens: usize,
    pub alignment_length: usize,
    pub query_start: usize,
    pub query_end: usize,
    pub subject_start: usize,
    pub subject_end: usize,
}
impl BlastHit {
    fn from_hit(hit: &Hit) -> Self {
        let statistic = &hit.statistics.hsp[0];
        Self {
            num: hit.num,
            id: hit.id.to_string(),
            definition: hit.definition.to_string(),
            accession: hit.accession.to_string(),
            length: hit.length,
            bit_score: statistic.bit_score,
            score: statistic.score,
            evalue: statistic.evalue,
            query_start: statistic.query_start,
            query_end: statistic.query_end,
            subject_start: statistic.subject_start,
            subject_end: statistic.subject_end,
            gap_opens: statistic.gap_opens,
            alignment_length: statistic.alignment_length,
        }
    }
}
impl IntoPyDict for BlastHit {
    fn into_py_dict(self, py: pyo3::Python<'_>) -> &pyo3::types::PyDict {
        let map = PyDict::new(py);
        map.set_item("num", self.num).unwrap();
        map.set_item("id", self.num).unwrap();
        map.set_item("definition", self.num).unwrap();
        map.set_item("accession", self.num).unwrap();
        map.set_item("length", self.num).unwrap();
        map.set_item("bit_score", self.num).unwrap();
        map.set_item("score", self.num).unwrap();
        map.set_item("evalue", self.num).unwrap();
        map.set_item("gap_opens", self.num).unwrap();
        map.set_item("alignment_length", self.num).unwrap();
        map.set_item("query_start", self.num).unwrap();
        map.set_item("query_end", self.num).unwrap();
        map.set_item("subject_start", self.num).unwrap();
        map.set_item("subject_end", self.num).unwrap();
        map
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlastOutput {
    #[serde(rename = "BlastOutput_iterations")]
    blast_output_iterations: BlastOutputIterations,
}

#[derive(Debug, Serialize, Deserialize)]
struct BlastOutputIterations {
    #[serde(rename = "Iteration")]
    iterations: Iteration,
}

#[derive(Debug, Serialize, Deserialize)]
struct Iteration {
    #[serde(rename = "Iteration_iter-num")]
    iteration_iter_num: usize,
    #[serde(rename = "Iteration_query-ID")]
    iteration_query_id: String,
    #[serde(rename = "Iteration_hits")]
    iteration_hits: IterationHits,
}

#[derive(Debug, Serialize, Deserialize)]
struct IterationHits {
    #[serde(rename = "Hit")]
    hits: Vec<Hit>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Hit {
    #[serde(rename = "Hit_num")]
    num: usize,
    #[serde(rename = "Hit_id")]
    id: String,
    #[serde(rename = "Hit_def")]
    definition: String,
    #[serde(rename = "Hit_accession")]
    accession: String,
    #[serde(rename = "Hit_len")]
    length: usize,
    #[serde(rename = "Hit_hsps")]
    statistics: HitStatistics,
}

#[derive(Debug, Serialize, Deserialize)]
struct HitStatistics {
    #[serde(rename = "Hsp")]
    hsp: Vec<Hsp>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Hsp {
    #[serde(rename = "Hsp_bit-score")]
    bit_score: f64,
    #[serde(rename = "Hsp_score")]
    score: usize,
    #[serde(rename = "Hsp_evalue")]
    evalue: f64,
    #[serde(rename = "Hsp_query-from")]
    query_start: usize,
    #[serde(rename = "Hsp_query-to")]
    query_end: usize,
    #[serde(rename = "Hsp_hit-from")]
    subject_start: usize,
    #[serde(rename = "Hsp_hit-to")]
    subject_end: usize,
    #[serde(rename = "Hsp_gaps")]
    gap_opens: usize,
    #[serde(rename = "Hsp_align-len")]
    alignment_length: usize,
}
