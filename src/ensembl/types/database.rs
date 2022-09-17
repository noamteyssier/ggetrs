use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseDatabases(pub Vec<Database>);
impl fmt::Display for ResponseDatabases {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).expect("cannot serialize")
        )
    }
}
impl ResponseDatabases {
    #[must_use]
    pub fn as_vec(&self) -> Vec<String> {
        self.0.iter().map(|x| x.0.to_string()).collect()
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Database(pub String);
impl fmt::Display for Database {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).expect("cannot serialize")
        )
    }
}
