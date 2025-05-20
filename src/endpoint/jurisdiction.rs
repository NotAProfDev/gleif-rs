//! # GLEIF API Jurisdiction Metadata Endpoints
//!
//! The Jurisdictions endpoint contains legal jurisdictions based on the ISO 3166 Country and Sub-Region Codes.
//! This module provides functionality for interacting with the Jurisdictions endpoint of the GLEIF API, enabling applications to access authoritative jurisdiction metadata for global entities.
//!
//! Jurisdictions represent the legal and administrative regions where entities operate. These are
//! defined using ISO 3166 standards for countries and sub-regions, ensuring consistency and interoperability in legal and regulatory contexts.
//!
//! The module includes methods to:
//! - Fetch a list of all jurisdictions, supporting pagination for large result sets.
//! - Fetch detailed information about a specific jurisdiction using its unique code, including metadata and region details.
//!
//! ## Endpoints
//!
//! - `/jurisdictions`: Fetches a list of all jurisdictions. The endpoint supports pagination and returns metadata for each jurisdiction.
//! - `/jurisdictions/{id}`: Fetches detailed information about a specific jurisdiction by its code, returning comprehensive metadata.
//!
//! ## Examples
//!
//! ```rust
//! use gleif_rs::{client::GleifClient};
//! use gleif_rs::model::{Jurisdiction, JurisdictionList};
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = GleifClient::new();
//!
//! // Fetch all jurisdictions
//! let jurisdictions: JurisdictionList = client.jurisdictions().send().await?; // strongly typed
//! let jurisdictions: serde_json::Value = client.jurisdictions().send().await?; // raw JSON
//!
//! // Fetch a specific jurisdiction by its code
//! let jurisdiction: Jurisdiction = client.jurisdiction_by_id("US").await?;
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
    /// Fetches a list of all jurisdictions (`/jurisdictions`).
    ///
    /// This method sends a request to retrieve all legal jurisdictions based on the ISO 3166 Country
    /// and Sub-Region Codes. Pagination parameters can be used to manage the large number of data items.
    ///
    /// # Errors
    ///
    /// This method does not itself return errors. However, errors may occur when sending the request or processing
    /// the response using the returned request builder (e.g., network failures or deserialization issues).
    ///
    /// # Examples
    ///
    /// ```rust, ignore
    /// let jurisdictions: JurisdictionList = client.jurisdictions().send().await?; // strongly typed
    /// let jurisdictions: serde_json::Value = client.jurisdictions().send().await?; // raw JSON
    /// ```
    #[must_use]
    pub fn jurisdictions(&self) -> GleifRequestBuilder {
        self.request("jurisdictions")
    }

    /// Fetches details of a single jurisdiction by jurisdiction code (`/jurisdictions/{id}`).
    ///
    /// This method sends a request to retrieve information about a specific jurisdiction by its code.
    ///
    /// # Parameters
    ///
    /// - `id` - A string slice representing the unique jurisdiction code.
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
    /// let jurisdiction: Jurisdiction = client.jurisdiction_by_id("US").await?; // strongly typed
    /// let jurisdiction: serde_json::Value = client.jurisdiction_by_id("US").await?; // raw JSON
    /// ```
    pub async fn jurisdiction_by_id<R>(&self, id: &str) -> Result<R>
    where
        R: DeserializeOwned,
    {
        self.request(&format!("jurisdictions/{id}")).send().await
    }
}
