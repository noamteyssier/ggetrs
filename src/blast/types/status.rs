use anyhow::{bail, Result};

pub enum BlastStatus {
    Waiting,
    Unknown,
    Ready,
}
impl BlastStatus {
    pub fn from_str(value: &str) -> Result<Self> {
        match value {
            "WAITING" => Ok(Self::Waiting),
            "UNKNOWN" => Ok(Self::Unknown),
            "READY" => Ok(Self::Ready),
            _ => bail!(format!("Unexpected request status found: {value}")),
        }
    }
}
