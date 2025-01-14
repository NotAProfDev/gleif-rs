//! Error types for the API client.
use thiserror::Error;

/// Represents errors that can occur in the API client.
#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Failed to build HTTP client: {0}")]
    ClientBuildError(reqwest::Error),
    #[error("Failed to deserialize response: {0}")]
    DeserializationError(serde_json::Error),
    #[error("Invalid header name: {0}")]
    InvalidHeaderName(reqwest::header::InvalidHeaderName),
    #[error("Invalid header value: {0}")]
    InvalidHeaderValue(reqwest::header::InvalidHeaderValue),
    #[error("Invalid URL: {0}")]
    InvalidUrl(url::ParseError),
    #[error("Request failed with status code {0}")]
    RequestError(reqwest::Error),
    #[error("Response error: {0}")]
    ResponseError(reqwest::Error),
    #[error("Response type error: {0}")]
    ResponseTypeError(String),
}
