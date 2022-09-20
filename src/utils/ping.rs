use reqwest::blocking::Client;
use std::time::Duration;

/// Pings a provided url to see if it is accessible.
///
/// Useful for FTP clients which cannot provide a timeout
pub fn ping(url: &str, timeout: usize) -> bool {
    let response = Client::new()
        .get(url)
        .timeout(Duration::from_secs(timeout as u64))
        .send();
    response.is_ok()
}

#[cfg(test)]
mod testing {
    use super::ping;

    #[test]
    fn test_ping_true() {
        let url = "https://google.com";
        assert!(ping(url, 3));
    }

    #[test]
    fn test_ping_false() {
        let url = "https://aisdjiaowdjanwidanowidnawdawd.com";
        assert!(!ping(url, 3));
    }
}
