//! # Error Handling ([`GleifError`]) - Error types for API operations.
//!
//! This module defines the error types used throughout the [`gleif-rs`](crate) client, ensuring
//! a consistent and comprehensive error handling experience when interacting with the
//! [GLEIF API](https://www.gleif.org/en/lei-data/gleif-api).
//!
//! ## The [`GleifError`] Type
//!
//! The primary error type you'll encounter is [`GleifError`]. It serves as a unified
//! wrapper around various potential issues, including:
//!
//! - **Network Issues:** Problems establishing a connection, DNS failures, timeouts,
//!   or other [`reqwest`] network errors.
//! - **API Errors:** HTTP status codes returned by the GLEIF API indicating an issue
//!   with the request (e.g., 400 Bad Request, 401 Unauthorized, 404 Not Found,
//!   500 Internal Server Error). [`GleifError`] provides methods to inspect these
//!   status codes.
//! - **Response Processing Errors:** Failures during the processing of a response,
//!   such as:
//!     - **JSON Deserialization Failures:** If the API response isn't the expected JSON
//!       structure, or if there's an error parsing it (typically wrapped in [`serde_json::Error`]).
//!     - **Middleware Errors:** Errors originating from the [`reqwest-middleware`](https://docs.rs/reqwest-middleware/) stack,
//!       such as retry policies being exhausted.
//! - **Request Building Errors:** Issues that occur before a request is sent, such as
//!   invalid URL formation or header construction problems.
//!
//! [`GleifError`] implements [`std::error::Error`] and [`std::fmt::Display`], allowing it to be easily integrated
//! into other error handling systems (e.g., `anyhow`, `eyre`).
//!
//! ## Inspecting and Handling Errors
//!
//! [`GleifError`] provides convenient helper methods like [`is_status`](GleifError::is_status), [`status`](GleifError::status),
//! [`is_timeout`](GleifError::is_timeout), and [`is_connect`](GleifError::is_connect) to categorize and inspect errors without needing
//! to match against potentially numerous internal variants directly. This simplifies
//! common error handling patterns.
//!
//! ### Example: Categorizing and Responding to Errors
//!
//! The following example demonstrates how to use the helper methods on [`GleifError`]
//! to differentiate between various error conditions and respond accordingly:
//!
//! ```rust
//! use gleif_rs::error::GleifError;
//!
//! fn handle_error(err: GleifError) {
//!     if err.is_status() {
//!         if let Some(status) = err.status() {
//!             eprintln!("HTTP error: {status}. Error: {err}");
//!         }
//!     } else if err.is_timeout() {
//!         eprintln!("Request timed out. Error: {err}");
//!     } else if err.is_connect() {
//!         eprintln!("Network connection error. Error: {err}");
//!     } else {
//!         eprintln!("Other error: {err}");
//!     }
//! }
//! ```
//!
//! By using these methods, you can build robust error handling logic tailored to the
//! needs of your application when interacting with the GLEIF API.
//! For more details on specific error variants or if you need to access the underlying
//! source error, you can use the `source()` method from the [`std::error::Error`] trait.

use std::error;
use std::fmt;
use url::Url;

/// Type alias for the result type used throughout the GLEIF API client.
/// This is a convenience type alias for `std::result::Result<T, GleifError>`.
pub type Result<T> = std::result::Result<T, GleifError>;

/// Errors that can occur when using the GLEIF API client.
#[derive(Debug)]
pub enum GleifError {
    /// Error from reqwest directly
    ReqwestError(reqwest::Error),

    /// Error from `reqwest_middleware`
    ReqwestMiddlewareError(reqwest_middleware::Error),

    /// Error when the URL is invalid
    UrlParseError(url::ParseError),

    /// Error from `serde_json`
    SerdeError(serde_json::Error),

    /// Error from `std::io`
    IoError(std::io::Error),

    /// Error with attached response content
    ResponseError(ResponseContent),

    /// Error when parsing a field or value fails
    ParseError {
        /// The kind of parse error.
        kind: ParseErrorKind,
        /// The error message describing the parse failure.
        message: String,
    },
}

/// Generic response content for errors with attached payloads.
#[derive(Debug, Clone)]
pub struct ResponseContent {
    /// The HTTP status code of the response.
    pub status: reqwest::StatusCode,
    /// The content of the response as a string.
    pub content: String,
}

/// Kinds of parse errors for fields and values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseErrorKind {
    /// Error related to parsing a field name.
    Field,
    /// Error related to parsing a value.
    Value,
}

impl fmt::Display for GleifError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            GleifError::ReqwestError(e) => ("reqwest", e.to_string()),
            GleifError::ReqwestMiddlewareError(e) => ("reqwest-middleware", e.to_string()),
            GleifError::UrlParseError(e) => ("url", e.to_string()),
            GleifError::SerdeError(e) => ("serde", e.to_string()),
            GleifError::IoError(e) => ("IO", e.to_string()),
            GleifError::ResponseError(e) => ("response", format!("status code {}", e.status)),
            GleifError::ParseError { kind, message } => match kind {
                ParseErrorKind::Field => ("field", message.to_owned()),
                ParseErrorKind::Value => ("value", message.to_owned()),
            },
        };
        write!(f, "error in {module}: {e}")
    }
}

impl error::Error for GleifError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            GleifError::ReqwestError(e) => Some(e),
            GleifError::ReqwestMiddlewareError(e) => Some(e),
            GleifError::SerdeError(e) => Some(e),
            GleifError::IoError(e) => Some(e),
            GleifError::UrlParseError(e) => Some(e),
            GleifError::ParseError { .. } | GleifError::ResponseError(_) => None,
        }
    }
}

impl From<reqwest::Error> for GleifError {
    fn from(e: reqwest::Error) -> Self {
        GleifError::ReqwestError(e)
    }
}

impl From<reqwest_middleware::Error> for GleifError {
    fn from(e: reqwest_middleware::Error) -> Self {
        GleifError::ReqwestMiddlewareError(e)
    }
}

impl From<url::ParseError> for GleifError {
    fn from(e: url::ParseError) -> Self {
        GleifError::UrlParseError(e)
    }
}

impl From<serde_json::Error> for GleifError {
    fn from(e: serde_json::Error) -> Self {
        GleifError::SerdeError(e)
    }
}

impl From<std::io::Error> for GleifError {
    fn from(e: std::io::Error) -> Self {
        GleifError::IoError(e)
    }
}

impl GleifError {
    /// Returns the URL associated with this error, if available.
    ///
    /// This will extract the URL from `reqwest_middleware::Error` or `reqwest::Error` if present.
    #[must_use]
    pub fn url(&self) -> Option<&Url> {
        match self {
            GleifError::ReqwestError(inner) => inner.url(),
            GleifError::ReqwestMiddlewareError(inner) => inner.url(),
            _ => None,
        }
    }

    /// Returns a mutable reference to the URL related to this error.
    ///
    /// This is useful if you need to remove sensitive information from the URL
    /// (e.g. an API key in the query), but do not want to remove the URL entirely.
    #[must_use]
    pub fn url_mut(&mut self) -> Option<&mut Url> {
        match self {
            GleifError::ReqwestError(inner) => inner.url_mut(),
            GleifError::ReqwestMiddlewareError(inner) => inner.url_mut(),
            _ => None,
        }
    }

    /// Returns a new error with the given URL attached (overwriting any existing).
    #[must_use]
    pub fn with_url(self, url: Url) -> Self {
        match self {
            GleifError::ReqwestError(inner) => GleifError::ReqwestError(inner.with_url(url)),
            GleifError::ReqwestMiddlewareError(inner) => {
                GleifError::ReqwestMiddlewareError(inner.with_url(url))
            }
            // For all other variants, attaching a URL is not applicable; return self unchanged.
            _ => self,
        }
    }

    /// Returns an error with the URL removed (if, for example, it contains sensitive information).
    #[must_use]
    pub fn without_url(self) -> Self {
        match self {
            GleifError::ReqwestError(inner) => GleifError::ReqwestError(inner.without_url()),
            GleifError::ReqwestMiddlewareError(inner) => {
                GleifError::ReqwestMiddlewareError(inner.without_url())
            }
            // For all other variants, removing a URL is not applicable; return self unchanged.
            _ => self,
        }
    }

    /// Returns true if this error originated from middleware.
    #[must_use]
    pub fn is_middleware(&self) -> bool {
        match self {
            GleifError::ReqwestMiddlewareError(inner) => inner.is_middleware(),
            _ => false,
        }
    }

    /// Returns true if this error originated from the builder.
    #[must_use]
    pub fn is_builder(&self) -> bool {
        match self {
            GleifError::ReqwestMiddlewareError(inner) => inner.is_builder(),
            _ => false,
        }
    }

    /// Returns true if this error is a redirect error.
    #[must_use]
    pub fn is_redirect(&self) -> bool {
        match self {
            GleifError::ReqwestError(inner) => inner.is_redirect(),
            GleifError::ReqwestMiddlewareError(inner) => inner.is_redirect(),
            _ => false,
        }
    }

    /// Returns true if this error is a status error.
    #[must_use]
    pub fn is_status(&self) -> bool {
        match self {
            GleifError::ReqwestError(inner) => inner.is_status(),
            GleifError::ReqwestMiddlewareError(inner) => inner.is_status(),
            GleifError::ResponseError(_) => true,
            _ => false,
        }
    }

    /// Returns true if this error is a timeout error.
    #[must_use]
    pub fn is_timeout(&self) -> bool {
        match self {
            GleifError::ReqwestError(inner) => inner.is_timeout(),
            GleifError::ReqwestMiddlewareError(inner) => inner.is_timeout(),
            // For all other variants, timeout is not applicable.
            _ => false,
        }
    }

    /// Returns true if this error is a request error.
    #[must_use]
    pub fn is_request(&self) -> bool {
        match self {
            GleifError::ReqwestError(inner) => inner.is_request(),
            GleifError::ReqwestMiddlewareError(inner) => inner.is_request(),
            _ => false,
        }
    }

    /// Returns true if this error is a connection error.
    #[must_use]
    pub fn is_connect(&self) -> bool {
        match self {
            GleifError::ReqwestError(inner) => inner.is_connect(),
            GleifError::ReqwestMiddlewareError(inner) => inner.is_connect(),
            _ => false,
        }
    }

    /// Returns true if this error is related to the request or response body.
    #[must_use]
    pub fn is_body(&self) -> bool {
        match self {
            GleifError::ReqwestError(inner) => inner.is_body(),
            GleifError::ReqwestMiddlewareError(inner) => inner.is_body(),
            _ => false,
        }
    }

    /// Returns true if this error is a decode error.
    #[must_use]
    pub fn is_decode(&self) -> bool {
        match self {
            GleifError::ReqwestError(inner) => inner.is_decode(),
            GleifError::ReqwestMiddlewareError(inner) => inner.is_decode(),
            GleifError::ParseError { .. } => true,
            _ => false,
        }
    }

    /// Returns the HTTP status code associated with this error, if available.
    #[must_use]
    pub fn status(&self) -> Option<reqwest::StatusCode> {
        match self {
            GleifError::ReqwestError(inner) => inner.status(),
            GleifError::ReqwestMiddlewareError(inner) => inner.status(),
            GleifError::ResponseError(resp) => Some(resp.status),
            _ => None,
        }
    }
}
