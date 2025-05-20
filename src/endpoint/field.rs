//! # GLEIF API Fields Metadata Endpoints
//!
//! This module provides functionality for interacting with the GLEIF API's fields metadata.
//! Fields represent the individual data points available for filtering and querying LEI (Legal Entity Identifier) records, and are essential for constructing precise and efficient queries.
//!
//! The provided methods allow users to:
//! - Retrieve a list of all available fields for filtering LEI records, including their metadata and usage information.
//! - Fetch detailed information about a specific field by its unique identifier, such as its data type, description, and allowed values.
//!
//! ## Endpoints
//!
//! - `/fields`: Fetches a list of all available fields. Supports pagination for large result sets.
//! - `/fields/{id}`: Fetches details of a specific field by its ID, returning comprehensive metadata for that field.
//!
//! ## Examples
//!
//! ```rust
//! use gleif_rs::{client::GleifClient};
//! use gleif_rs::model::{Field, FieldList};
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = GleifClient::new();
//!
//! // Fetch all available fields
//! let fields: FieldList = client.fields().send().await?; // strongly typed
//! let fields: serde_json::Value = client.fields().send().await?; // raw JSON
//!
//! // Fetch details of a specific field
//! let field: Field = client.field_by_id("LEIREC_LEGAL_NAME").await?;
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
    /// Fetches a list of all available data fields for filtering LEI records (`/fields`).
    ///
    /// This endpoint provides detailed documentation of the data fields available in the API.
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
    /// let fields: FieldList = client.fields().send().await?; // strongly typed
    /// let fields: serde_json::Value = client.fields().send().await?; // raw JSON
    /// ```
    #[must_use]
    pub fn fields(&self) -> GleifRequestBuilder {
        self.request("fields")
    }

    /// Fetches details of a single data field for filtering LEI records (`/fields/{id}`).
    ///
    /// This method sends a request to retrieve information about a specific field by its ID.
    ///
    /// # Parameters
    ///
    /// * `id` - A string slice representing the unique identifier of the field.
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
    /// let field: Field = client.field_by_id("LEIREC_LEGAL_NAME").await?; // strongly typed
    /// let field: serde_json::Value = client.field_by_id("LEIREC_LEGAL_NAME").await?; // raw JSON
    /// ```
    pub async fn field_by_id<R>(&self, id: &str) -> Result<R>
    where
        R: DeserializeOwned,
    {
        self.request(&format!("fields/{id}")).send().await
    }
}
