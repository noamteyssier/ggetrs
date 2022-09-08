use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseEnrich (
    HashMap<String, Vec<ResultEnrichr>>
);
impl fmt::Display for ResponseEnrich {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).expect("cannot serialize"))
    }
}
// impl fmt::Display for ResponseEnrich {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{{")?;
//         for (k, v) in self.0.iter() {
//             write!(f, "{}: ", k)?;
//             for record in v {
//                 write!(f, "{}", serde_json::to_string_pretty(&record).expect("cannot serialize"))?;
//             }
//         }
//         write!(f, "}}")
//     }
// }

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
