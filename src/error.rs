//! Error types for the GLEIF API client.
//!
//! This module provides a comprehensive error type hierarchy for the GLEIF API client.
//! [`GleifError`] represents common errors that can occur when interacting with the
//! GLEIF API, including network issues, API errors, JSON parsing failures, and more.
//!
//! The error variants are designed to be specific enough for detailed error handling
//! while providing informative error messages suitable for logging or debugging.
//!
//! # Example: Matching and Handling Errors
//!
//! ```rust
//! use gleif_rs::error::GleifError;
//!
//! fn handle_error(err: GleifError) {
//!     match &err {
//!         e if e.is_status() => {
//!             // Handle HTTP status errors (e.g., 404, 500)
//!             if let Some(status) = e.status() {
//!                 eprintln!("HTTP error: {status}. Error: {err}");
//!             }
//!         }
//!         e if e.is_timeout() => {
//!             // Handle timeouts
//!             eprintln!("Request timed out. Error: {err}");
//!         }
//!         e if e.is_connect() => {
//!             // Handle network connection errors
//!             eprintln!("Network connection error. Error: {err}");
//!         }
//!         _ => {
//!             // Handle all other errors or add more specific cases
//!             eprintln!("Other error: {err}");
//!         }
//!     }
//! }
//! ```

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
}

impl GleifError {
    /// Returns a possible URL related to this error.
    pub fn url(&self) -> Option<&Url> {
        match self {
            GleifError::ReqwestMiddlewareError(inner) => inner.url(),
            GleifError::UrlParseError(_) => None,
        }
    }

    /// Returns a mutable reference to the URL related to this error.
    ///
    /// This is useful if you need to remove sensitive information from the URL
    /// (e.g. an API key in the query), but do not want to remove the URL entirely.
    pub fn url_mut(&mut self) -> Option<&mut Url> {
        match self {
            GleifError::ReqwestMiddlewareError(inner) => inner.url_mut(),
            GleifError::UrlParseError(_) => None,
        }
    }

    /// Adds a url related to this error (overwriting any existing).
    pub fn with_url(self, url: Url) -> Self {
        match self {
            GleifError::ReqwestMiddlewareError(inner) => {
                GleifError::ReqwestMiddlewareError(inner.with_url(url))
            }
            GleifError::UrlParseError(e) => GleifError::UrlParseError(e),
        }
    }

    /// Strips the related URL from this error (if, for example, it contains sensitive information).
    pub fn without_url(self) -> Self {
        match self {
            GleifError::ReqwestMiddlewareError(inner) => {
                GleifError::ReqwestMiddlewareError(inner.without_url())
            }
            GleifError::UrlParseError(e) => GleifError::UrlParseError(e),
        }
    }

    /// Returns true if the error is from any middleware.
    pub fn is_middleware(&self) -> bool {
        match self {
            GleifError::ReqwestMiddlewareError(inner) => inner.is_middleware(),
            GleifError::UrlParseError(_) => false,
        }
    }

    /// Returns true if the error is from a type `Builder`.
    pub fn is_builder(&self) -> bool {
        match self {
            GleifError::ReqwestMiddlewareError(inner) => inner.is_builder(),
            GleifError::UrlParseError(_) => false,
        }
    }

    /// Returns true if the error is from a `RedirectPolicy`.
    pub fn is_redirect(&self) -> bool {
        match self {
            GleifError::ReqwestMiddlewareError(inner) => inner.is_redirect(),
            GleifError::UrlParseError(_) => false,
        }
    }

    /// Returns true if the error is from `Response::error_for_status`.
    pub fn is_status(&self) -> bool {
        match self {
            GleifError::ReqwestMiddlewareError(inner) => inner.is_status(),
            GleifError::UrlParseError(_) => false,
        }
    }

    /// Returns true if the error is related to a timeout.
    pub fn is_timeout(&self) -> bool {
        match self {
            GleifError::ReqwestMiddlewareError(inner) => inner.is_timeout(),
            GleifError::UrlParseError(_) => false,
        }
    }

    /// Returns true if the error is related to the request.
    pub fn is_request(&self) -> bool {
        match self {
            GleifError::ReqwestMiddlewareError(inner) => inner.is_request(),
            GleifError::UrlParseError(_) => false,
        }
    }

    /// Returns true if the error is related to connect.
    pub fn is_connect(&self) -> bool {
        match self {
            GleifError::ReqwestMiddlewareError(inner) => inner.is_connect(),
            GleifError::UrlParseError(_) => false,
        }
    }

    /// Returns true if the error is related to the request or response body.
    pub fn is_body(&self) -> bool {
        match self {
            GleifError::ReqwestMiddlewareError(inner) => inner.is_body(),
            GleifError::UrlParseError(_) => false,
        }
    }

    /// Returns true if the error is related to decoding the response's body.
    pub fn is_decode(&self) -> bool {
        match self {
            GleifError::ReqwestMiddlewareError(inner) => inner.is_decode(),
            GleifError::UrlParseError(_) => false,
        }
    }

    /// Returns the status code, if the error was generated from a response.
    pub fn status(&self) -> Option<reqwest::StatusCode> {
        match self {
            GleifError::ReqwestMiddlewareError(inner) => inner.status(),
            GleifError::UrlParseError(_) => None,
        }
    }
}
