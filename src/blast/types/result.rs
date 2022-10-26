use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BlastResult {
    results: Vec<BlastHit>
}
impl BlastResult {
    pub fn from_blast_output(output: &BlastOutput) -> Self {
        Self {
            results: output
                .blast_output_iterations.iterations
                .iteration_hits
                .hits
                .iter()
                .map(|x| BlastHit::from_hit(x))
                .collect()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlastHit {
    num: usize,
    id: String,
    definition: String,
    accession: String,
    length: usize,
    bit_score: f64,
    score: usize,
    evalue: f64,
    gap_opens: usize,
    alignment_length: usize,
    query_start: usize,
    query_end: usize,
    subject_start: usize,
    subject_end: usize,
}
impl BlastHit {
    fn from_hit(hit: &Hit) -> Self {
        Self {
            num: hit.num,
            id: hit.id.to_string(),
            definition: hit.definition.to_string(),
            accession: hit.accession.to_string(),
            length: hit.length,
            bit_score: hit.statistics.hsp.bit_score,
            score: hit.statistics.hsp.score,
            evalue: hit.statistics.hsp.evalue,
            query_start: hit.statistics.hsp.query_start,
            query_end: hit.statistics.hsp.query_end,
            subject_start: hit.statistics.hsp.subject_start,
            subject_end: hit.statistics.hsp.subject_end,
            gap_opens: hit.statistics.hsp.gap_opens,
            alignment_length: hit.statistics.hsp.alignment_length
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlastOutput {
    #[serde(rename = "BlastOutput_iterations")]
    blast_output_iterations: BlastOutputIterations
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
    iteration_hits: IterationHits
}

#[derive(Debug, Serialize, Deserialize)]
struct IterationHits {
    #[serde(rename = "Hit")]
    hits: Vec<Hit>
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
    statistics: HitStatistics
}

#[derive(Debug, Serialize, Deserialize)]
struct HitStatistics {
    #[serde(rename = "Hsp")]
    hsp: Hsp
}

#[derive(Debug, Serialize, Deserialize)]
struct Hsp{
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
    alignment_length: usize
}
