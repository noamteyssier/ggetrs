/// `Enrichr` submodule
pub mod enrichr;

/// Useful generic type for Request Errors
pub type RequestError = Box<dyn std::error::Error + Send + Sync>;
