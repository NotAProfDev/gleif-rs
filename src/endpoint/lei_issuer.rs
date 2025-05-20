//! # GLEIF API LEI Issuers Endpoints
//!
//! This module provides functionality for interacting with the LEI Issuers endpoint of the GLEIF API.
//!
//! LEI Issuers, also referred to as Local Operating Units (LOUs), are entities authorized to supply
//! registration, renewal, and other services related to Legal Entity Identifiers (LEIs). They act as
//! the primary interface for legal entities wishing to obtain or manage an LEI, and are accredited by GLEIF to operate in specific jurisdictions.
//!
//! The module includes methods to:
//! - Fetch a list of all LEI Issuers, optionally filtered by specific criteria, with support for pagination.
//! - Fetch detailed information about a specific LEI Issuer using its LEI, including name, country, and accreditation details.
//! - Retrieve all jurisdictions for which a specific LEI Issuer is accredited, providing insight into the issuer's operational scope.
//! - Retrieve identification and descriptive data of the LEI Issuer responsible for administering a specific LEI, enabling traceability of LEI management.
//!
//! ## Endpoints
//!
//! - `/lei-issuers`: Fetches a list of all LEI Issuers. Supports pagination and filtering.
//! - `/lei-issuers/{lei}`: Fetches detailed information about a specific LEI Issuer by its LEI.
//! - `/lei-issuers/{lei}/jurisdictions`: Fetches jurisdictions for which the LEI Issuer is accredited.
//! - `/lei-records/{lei}/lei-issuer`: Fetches the LEI Issuer responsible for a specific LEI.
//!
//! ## Examples
//!
//! ```rust
//! use gleif_rs::{client::GleifClient};
//! use gleif_rs::model::{LeiIssuer, LeiIssuerList, LeiIssuerJurisdictionList};
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = GleifClient::new();
//!
//! // Fetch all LEI Issuers
//! let issuers: LeiIssuerList = client.lei_issuers().send().await?; // strongly typed
//! let issuers: serde_json::Value = client.lei_issuers().send().await?; // raw JSON
//!
//! // Fetch a specific LEI Issuer by LEI
//! let issuer: LeiIssuer = client.lei_issuer_by_id("5493000IBP32UQZ0KL24").await?;
//!
//! // Fetch jurisdictions for a specific LEI Issuer
//! let jurisdictions: LeiIssuerJurisdictionList = client
//!     .lei_issuer_jurisdictions("549300IBP32UQZ0KL24")
//!     .send()
//!     .await?;
//!
//! // Fetch the LEI Issuer responsible for a specific LEI
//! let issuer: LeiIssuer = client
//!     .lei_issuer("5493000IBP32UQZ0KL24")
//!     .await?;
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
    /// Retrieves identification and descriptive data of the LEI Issuer responsible for administering a specific LEI registration.
    ///
    /// This method sends a request to the `/lei-records/{lei}/lei-issuer` endpoint to fetch the LEI issuer
    /// associated with the provided LEI identifier. The response includes details about the LEI issuer,
    /// such as its name, country, and other relevant attributes.
    ///
    /// # Parameters
    ///
    /// * `lei` - A string slice representing the LEI identifier.
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
    /// let issuer: LeiIssuer = client.lei_issuer("5493000IBP32UQZ0KL24").await?; // strongly typed
    /// let issuer: serde_json::Value = client.lei_issuer("5493000IBP32UQZ0KL24").await?; // raw JSON
    /// ```
    pub async fn lei_issuer<R>(&self, lei: &str) -> Result<R>
    where
        R: DeserializeOwned,
    {
        self.request(&format!("lei-records/{lei}/lei-issuer"))
            .send()
            .await
    }

    /// Retrieves a list of LEI Issuers (Legal Entity Identifier Issuers), optionally filtered by specific criteria.
    ///
    /// This method sends a request to the `/lei-issuers` endpoint to fetch LEI Issuers. If no filtering
    /// parameters are provided, it will return all available LEI Issuers. Pagination parameters can be
    /// used to manage large datasets. The response includes details about each LEI Issuer, such as its
    /// name, country, and other relevant attributes.
    ///
    /// # Errors
    ///
    /// This method does not itself return errors. However, errors may occur when sending the request or processing
    /// the response using the returned request builder (e.g., network failures or deserialization issues).
    ///
    /// # Examples
    ///
    /// ```rust, ignore
    /// let issuers: LeiIssuerList = client.lei_issuers().send().await?; // strongly typed
    /// let issuers: serde_json::Value = client.lei_issuers().send().await?; // raw JSON
    /// ```
    #[must_use]
    pub fn lei_issuers(&self) -> GleifRequestBuilder {
        self.request("lei-issuers")
    }

    /// Fetch a single LEI issuer by LEI (`/lei-issuers/{lei}`).
    ///
    /// This method retrieves detailed information about a specific LEI issuer using its LEI identifier.
    /// The response includes attributes such as the issuer's name, country, and other relevant details.
    ///
    /// # Parameters
    ///
    /// * `lei` - A string slice representing the LEI identifier of the issuer.
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
    /// let issuer: LeiIssuer = client.lei_issuer_by_id("5493000IBP32UQZ0KL24").await?; // strongly typed
    /// let issuer: serde_json::Value = client.lei_issuer_by_id("5493000IBP32UQZ0KL24").await?; // raw JSON
    /// ```
    pub async fn lei_issuer_by_id<R>(&self, lei: &str) -> Result<R>
    where
        R: DeserializeOwned,
    {
        self.request(&format!("lei-issuers/{lei}")).send().await
    }

    /// Retrieves all jurisdictions for which the LEI Issuer is accredited.
    ///
    /// This method sends a request to the `/lei-issuers/{lei}/jurisdictions` endpoint to fetch the
    /// jurisdictions associated with the specified LEI issuer. The response includes details about
    /// the jurisdictions where the issuer is accredited to operate.
    ///
    /// # Parameters
    ///
    /// * `lei` - A string slice representing the LEI identifier of the issuer.
    ///
    /// # Errors
    ///
    /// This method does not itself return errors. However, errors may occur when sending the request or processing
    /// the response using the returned request builder (e.g., network failures or deserialization issues).
    ///
    /// # Examples
    ///
    /// ```rust, ignore
    /// let jurisdictions: LeiIssuerJurisdictionsList = client
    ///     .lei_issuer_jurisdictions("5493000IBP32UQZ0KL24")
    ///     .send()
    ///     .await?;
    /// let jurisdictions: serde_json::Value = client
    ///     .lei_issuer_jurisdictions("5493000IBP32UQZ0KL24")
    ///     .send()
    ///     .await?;
    /// ```
    #[must_use]
    pub fn lei_issuer_jurisdictions(&self, lei: &str) -> GleifRequestBuilder {
        self.request(&format!("lei-issuers/{lei}/jurisdictions"))
    }
}
