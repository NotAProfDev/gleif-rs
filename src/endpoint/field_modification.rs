//! Field Modifications Metadata Endpoints
//!
//! This module provides functionality for interacting with the GLEIF API's field modifications metadata.
//! Field modifications represent changes made to specific fields of LEI (Legal Entity Identifier) records.
//!
//! The provided methods allow users to:
//! - Retrieve a list of field modifications for a specific LEI record.
//! - Filter the modifications based on various criteria, such as record type, modification date, or field name.
//!
//! # Endpoints
//!
//! - `/lei-records/{lei}/field-modifications`: Fetches field modifications for a specific LEI record.
//!
//! # Examples
//!
//! ```rust
//! use gleif_rs::{client::GleifClient};
//! use gleif_rs::model::field_modification::FieldModification;
//! use gleif_rs::model::common::GleifApiResponse;
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = GleifClient::new();
//!
//! // Fetch field modifications for a specific LEI
//! let modifications: GleifApiResponse<Vec<FieldModification>> = client.field_modifications("5493000IBP32UQZ0KL24").send().await?; // strongly typed
//! let modifications: serde_json::Value = client.field_modifications("5493000IBP32UQZ0KL24").send().await?; // raw JSON
//! # Ok(()) }
//! ```
//!
//! # Errors
//!
//! Errors may occur during network communication, server-side issues, or deserialization of the response.
//! These errors are returned as [`GleifError`] instances.

use crate::{client::GleifClient, request_builder::GleifRequestBuilder};

impl GleifClient {
    /// Fetches field modifications for a specific LEI (Legal Entity Identifier).
    ///
    /// This method sends a request to the `/lei-records/{lei}/field-modifications` endpoint to retrieve
    /// details about modifications made to the fields of the specified LEI record. The response can be
    /// filtered to include only a subset of field modifications.
    ///
    /// Supported filters:
    /// - `recordType`
    /// - `modificationDate`
    /// - `field`
    /// - `date`
    ///
    /// # Parameters
    ///
    /// * `lei` - A string slice representing the LEI identifier.
    ///
    /// # Errors
    ///
    /// This method does not itself return errors. However, errors may occur when sending the request or processing
    /// the response using the returned request builder (e.g., network failures or deserialization issues).
    ///
    /// # Examples
    ///
    /// ```rust, ignore
    /// let modifications: GleifApiResponse<Vec<LeiFieldModification>> = client.field_modifications("5493000IBP32UQZ0KL24").send().await?; // strongly typed
    /// let modifications: serde_json::Value = client.field_modifications("5493000IBP32UQZ0KL24").send().await?; // raw JSON
    /// ```
    #[must_use]
    pub fn field_modifications(&self, lei: &str) -> GleifRequestBuilder {
        self.request(&format!("lei-records/{lei}/field-modifications"))
    }
}
