use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// A struct to hold the results of an enrichment test.
///
/// The keys of this `HashMap` will be the background library
/// tested against and the values will each be an instance of [ResultEnrichr]
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseEnrich (
    HashMap<String, Vec<ResultEnrichr>>
);
impl fmt::Display for ResponseEnrich {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).expect("cannot serialize"))
    }
}

/// A singular enrichment result.
///
/// Names were taken from <https://maayanlab.cloud/Enrichr/help#api&q=3>
#[derive(Serialize, Deserialize, Debug)]
pub struct ResultEnrichr {
    pub rank: usize,
    pub term_name: String,
    pub pvalue: f64,
    pub zscore: f64,
    pub combined_score: f64,
    pub overlapping_genes: Vec<String>,
    pub adj_pvalue: f64,
    pub old_pvalue: f64,
    pub old_adj_pvalue: f64
}
impl fmt::Display for ResultEnrichr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{}\n,\t{}\n,\t{}\n,\t{}\n,\t{}\n,\t{:?}\n,\t{}\n,\t{}\n,\t{}\n}}",
            self.rank,
            self.term_name,
            self.pvalue,
            self.zscore,
            self.combined_score,
            self.overlapping_genes,
            self.adj_pvalue,
            self.old_pvalue,
            self.old_adj_pvalue
        )
    }
}

/// Performs an API call to the `Enrichr`'s `enrich`. 
///
/// This measures the significance of overlap of the provided gene list to the provided library
/// name.
pub async fn enrich(list_id: usize, library_name: &str) -> Result<ResponseEnrich, Error> {
    let url = format!(
        "https://maayanlab.cloud/Enrichr/enrich?userListId={}&backgroundType={}",
        list_id,
        library_name
    );
    reqwest::get(url)
        .await?
        .json::<ResponseEnrich>()
        .await
}
