//! GLEIF API Client
//!
//! This module provides the `GleifClient` struct for interacting with the GLEIF v1 API.
//! It is designed for extensibility and ergonomic request building, using reqwest and reqwest-middleware.

use crate::{DEFAULT_BASE_URL, request_builder::GleifRequestBuilder};
use reqwest::{Client as ReqwestClient, Method};
use reqwest_middleware::ClientWithMiddleware;
use std::sync::Arc;

/// The main entry point for interacting with the GLEIF API.
///
/// Wraps a `reqwest_middleware::ClientWithMiddleware` and provides methods for building requests to GLEIF endpoints.
#[derive(Clone, Debug)]
pub struct GleifClient {
    client: Arc<ClientWithMiddleware>,
    base_url: String,
}

impl GleifClient {
    /// Create a new `GleifClient` from a pre-configured `ClientWithMiddleware`.
    pub fn from_middleware_client(client: ClientWithMiddleware) -> Self {
        Self {
            client: Arc::new(client),
            base_url: DEFAULT_BASE_URL.to_string(),
        }
    }

    /// Create a new `GleifClient` from a plain `reqwest::Client` (no middleware).
    pub fn from_reqwest_client(client: ReqwestClient) -> Self {
        let client = reqwest_middleware::ClientBuilder::new(client).build();
        Self::from_middleware_client(client)
    }

    /// Returns the base URL used by this client.
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Returns a reference to the underlying reqwest_middleware client.
    pub(crate) fn client(&self) -> &Arc<ClientWithMiddleware> {
        &self.client
    }

    /// Start building a generic request to a GLEIF API endpoint.
    pub fn request(&self, path: &str) -> GleifRequestBuilder {
        GleifRequestBuilder::new(self.clone(), Method::GET, path)
    }
}

// Unit tests for construction and middleware
#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::Client as ReqwestClient;
    use reqwest_middleware::ClientBuilder;
    use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};

    #[test]
    fn test_client_construction_from_reqwest() {
        let reqwest_client = ReqwestClient::new();
        let client = GleifClient::from_reqwest_client(reqwest_client);
        assert_eq!(client.base_url(), DEFAULT_BASE_URL);
    }

    #[test]
    fn test_client_construction_from_middleware_client() {
        let reqwest_client = ReqwestClient::new();
        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
        let client_with_middleware = ClientBuilder::new(reqwest_client)
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build();
        let client = GleifClient::from_middleware_client(client_with_middleware);
        assert_eq!(client.base_url(), DEFAULT_BASE_URL);
    }
}
