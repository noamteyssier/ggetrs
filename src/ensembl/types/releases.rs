use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Releases {
    releases: Vec<usize>,
}
impl Releases {
    pub fn max(&self) -> usize {
        *self.releases.iter().max().expect("No releases recovered")
    }
}
