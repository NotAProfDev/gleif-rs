//! # Main GLEIF API Client ([`GleifClient`]) - Entry Point for GLEIF API Interactions.
//!
//! This module provides [`GleifClient`], the main entry point for interacting with the [Global Legal Entity Identifier
//! Foundation (GLEIF) API](https://www.gleif.org/en/lei-data/gleif-api). It is designed for both simplicity and extensibility, supporting ergonomic
//! request building, custom configuration, and advanced middleware.
//!
//! ## Key Features
//!
//! - **Simple Default Setup:** Quickly get started with [`crate::client::GleifClient::new`].
//! - **Flexible Configuration:** Use the builder pattern ([`crate::client::GleifClient::builder`]) for fine-grained control, including custom base URLs and middleware.
//! - **Bring Your Own Client:** Integrate an existing [`reqwest::Client`] or a pre-configured [`reqwest_middleware::ClientWithMiddleware`].
//! - **Middleware Ready:** Leverage [`reqwest-middleware`] for retries, logging, and more.
//! - **Ergonomic Request Building:** Provides methods for interacting with GLEIF API endpoints using [`crate::request_builder::GleifRequestBuilder`].
//!
//! Below are various ways to create and configure your [`GleifClient`].
//!
//! # Client Instantiation Examples
//!
//! ## 1. Basic Client Creation (Default Settings)
//!
//! For most common use cases, creating a client with default settings is sufficient.
//! This uses the default GLEIF API base URL and a standard `reqwest` client.
//!
//! ```rust
//! use gleif_rs::client::GleifClient;
//!
//! // Create a client with sensible defaults
//! let client = GleifClient::new();
//! // You can now use `client` to make API calls.
//! ```
//!
//! ## 2. Using the Builder Pattern for Customization
//!
//! The builder pattern allows for more control over the client's configuration,
//! such as specifying a different base URL (e.g., for a sandbox environment).
//!
//! ```rust
//! use gleif_rs::client::GleifClient;
//!
//! // Create a client targeting the GLEIF sandbox API
//! let client = GleifClient::builder()
//!     .base_url("[https://api-sandbox.gleif.org/api/v1](https://api-sandbox.gleif.org/api/v1)")
//!     .build();
//! // `client` is now configured for the sandbox.
//! ```
//!
//! ## 3. Creating a Client from Existing HTTP Clients
//!
//! If you have an existing [`reqwest::Client`] or a [`reqwest_middleware::ClientWithMiddleware`]
//! (perhaps shared across your application or configured with specific policies),
//! you can easily wrap it with [`GleifClient`].
//!
//! ### From a plain [`reqwest::Client`]
//!
//! ```rust
//! use gleif_rs::client::GleifClient;
//! use reqwest::Client as ReqwestClient;
//!
//! // Assume `reqwest_client` is already configured elsewhere
//! let reqwest_client = ReqwestClient::new();
//! let client = GleifClient::from_reqwest_client(reqwest_client);
//! // `client` will use your provided reqwest_client.
//! ```
//!
//! ### From a pre-configured [`reqwest_middleware::ClientWithMiddleware`]
//!
//! This is useful if you've already set up middleware like retry mechanisms.
//!
//! ```rust
//! use gleif_rs::client::GleifClient;
//! use reqwest::Client as ReqwestClient;
//! use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
//! use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};
//!
//! let reqwest_client = ReqwestClient::new();
//! let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
//!
//! // Your existing middleware-enhanced client
//! let preconfigured_middleware_client = ClientBuilder::new(reqwest_client)
//!     .with(RetryTransientMiddleware::new_with_policy(retry_policy))
//!     .build();
//!
//! let client = GleifClient::from_middleware_client(preconfigured_middleware_client);
//! // `client` now uses your sophisticated, pre-configured middleware client.
//! ```
//!
//! ## 4. Advanced Configuration with Custom HTTP Client and Middleware
//!
//! For full control, you can configure a [`reqwest::Client`] with specific settings (like timeouts),
//! wrap it with desired middleware, and then provide this to the [`GleifClient`] builder.
//!
//! ```rust
//! use gleif_rs::client::GleifClient;
//! use reqwest::ClientBuilder as ReqwestClientBuilder; // Renamed for clarity
//! use reqwest_middleware::ClientBuilder as MiddlewareClientBuilder; // Renamed for clarity
//! use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};
//! use std::time::Duration;
//!
//! // 1. Configure a custom reqwest client
//! let underlying_reqwest_client = ReqwestClientBuilder::new()
//!     .timeout(Duration::from_secs(30))       // Overall request timeout
//!     .connect_timeout(Duration::from_secs(5)) // Connection timeout
//!     .build()
//!     .expect("Failed to build reqwest client");
//!
//! // 2. Configure a retry policy
//! let retry_policy = ExponentialBackoff::builder()
//!     .build_with_max_retries(3); // Retry up to 3 times with exponential backoff
//!
//! // 3. Create the middleware stack
//! let middleware_client_builder = MiddlewareClientBuilder::new(underlying_reqwest_client)
//!     .with(RetryTransientMiddleware::new_with_policy(retry_policy));
//!     // You could add more .with(...) calls here for other middleware (e.g., logging)
//!
//! // 4. Build the GleifClient using the custom middleware builder
//! let client = GleifClient::builder()
//!     .base_url("[https://api.gleif.org/api/v1](https://api.gleif.org/api/v1)") // Or your preferred base URL
//!     .middleware_builder(middleware_client_builder)
//!     .build();
//!
//! // `client` is now highly customized with specific timeouts and retry logic.
//! ```
//!
//! ## Making API Requests
//!
//! Once you have an instance of [`crate::client::GleifClient`], you can use its methods to interact with the various GLEIF API endpoints.
//! These methods handle request construction, sending the request, and deserializing the response.
//! See [`crate::request_builder::GleifRequestBuilder`] for more details on building requests.
//!
//! ## Error Handling
//!
//! All methods return [`crate::error::Result`]. See the [`crate::error`] module for details.

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
    /// Create a new [`GleifClient`] with default settings.
    fn default() -> Self {
        // Unwrap is safe here because the default configuration is valid.
        GleifClientBuilder::new().build().unwrap()
    }
}

impl GleifClient {
    /// Create a new [`GleifClient`] with default configuration.
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
    pub(crate) fn request(&self, path: &str) -> GleifRequestBuilder {
        GleifRequestBuilder::new(self.clone(), Method::GET, path)
    }
}

/// Builder for configuring and constructing a [`GleifClient`].
///
/// Provides a fluent interface for customizing various aspects of the client
/// before construction, such as the base URL, middleware, and other options.
pub struct GleifClientBuilder {
    middleware_builder: Option<ClientBuilder>,
    reqwest_client: Option<ReqwestClient>,
    base_url: String,
}

impl Default for GleifClientBuilder {
    /// Create a new [`GleifClientBuilder`] with default settings.
    fn default() -> Self {
        Self {
            middleware_builder: None,
            reqwest_client: None,
            base_url: String::from(DEFAULT_BASE_URL),
        }
    }
}

impl GleifClientBuilder {
    /// Create a new [`GleifClientBuilder`] with default settings.
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

    /// Build the [`GleifClient`] with the configured settings.
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
