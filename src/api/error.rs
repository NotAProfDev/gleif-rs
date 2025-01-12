//! Error types for the API client.
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Request failed with status code {0}")]
    RequestError(reqwest::Error),
    #[error("Failed to deserialize response: {0}")]
    DeserializationError(reqwest::Error),
    #[error("Invalid URL: {0}")]
    InvalidUrl(url::ParseError),
    #[error("Failed to build HTTP client: {0}")]
    ClientBuildError(reqwest::Error),
    #[error("Response type error: {0}")]
    ResponseTypeError(String),
}
