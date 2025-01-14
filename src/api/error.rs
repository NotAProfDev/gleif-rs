//! Error types for the API client.
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Request failed with status code {0}")]
    RequestError(reqwest::Error),
    #[error("Failed to deserialize response: {0}")]
    DeserializationError(serde_json::Error),
    #[error("Invalid URL: {0}")]
    InvalidUrl(url::ParseError),
    #[error("Failed to build HTTP client: {0}")]
    ClientBuildError(reqwest::Error),
    #[error("Response type error: {0}")]
    ResponseTypeError(String),
    #[error("Response error: {0}")]
    ResponseError(reqwest::Error),
    #[error("Invalid header name: {0}")]
    InvalidHeaderName(reqwest::header::InvalidHeaderName),
    #[error("Invalid header value: {0}")]
    InvalidHeaderValue(reqwest::header::InvalidHeaderValue),
    #[error("Invalid timeout value {0}")]
    InvalidTimeout(u64),
}
