//! # GLEIF API VLEI Issuers Endpoints
//!
//! This module provides functionality for interacting with the vLEI Issuers endpoint of the GLEIF API.
//!
//! vLEI Issuers, also known as Qualified vLEI Issuing Organizations (QVIs), are entities authorized by
//! the Global Legal Entity Identifier Foundation (GLEIF) to issue and revoke vLEI Credentials. These
//! credentials are used by legal entities to establish their identity in a verifiable and standardized
//! manner, supporting digital trust, compliance, and secure business transactions.
//!
//! The module includes methods to:
//! - Retrieve a list of all qualified vLEI Issuers, including their identification and descriptive data, with support for pagination.
//! - Fetch detailed information about a specific vLEI Issuer using its LEI (Legal Entity Identifier), including legal name, marketing name, website, and qualification date.
//!
//! ## Endpoints
//!
//! - `/vlei-issuers`: Fetches a list of all qualified vLEI Issuers. Supports pagination for large result sets.
//! - `/vlei-issuers/{lei}`: Fetches detailed information about a specific vLEI Issuer by its LEI, returning comprehensive metadata.
//!
//! ## Examples
//!
//! ```rust
//! use gleif_rs::{client::GleifClient};
//! use gleif_rs::model::{VLeiIssuer, VLeiIssuerList};
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = GleifClient::new();
//!
//! // Fetch all vLEI Issuers
//! let vlei_issuers: VLeiIssuerList = client.vlei_issuers().send().await?; // strongly typed
//! let vlei_issuers: serde_json::Value = client.vlei_issuers().send().await?; // raw JSON
//!
//! // Fetch a specific vLEI Issuer by LEI
//! let vlei_issuer: VLeiIssuer = client.vlei_issuer_by_id("5493000IBP32UQZ0KL24").await?;
//! # Ok(()) }
//! ```
//!
//! ## Errors
//!
//! Errors may occur during network communication, server-side issues, or deserialization of the response.
//! These errors are returned as [`crate::error::GleifError`] instances. All methods in this module are asynchronous and return a `Result` type, making them suitable for use in async Rust applications.

use crate::{client::GleifClient, error::Result, request_builder::GleifRequestBuilder};
use serde::de::DeserializeOwned;

impl GleifClient {
    /// Retrieves a list of qualified vLEI Issuers, including their identification and descriptive data.
    ///
    /// This method sends a request to the `/vlei-issuers` endpoint to fetch all qualified vLEI Issuers.
    /// The response includes details such as the issuer's LEI, legal name, marketing name, website, and
    /// the date they were officially qualified by GLEIF.
    ///
    /// # Errors
    ///
    /// This method does not itself return errors. However, errors may occur when sending the request or processing
    /// the response using the returned request builder (e.g., network failures or deserialization issues).
    ///
    /// # Examples
    ///
    /// ```rust, ignore
    /// let vlei_issuers: VLeiIssuerList = client.vlei_issuers().send().await?;
    /// let vlei_issuers: serde_json::Value = client.vlei_issuers().send().await?;
    /// ```
    #[must_use]
    pub fn vlei_issuers(&self) -> GleifRequestBuilder {
        self.request("vlei-issuers")
    }

    /// Fetches detailed information about a specific qualified vLEI Issuer using its LEI identifier.
    ///
    /// This method sends a request to the `/vlei-issuers/{lei}` endpoint to retrieve information about
    /// a single vLEI Issuer. The response includes attributes such as the issuer's LEI, legal name,
    /// marketing name, website, and the date they were officially qualified by GLEIF.
    ///
    /// # Parameters
    ///
    /// - `lei` - A string slice representing the LEI identifier of the vLEI Issuer.
    ///
    /// # Errors
    ///
    /// This method returns a [`crate::error::GleifError`] in the following cases:
    /// - The request could not be completed due to network or server issues.
    /// - The response body could not be deserialized into the expected type.
    ///
    /// # Examples
    ///
    /// ```rust, ignore
    /// let vlei_issuer: VLeiIssuer = client.vlei_issuer_by_id("5493000IBP32UQZ0KL24").await?; // strongly typed
    /// let vlei_issuer: serde_json::Value = client.vlei_issuer_by_id("5493000IBP32UQZ0KL24").await?; // raw JSON
    /// ```
    pub async fn vlei_issuer_by_id<R>(&self, lei: &str) -> Result<R>
    where
        R: DeserializeOwned,
    {
        self.request(&format!("vlei-issuers/{lei}")).send().await
    }
}
