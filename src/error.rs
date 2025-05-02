//! Error types for the GLEIF API client.

use thiserror::Error;

/// Errors that can occur when using the GLEIF API client.
#[derive(Debug, Error)]
pub enum GleifError {
    /// Network or HTTP error from reqwest or middleware.
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest_middleware::Error),

    /// Non-successful HTTP status code.
    #[error("GLEIF API returned error status: {0}")]
    Status(reqwest::StatusCode),

    /// Error parsing JSON response.
    #[error("Failed to parse JSON: {0}")]
    Json(#[from] serde_json::Error),

    /// URL construction or encoding error.
    #[error("URL error: {0}")]
    Url(#[from] reqwest::Error),

    /// Any other error.
    #[error("Other error: {0}")]
    Other(String),
}
