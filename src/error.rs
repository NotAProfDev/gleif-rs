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
//!   or other [`reqwest`] related network errors.
//! - **API Errors:** HTTP status codes returned by the GLEIF API indicating an issue
//!   with the request (e.g., 400 Bad Request, 401 Unauthorized, 404 Not Found,
//!   500 Internal Server Error). [`GleifError`] provides methods to inspect these
//!   status codes.
//! - **Response Processing Errors:** Failures during the processing of a response,
//!   such as:
//!     - **JSON Deserialization Failures:** If the API response isn't the expected JSON
//!       structure, or if there's an error parsing it (typically wrapped in [`reqwest_middleware::Error`]).
//!     - **Middleware Errors:** Errors originating from the [`reqwest-middleware`](https://docs.rs/reqwest-middleware/) stack,
//!       like retry policies being exhausted.
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
//! to differentiate between various error conditions and respond accordingly.
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
//! specifics of your application's needs when interacting with the GLEIF API.
//! For more details on specific error variants or if you need to access the underlying
//! source error, you can use the `source()` method from the [`std::error::Error`] trait.

use thiserror::Error;
use url::Url;

/// Type alias for the result type used throughout the GLEIF API client.
/// This is a convenience type alias for `std::result::Result<T, GleifError>`.
pub type Result<T> = std::result::Result<T, GleifError>;

/// Errors that can occur when using the GLEIF API client.
#[derive(Debug, Error)]
pub enum GleifError {
    /// Wrapper for all errors originating from the HTTP client layer
    #[error(transparent)]
    ReqwestMiddlewareError(#[from] reqwest_middleware::Error),

    /// Error when the URL is invalid
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),

    /// Error when parsing a field name fails or the field is not allowed
    #[error("Field parse error: {0}")]
    FieldParseError(String),

    /// Error when parsing a value name fails
    #[error("Value parse error: {0}")]
    ValueParseError(String),
}

impl GleifError {
    /// Returns the URL associated with this error, if available.
    #[must_use]
    pub fn url(&self) -> Option<&Url> {
        match self {
            GleifError::ReqwestMiddlewareError(inner) => inner.url(),
            GleifError::UrlParseError(_)
            | GleifError::FieldParseError(_)
            | GleifError::ValueParseError(_) => None,
        }
    }

    /// Returns a mutable reference to the URL related to this error.
    ///
    /// This is useful if you need to remove sensitive information from the URL
    /// (e.g. an API key in the query), but do not want to remove the URL entirely.
    pub fn url_mut(&mut self) -> Option<&mut Url> {
        match self {
            GleifError::ReqwestMiddlewareError(inner) => inner.url_mut(),
            GleifError::UrlParseError(_)
            | GleifError::FieldParseError(_)
            | GleifError::ValueParseError(_) => None,
        }
    }

    /// Returns a new error with the given URL attached (overwriting any existing).
    #[must_use]
    pub fn with_url(self, url: Url) -> Self {
        match self {
            GleifError::ReqwestMiddlewareError(inner) => {
                GleifError::ReqwestMiddlewareError(inner.with_url(url))
            }
            GleifError::UrlParseError(e) => GleifError::UrlParseError(e),
            GleifError::FieldParseError(e) => GleifError::FieldParseError(e),
            GleifError::ValueParseError(e) => GleifError::ValueParseError(e),
        }
    }

    /// Returns a error with the URL removed (if, for example, it contains sensitive information).
    #[must_use]
    pub fn without_url(self) -> Self {
        match self {
            GleifError::ReqwestMiddlewareError(inner) => {
                GleifError::ReqwestMiddlewareError(inner.without_url())
            }
            GleifError::UrlParseError(e) => GleifError::UrlParseError(e),
            GleifError::FieldParseError(e) => GleifError::FieldParseError(e),
            GleifError::ValueParseError(e) => GleifError::ValueParseError(e),
        }
    }

    /// Returns true if this error originated from middleware.
    #[must_use]
    pub fn is_middleware(&self) -> bool {
        match self {
            GleifError::ReqwestMiddlewareError(inner) => inner.is_middleware(),
            GleifError::UrlParseError(_)
            | GleifError::FieldParseError(_)
            | GleifError::ValueParseError(_) => false,
        }
    }

    /// Returns true if this error originated from the builder.
    #[must_use]
    pub fn is_builder(&self) -> bool {
        match self {
            GleifError::ReqwestMiddlewareError(inner) => inner.is_builder(),
            GleifError::UrlParseError(_)
            | GleifError::FieldParseError(_)
            | GleifError::ValueParseError(_) => false,
        }
    }

    /// Returns true if this error is a redirect error.
    #[must_use]
    pub fn is_redirect(&self) -> bool {
        match self {
            GleifError::ReqwestMiddlewareError(inner) => inner.is_redirect(),
            GleifError::UrlParseError(_)
            | GleifError::FieldParseError(_)
            | GleifError::ValueParseError(_) => false,
        }
    }

    /// Returns true if this error is a status error.
    #[must_use]
    pub fn is_status(&self) -> bool {
        match self {
            GleifError::ReqwestMiddlewareError(inner) => inner.is_status(),
            GleifError::UrlParseError(_)
            | GleifError::FieldParseError(_)
            | GleifError::ValueParseError(_) => false,
        }
    }

    /// Returns true if this error is a timeout error.
    #[must_use]
    pub fn is_timeout(&self) -> bool {
        match self {
            GleifError::ReqwestMiddlewareError(inner) => inner.is_timeout(),
            GleifError::UrlParseError(_)
            | GleifError::FieldParseError(_)
            | GleifError::ValueParseError(_) => false,
        }
    }

    /// Returns true if this error is a request error.
    #[must_use]
    pub fn is_request(&self) -> bool {
        match self {
            GleifError::ReqwestMiddlewareError(inner) => inner.is_request(),
            GleifError::UrlParseError(_)
            | GleifError::FieldParseError(_)
            | GleifError::ValueParseError(_) => false,
        }
    }

    /// Returns true if this error is a connection error.
    #[must_use]
    pub fn is_connect(&self) -> bool {
        match self {
            GleifError::ReqwestMiddlewareError(inner) => inner.is_connect(),
            GleifError::UrlParseError(_)
            | GleifError::FieldParseError(_)
            | GleifError::ValueParseError(_) => false,
        }
    }

    /// Returns true if this error is related to the request or response body.
    #[must_use]
    pub fn is_body(&self) -> bool {
        match self {
            GleifError::ReqwestMiddlewareError(inner) => inner.is_body(),
            GleifError::UrlParseError(_)
            | GleifError::FieldParseError(_)
            | GleifError::ValueParseError(_) => false,
        }
    }

    /// Returns true if this error is a decode error.
    #[must_use]
    pub fn is_decode(&self) -> bool {
        match self {
            GleifError::ReqwestMiddlewareError(inner) => inner.is_decode(),
            GleifError::UrlParseError(_)
            | GleifError::FieldParseError(_)
            | GleifError::ValueParseError(_) => false,
        }
    }

    /// Returns the HTTP status code associated with this error, if available.
    #[must_use]
    pub fn status(&self) -> Option<reqwest::StatusCode> {
        match self {
            GleifError::ReqwestMiddlewareError(inner) => inner.status(),
            GleifError::UrlParseError(_)
            | GleifError::FieldParseError(_)
            | GleifError::ValueParseError(_) => None,
        }
    }
}
