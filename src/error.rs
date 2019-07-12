use std::fmt;

use reqwest::header::InvalidHeaderValue;

/// Result type for `rs621`, using [`rs621::error::Error`].
///
/// [`rs621::error::Error`]: enum.Error.html
pub type Result<T> = ::std::result::Result<T, Error>;

/// Enum for `rs621` errors.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// The given value for the `limit` option is above the maximum value allowed in a context.
    /// For instance, `order:score limit:350` is an invalid request because the maximum limit for
    /// ordered queries is 320.
    /// The first value is the value of the `limit` option. The second value is the biggest value
    /// allowed in this context.
    AboveLimit(usize, usize),
    /// An HTTP error has occurred. The `u16` value is the HTTP error code.
    Http(u16),
    /// Serialization error. The `String` value is a description of the error.
    Serial(String),
    /// Redirection error (e.g. redirection loop). The `String` value is a description of the error.
    Redirect(String),
    /// The request couldn't be send. The `String` value is a description of the error.
    CannotSendRequest(String),
    /// The client couldn't be created. The `String` value is a description of the error.
    CannotCreateClient(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::AboveLimit(limit, max) => write!(
                f,
                "limit:{} is above the maximum value for ordered queries ({})",
                limit, max
            ),
            Error::Http(code) => write!(f, "HTTP error: {}", code),
            Error::Serial(msg) => write!(f, "Serialization error: {}", msg),
            Error::Redirect(msg) => write!(f, "Redirection error: {}", msg),
            Error::CannotSendRequest(msg) => write!(f, "Couldn't send request: {}", msg),
            Error::CannotCreateClient(msg) => write!(f, "Couldn't create client: {}", msg),
        }
    }
}

impl From<InvalidHeaderValue> for Error {
    fn from(e: InvalidHeaderValue) -> Error {
        Error::CannotCreateClient(format!("Invalid header value: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_display_above_limit() {
        assert_eq!(
            format!("{}", Error::AboveLimit(400, 320)),
            String::from("limit:400 is above the maximum value for ordered queries (320)")
        );
    }

    #[test]
    fn error_display_http() {
        assert_eq!(
            format!("{}", Error::Http(404)),
            String::from("HTTP error: 404")
        );
    }

    #[test]
    fn error_display_serial() {
        assert_eq!(
            format!("{}", Error::Serial(String::from("foo"))),
            String::from("Serialization error: foo")
        );
    }

    #[test]
    fn error_display_redirect() {
        assert_eq!(
            format!("{}", Error::Redirect(String::from("foo"))),
            String::from("Redirection error: foo")
        );
    }

    #[test]
    fn error_display_cannot_send_request() {
        assert_eq!(
            format!("{}", Error::CannotSendRequest(String::from("foo"))),
            String::from("Couldn't send request: foo")
        );
    }

    #[test]
    fn error_display_cannot_create_client() {
        assert_eq!(
            format!("{}", Error::CannotCreateClient(String::from("foo"))),
            String::from("Couldn't create client: foo")
        );
    }
}