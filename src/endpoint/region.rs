//! # GLEIF API Region Metadata Endpoints
//!
//! This module provides functionality for interacting with the Regions endpoint of the GLEIF API.
//!
//! The Regions endpoint contains ISO 3166 Sub-Region Codes. The Sub-Region Code is an alpha-numeric code
//! of two characters from the basic Latin character set. These codes are used to identify specific
//! sub-regions within countries (e.g., "US-CA" for California in the United States), supporting standardized regional data for regulatory, compliance, and reporting purposes.
//!
//! The module includes methods to:
//! - Fetch a list of all ISO 3166 Region Codes, with support for pagination.
//! - Fetch detailed information about a specific region using its unique code, including region metadata and country association.
//!
//! ## Endpoints
//!
//! - `/regions`: Fetches a list of all ISO 3166 Region Codes. Supports pagination for large result sets.
//! - `/regions/{id}`: Fetches detailed information about a specific region by its code, returning comprehensive metadata.
//!
//! ## Examples
//!
//! ```rust
//! use gleif_rs::{client::GleifClient};
//! use gleif_rs::model::{Region, RegionList};
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = GleifClient::new();
//!
//! // Fetch all regions
//! let regions: RegionList = client.regions().send().await?; // strongly typed
//! let regions: serde_json::Value = client.regions().send().await?; // raw JSON
//!
//! // Fetch a specific region by its code
//! let region: Region = client.region_by_id("AD-03").await?;
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
    /// Retrieves all ISO 3166 Region Codes (`/regions`).
    ///
    /// This method sends a request to fetch a list of all ISO 3166 Region Codes.
    /// Pagination parameters can be used to manage the large number of data items.
    ///
    /// # Errors
    ///
    /// This method does not itself return errors. However, errors may occur when sending the request or processing
    /// the response using the returned request builder (e.g., network failures or deserialization issues).
    ///
    /// # Examples
    ///
    /// ```rust, ignore
    /// let regions: RegionList = client.regions().send().await?; // strongly typed
    /// let regions: serde_json::Value = client.regions().send().await?; // raw JSON
    /// ```
    #[must_use]
    pub fn regions(&self) -> GleifRequestBuilder {
        self.request("regions")
    }

    /// Fetches details of a single region by ISO 3166 Region Code (`/regions/{id}`).
    ///
    /// This method sends a request to retrieve information about a specific region by its code.
    ///
    /// # Parameters
    ///
    /// * `id` - A string slice representing the unique ISO 3166 Region Code.
    ///
    /// # Errors
    ///
    /// This method returns a [`crate::error::GleifError`] in the following cases:
    /// * The request could not be completed due to network or server issues.
    /// * The response body could not be deserialized into the expected type.
    ///
    /// # Examples
    ///
    /// ```rust, ignore
    /// let region: Region = client.region_by_id("AD-03").await?; // strongly typed
    /// let region: serde_json::Value = client.region_by_id("AD-03").await?; // raw JSON
    /// ```
    pub async fn region_by_id<R>(&self, id: &str) -> Result<R>
    where
        R: DeserializeOwned,
    {
        self.request(&format!("regions/{id}")).send().await
    }
}
