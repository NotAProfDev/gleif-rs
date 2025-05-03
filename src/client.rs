//! GLEIF API Client
//!
//! This module provides the `GleifClient` struct for interacting with the GLEIF v1 API.
//! It is designed for extensibility and ergonomic request building, using reqwest and reqwest-middleware.
//!
//! # Examples
//!
//! ## Basic client creation
//!
//! ```rust
//! use gleif_rs::client::GleifClient;
//!
//! // Create a client with default settings
//! let client = GleifClient::new();
//! ```
//!
//! ## Using the builder pattern
//!
//! ```rust
//! use gleif_rs::client::GleifClient;
//!
//! // Create a client with a custom base URL
//! let client = GleifClient::builder()
//!     .base_url("https://api-sandbox.gleif.org/api/v1")
//!     .build();
//! ```
//!
//! ## Creating a client from existing HTTP clients
//!
//! ```rust
//! use gleif_rs::client::GleifClient;
//! use reqwest::Client as ReqwestClient;
//! use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
//! use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};
//!
//! // Create from a plain reqwest client
//! let reqwest_client = ReqwestClient::new();
//! let client = GleifClient::from_reqwest_client(reqwest_client);
//!
//! // Create from a pre-configured middleware client
//! let reqwest_client = ReqwestClient::new();
//! let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
//! let middleware_client = ClientBuilder::new(reqwest_client)
//!     .with(RetryTransientMiddleware::new_with_policy(retry_policy))
//!     .build();
//! let client = GleifClient::from_middleware_client(middleware_client);
//! ```
//!
//! ## Advanced configuration with custom HTTP client and middleware
//!
//! ```rust
//! use gleif_rs::client::GleifClient;
//! use reqwest::ClientBuilder as ReqwestClientBuilder;
//! use reqwest_middleware::ClientBuilder;
//! use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};
//! use std::time::Duration;
//!
//! // Configure a custom reqwest client with timeouts
//! let reqwest_client = ReqwestClientBuilder::new()
//!     .timeout(Duration::from_secs(30))
//!     .connect_timeout(Duration::from_secs(5))
//!     .build()
//!     .expect("Failed to build reqwest client");
//!
//! // Configure a retry policy with exponential backoff
//! let retry_policy = ExponentialBackoff::builder()
//!     .build_with_max_retries(3);
//!
//! // Create middleware with retry capability
//! let middleware = ClientBuilder::new(reqwest_client)
//!     .with(RetryTransientMiddleware::new_with_policy(retry_policy));
//!
//! // Build the client with all configurations
//! let client = GleifClient::builder()
//!     .base_url("https://api.gleif.org/api/v1")
//!     .middleware_builder(middleware)
//!     .build();
//! ```

use crate::{
    DEFAULT_BASE_URL, error::GleifError, error::Result, request_builder::GleifRequestBuilder,
};
use reqwest::{Client as ReqwestClient, Method};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use std::sync::Arc;
use url::Url;

/// The main entry point for interacting with the GLEIF API.
///
/// Wraps a `reqwest_middleware::ClientWithMiddleware` and provides methods for building requests to GLEIF endpoints.
#[derive(Clone, Debug)]
pub struct GleifClient {
    client: Arc<ClientWithMiddleware>,
    base_url: Url,
}

impl Default for GleifClient {
    /// Create a new `GleifClient` with default settings.
    fn default() -> Self {
        // Unwrap is safe here because the default configuration is valid.
        GleifClientBuilder::new().build().unwrap()
    }
}

impl GleifClient {
    /// Create a new `GleifClient` with default configuration.
    ///
    /// # Panics
    ///
    /// Panics if the default configuration is invalid. This should never happen unless the default constants are changed to invalid values.
    #[must_use]
    pub fn new() -> Self {
        Self::builder()
            .build()
            .expect("Default config should not fail")
    }

    /// Returns a builder for configuring a `GleifClient`.
    #[must_use]
    pub fn builder() -> GleifClientBuilder {
        GleifClientBuilder::new()
    }

    /// Create a client from a `reqwest_middleware::ClientWithMiddleware`.
    ///
    /// # Panics
    ///
    /// Panics if the default base URL is invalid. This should never happen unless the constant is changed to an invalid value.
    #[must_use]
    pub fn from_middleware_client(client: ClientWithMiddleware) -> Self {
        Self {
            client: Arc::new(client),
            base_url: Url::parse(DEFAULT_BASE_URL).expect("Default base URL should be valid"),
        }
    }

    /// Create a client from a `reqwest::Client`.
    #[must_use]
    pub fn from_reqwest_client(client: ReqwestClient) -> Self {
        let middleware = ClientBuilder::new(client).build();
        Self::from_middleware_client(middleware)
    }

    /// Returns the base URL for the API.
    #[must_use]
    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    /// Returns a reference to the underlying `reqwest_middleware` client.
    #[must_use]
    pub fn client(&self) -> &Arc<ClientWithMiddleware> {
        &self.client
    }

    /// Returns a request builder for the given endpoint path.
    #[must_use]
    pub fn request(&self, path: &str) -> GleifRequestBuilder {
        GleifRequestBuilder::new(self.clone(), Method::GET, path)
    }
}

/// Builder for configuring and constructing a `GleifClient`.
///
/// Provides a fluent interface for customizing various aspects of the client
/// before construction, such as the base URL, middleware, and other options.
pub struct GleifClientBuilder {
    middleware_builder: Option<ClientBuilder>,
    reqwest_client: Option<ReqwestClient>,
    base_url: String,
}

impl Default for GleifClientBuilder {
    /// Create a new `GleifClientBuilder` with default settings.
    fn default() -> Self {
        Self {
            middleware_builder: None,
            reqwest_client: None,
            base_url: String::from(DEFAULT_BASE_URL),
        }
    }
}

impl GleifClientBuilder {
    /// Create a new `GleifClientBuilder` with default settings.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a custom base URL for the API (replacing the default).
    #[must_use]
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    /// Use a custom reqwest client for the underlying HTTP operations.
    #[must_use]
    pub fn reqwest_client(mut self, client: ReqwestClient) -> Self {
        self.reqwest_client = Some(client);
        self
    }

    /// Use a custom middleware client builder for the underlying HTTP operations.
    ///
    /// This is useful when you need to add middleware like retry, logging, etc.
    #[must_use]
    pub fn middleware_builder(mut self, builder: ClientBuilder) -> Self {
        self.middleware_builder = Some(builder);
        self
    }

    /// Build the `GleifClient` with the configured settings.
    ///
    /// Consumes the builder to prevent accidental reuse.
    ///
    /// # Errors
    ///
    /// Returns a [`GleifError`] if the base URL is invalid or the client cannot be constructed.
    pub fn build(self) -> Result<GleifClient> {
        // Use the provided reqwest client or create a new one if not provided.
        let reqwest_client = self.reqwest_client.unwrap_or_default();

        // Use the provided middleware builder or create a new one if not provided.
        let client = if let Some(builder) = self.middleware_builder {
            builder.build()
        } else {
            ClientBuilder::new(reqwest_client).build()
        };

        // Parse the base URL. If invalid, return an error.
        let base_url = Url::parse(&self.base_url).map_err(GleifError::UrlParseError)?;

        Ok(GleifClient {
            client: Arc::new(client),
            base_url,
        })
    }
}

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
        assert_eq!(client.base_url().as_str(), DEFAULT_BASE_URL);
    }

    #[test]
    fn test_client_construction_from_middleware_client() {
        let reqwest_client = ReqwestClient::new();
        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
        let client_with_middleware = ClientBuilder::new(reqwest_client)
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build();
        let client = GleifClient::from_middleware_client(client_with_middleware);
        assert_eq!(client.base_url().as_str(), DEFAULT_BASE_URL);
    }

    #[test]
    fn test_client_builder() {
        let client = GleifClient::builder()
            .build()
            .expect("Client build should succeed");
        assert_eq!(client.base_url().as_str(), DEFAULT_BASE_URL);

        let custom_url = "https://custom.gleif.org/api/v2";
        let client = GleifClient::builder()
            .base_url(custom_url)
            .build()
            .expect("Client build should succeed");
        assert_eq!(client.base_url().as_str(), custom_url);
    }

    #[test]
    fn test_client_new() {
        let client = GleifClient::new();
        assert_eq!(client.base_url().as_str(), DEFAULT_BASE_URL);
    }

    #[test]
    fn test_client_builder_with_middleware() {
        let reqwest_client = ReqwestClient::new();
        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
        let middleware_builder = ClientBuilder::new(reqwest_client.clone())
            .with(RetryTransientMiddleware::new_with_policy(retry_policy));

        let client = GleifClient::builder()
            .middleware_builder(middleware_builder)
            .build()
            .expect("Client build should succeed");

        assert_eq!(client.base_url().as_str(), DEFAULT_BASE_URL);
    }
}
