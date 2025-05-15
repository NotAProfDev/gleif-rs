//! This module provides functionality for interacting with the reporting exception endpoints of the GLEIF API.
//!
//! Reporting exceptions indicate cases where a legal entity has declined to report a direct or ultimate
//! accounting consolidation parent, based on applicable accounting standards. These exceptions are recorded
//! and made available through the GLEIF API.
//!
//! The module includes methods to:
//! - Retrieve the reporting exception record for a specific LEI's direct parent.
//! - Retrieve the reporting exception record for a specific LEI's ultimate parent.
//!
//! # Endpoints
//!
//! - `/lei-records/{lei}/direct-parent-reporting-exception`: Fetches the reporting exception for a direct parent.
//! - `/lei-records/{lei}/ultimate-parent-reporting-exception`: Fetches the reporting exception for an ultimate parent.
//!
//! # Examples
//!
//! ```rust
//! use gleif_rs::{client::GleifClient};
//! use gleif_rs::model::ReportingException;
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = GleifClient::new();
//!
//! // Fetch the direct parent reporting exception for a specific LEI
//! let exception: ReportingException = client.direct_parent_reporting_exception("5493000IBP32UQZ0KL24").await?;
//!
//! // Fetch the ultimate parent reporting exception for a specific LEI
//! let exception: ReportingException = client.ultimate_parent_reporting_exception("5493000IBP32UQZ0KL24").await?;
//! # Ok(()) }
//! ```
//!
//! # Errors
//!
//! Errors may occur during network communication, server-side issues, or deserialization of the response.
//! These errors are returned as [`GleifError`] instances.

use crate::{client::GleifClient, error::Result};
use serde::de::DeserializeOwned;

impl GleifClient {
    /// Retrieves the reporting exception record for a specific LEI (Legal Entity Identifier).
    ///
    /// This method sends a request to the `/lei-records/{lei}/direct-parent-reporting-exception` endpoint
    /// to fetch the record indicating that the legal entity has declined to report a direct accounting
    /// consolidation parent, based on applicable accounting standards.
    ///
    /// # Parameters
    ///
    /// * `lei` - A string slice representing the LEI identifier.
    ///
    /// # Errors
    ///
    /// This method returns a [`GleifError`] in the following cases:
    /// * The request could not be completed due to network or server issues.
    /// * The response body could not be deserialized into the expected type.
    ///
    /// # Examples
    ///
    /// ```rust, ignore
    /// let exception: ReportingException = client.direct_parent_reporting_exception("5493000IBP32UQZ0KL24").await?;
    /// let exception: serde_json::Value = client.direct_parent_reporting_exception("5493000IBP32UQZ0KL24").await?; // raw JSON
    /// ```
    pub async fn direct_parent_reporting_exception<R>(&self, lei: &str) -> Result<R>
    where
        R: DeserializeOwned,
    {
        self.request(&format!(
            "lei-records/{lei}/direct-parent-reporting-exception"
        ))
        .send()
        .await
    }

    /// Retrieves the reporting exception record for the ultimate parent of a specific LEI (Legal Entity Identifier).
    ///
    /// This method sends a request to the `/lei-records/{lei}/ultimate-parent-reporting-exception` endpoint
    /// to fetch the record indicating that the legal entity has declined to report an ultimate accounting
    /// consolidation parent, based on applicable accounting standards.
    ///
    /// # Parameters
    ///
    /// * `lei` - A string slice representing the LEI identifier.
    ///
    /// # Errors
    ///
    /// This method returns a [`GleifError`] in the following cases:
    /// * The request could not be completed due to network or server issues.
    /// * The response body could not be deserialized into the expected type.
    ///
    /// # Examples
    ///
    /// ```rust, ignore
    /// let exception: ReportingException = client.ultimate_parent_reporting_exception("5493000IBP32UQZ0KL24").await?; // strongly typed
    /// let exception: serde_json::Value = client.ultimate_parent_reporting_exception("5493000IBP32UQZ0KL24").await?; // raw JSON
    /// ```
    pub async fn ultimate_parent_reporting_exception<R>(&self, lei: &str) -> Result<R>
    where
        R: DeserializeOwned,
    {
        self.request(&format!(
            "lei-records/{lei}/ultimate-parent-reporting-exception"
        ))
        .send()
        .await
    }
}
