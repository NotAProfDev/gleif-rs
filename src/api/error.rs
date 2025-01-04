//! Error types for the GLEIF API client.

use reqwest::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GleifError {
    #[error("Request failed with status code {0}")]
    RequestFailed(StatusCode),
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("Failed to deserialize response: {0}")]
    DeserializationError(reqwest::Error),
}
