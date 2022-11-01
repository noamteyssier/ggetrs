use serde::{Deserialize, Serialize};
use std::fmt;

/// An instance of a library contained within `Enrichr`
///
/// Data is stored as a json at <https://maayanlab.cloud/Enrichr/datasetStatistics>
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Library {
    pub library_name: String,
    pub category_id: usize,
    pub gene_coverage: usize,
    pub genes_per_term: f64,
    pub num_terms: usize,
    pub link: String,
}
impl fmt::Display for Library {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).expect("cannot serialize")
        )
    }
}
impl Library {
    pub fn minimal(&self) -> &str {
        &self.library_name
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Libraries(pub Vec<Library>);
impl fmt::Display for Libraries {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).expect("cannot serialize")
        )
    }
}
impl Libraries {
    pub fn minimal(&self) -> String {
        self.0
            .iter()
            .map(|x| x.minimal())
            .enumerate()
            .fold(String::new(), |mut s, (idx, x)| {
                let substring = if idx == 0 {
                    format!("{x}")
                } else {
                    format!("\n{x}")
                };
                s.push_str(&substring);
                s
            })
    }
}

/// An instance of category contained within `Enrichr`
///
/// Data is stored as a json at <https://maayanlab.cloud/Enrichr/datasetStatistics>
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub name: String,
    pub category_id: usize,
}
impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).expect("cannot serialize")
        )
    }
}

/// Multiple instances of category contained within `Enrichr`
#[derive(Serialize, Deserialize, Debug)]
pub struct Categories(pub Vec<Category>);
impl fmt::Display for Categories {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).expect("cannot serialize")
        )
    }
}

/// All libraries contained within `Enrichr`.
///
/// The `statistics` attribute is a container of all known [Library].
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseLibraries {
    pub statistics: Vec<Library>,
    pub categories: Vec<Category>,
}
impl ResponseLibraries {
    pub fn iter(&self) -> impl Iterator<Item = &Library> {
        self.statistics.iter()
    }
    pub fn categories(&self) -> Categories {
        Categories(self.categories.to_owned())
    }
    pub fn filter_categories(&self, cid: usize) -> Libraries {
        let libraries = self
            .statistics
            .iter()
            .filter(|x| x.category_id == cid)
            .map(|x| x.to_owned())
            .collect();
        Libraries(libraries)
    }
    pub fn libraries(&self) -> Libraries {
        Libraries(self.statistics.to_owned())
    }
}
impl fmt::Display for ResponseLibraries {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).expect("cannot serialize")
        )
    }
}
