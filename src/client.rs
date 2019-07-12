use super::error::{Error, Result};

use reqwest::header::{self, HeaderMap, HeaderValue};

/// Client struct.
#[derive(Debug)]
pub struct Client {
    client: reqwest::Client,
}

impl Client {
    /// Create a new client with the specified value for the User-Agent header. The API requires a
    /// non-empty User-Agent header for all requests, preferably including your E621 username and
    /// the name of your project.
    pub fn new(user_agent: impl AsRef<[u8]>) -> Result<Self> {
        if user_agent.as_ref() == b"" {
            Err(Error::CannotCreateClient(String::from(
                "User Agent mustn't be empty",
            )))
        } else {
            let mut headers = HeaderMap::new();
            headers.insert(
                header::USER_AGENT,
                HeaderValue::from_bytes(user_agent.as_ref())?,
            );

            Ok(Client {
                client: reqwest::Client::builder()
                    .timeout(None)
                    .default_headers(headers)
                    .build()
                    .unwrap(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_new() {
        Client::new(b"rs621/unit_test").unwrap();
    }

    #[test]
    fn client_new_requires_valid_user_agent() {
        assert!(Client::new(b"\n").is_err());
    }

    #[test]
    fn client_new_requires_non_empty_user_agent() {
        assert!(Client::new(b"").is_err());
    }
}