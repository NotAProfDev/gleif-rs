//! # GLEIF API Country Metadata Endpoints
//!
//! This module provides functionality to interact with the GLEIF API's country metadata endpoints.
//! It allows users to retrieve information about countries based on the ISO 3166 Country Codes, which are internationally recognized codes for the representation of country names and territories.
//!
//! The GLEIF API provides access to country metadata, including a list of all ISO 3166 Country Codes
//! and detailed information about individual countries. These endpoints are useful for applications
//! that require standardized country data for legal, regulatory, or organizational purposes, ensuring consistency and accuracy in country identification.
//!
//! ## Endpoints
//!
//! - `/countries`: Retrieve a list of all ISO 3166 Country Codes. Supports pagination for large result sets.
//! - `/countries/{id}`: Fetch details of a specific country by its ISO 3166 Country Code. Returns comprehensive metadata for the requested country.
//!
//! ## Examples
//!
//! ```rust
//! use gleif_rs::{client::GleifClient};
//! use gleif_rs::model::{Country, CountryList};
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = GleifClient::new();
//!
//! // Retrieve all countries
//! let countries: CountryList = client.countries().send().await?; // strongly typed
//! let countries: serde_json::Value = client.countries().send().await?; // raw JSON
//!
//! // Retrieve details of a specific country
//! let country: Country = client.country_by_id("US").await?;
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
    /// Retrieves all ISO 3166 Country Codes.
    ///
    /// This method sends a request to the `/countries` endpoint to fetch a list of all ISO 3166 Country Codes.
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
    /// let countries: CountryList = client.countries().send().await?; // strongly typed
    /// let countries: serde_json::Value = client.countries().send().await?; // raw JSON
    /// ```
    #[must_use]
    pub fn countries(&self) -> GleifRequestBuilder {
        self.request("countries")
    }

    /// Fetches ISO 3166 Country Code details by ISO 3166 Country Code (`/countries/{id}`).
    ///
    /// This method sends a request to retrieve details of a single country by its ISO 3166 code.
    ///
    /// # Parameters
    ///
    /// - `id` - A string slice representing the ISO 3166 Country Code.
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
    /// let country: Country = client.country_by_id("US").await?; // strongly typed
    /// let country: serde_json::Value = client.country_by_id("US").await?; // raw JSON
    /// ```
    pub async fn country_by_id<R>(&self, id: &str) -> Result<R>
    where
        R: DeserializeOwned,
    {
        self.request(&format!("countries/{id}")).send().await
    }
}
