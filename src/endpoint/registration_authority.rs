//! Endpoints for GLEIF API registration authority metadata.
//!
//! The Registration Authorities endpoint contains Registration Authority (RA) Codes based on the GLEIF Registration Authority (RA) Code List.
//! This module provides functionality for interacting with the Registration Authorities endpoint of the GLEIF API.
//!
//! Registration Authorities are entities responsible for assigning and maintaining unique identifiers for legal entities.
//! They play a critical role in ensuring the integrity and consistency of legal entity data.
//!
//! The module includes methods to:
//! - Fetch a list of all Registration Authorities, optionally filtered by specific criteria.
//! - Fetch detailed information about a specific Registration Authority using its RA List Code.
//!
//! # Endpoints
//!
//! - `/registration-authorities`: Fetches a list of all Registration Authorities.
//! - `/registration-authorities/{id}`: Fetches detailed information about a specific Registration Authority by its RA List Code.
//!
//! # Examples
//!
//! ```rust
//! use gleif_rs::{client::GleifClient};
//! use gleif_rs::model::{RegistrationAuthority, RegistrationAuthorityList};
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = GleifClient::new();
//!
//! // Fetch all Registration Authorities
//! let authorities: RegistrationAuthorityList = client.registration_authorities().send().await?; // strongly typed
//! let authorities: serde_json::Value = client.registration_authorities().send().await?; // raw JSON
//!
//! // Fetch a specific Registration Authority by RA List Code
//! let authority: RegistrationAuthority = client.registration_authority_by_id("RA000001").await?;
//! # Ok(()) }
//! ```
//!
//! # Errors
//!
//! Errors may occur during network communication, server-side issues, or deserialization of the response.
//! These errors are returned as [`crate::error::GleifError`] instances.

use crate::{client::GleifClient, error::Result, request_builder::GleifRequestBuilder};
use serde::de::DeserializeOwned;

impl GleifClient {
    /// Retrieves all Registration Authorities (`/registration-authorities`).
    ///
    /// This method sends a request to fetch the complete list of Registration Authorities based on
    /// the GLEIF Registration Authority (RA) Code List. Pagination parameters can be used to manage
    /// the large number of data items.
    ///
    /// # Errors
    ///
    /// This method does not itself return errors. However, errors may occur when sending the request or processing
    /// the response using the returned request builder (e.g., network failures or deserialization issues).
    ///
    /// # Examples
    ///
    /// ```rust, ignore
    /// let authorities: RegistrationAuthorityList = client.registration_authorities().send().await?; // strongly typed
    /// let authorities: serde_json::Value = client.registration_authorities().send().await?; // raw JSON
    /// ```
    #[must_use]
    pub fn registration_authorities(&self) -> GleifRequestBuilder {
        self.request("registration-authorities")
    }

    /// Fetches details of a single Registration Authority by RA List Code (`/registration-authorities/{id}`).
    ///
    /// This method sends a request to retrieve information about a specific registration authority by its code.
    ///
    /// # Parameters
    ///
    /// * `id` - A string slice representing the unique RA List Code of the registration authority.
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
    /// let authority: RegistrationAuthority = client.registration_authority_by_id("RA000001").await?; // strongly typed
    /// let authority: serde_json::Value = client.registration_authority_by_id("RA000001").await?; // raw JSON
    /// ```
    pub async fn registration_authority_by_id<R>(&self, id: &str) -> Result<R>
    where
        R: DeserializeOwned,
    {
        self.request(&format!("registration-authorities/{id}"))
            .send()
            .await
    }
}
