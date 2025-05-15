//! This module provides functionality for interacting with the Fuzzy Completions endpoint of the GLEIF API.
//!
//! Fuzzy Completions is a search tool that returns approximate results based on statistical similarity, rather than exact matching.
//! It is particularly useful for finding Legal Entity Identifiers (LEIs) or related records when the exact name or identifier is unknown.
//!
//! The module includes methods to:
//! - Perform a fuzzy search on specific fields of LEI records using a search term.
//! - Retrieve approximate matches based on the supplied search term and field criteria.
//!
//! # Endpoints
//!
//! - `/fuzzycompletions`: Fetches approximate matches for a given search term and field.
//!
//! # Examples
//!
//! ```rust
//! use gleif_rs::{client::GleifClient};
//! use gleif_rs::model::fuzzy_completion::FuzzyCompletion;
//! use gleif_rs::model::common::GleifApiResponse;
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = GleifClient::new();
//!
//! // Perform a fuzzy search on the `entity.legalName` field
//! let results: GleifApiResponse<Vec<FuzzyCompletion>> = client
//!     .fuzzy_completions("entity.legalName", "factbook")
//!     .await?;
//!
//! // Perform a fuzzy search and retrieve raw JSON
//! let results: serde_json::Value = client
//!     .fuzzy_completions("entity.legalName", "factbook")
//!     .await?;
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
    /// Retrieves a list of approximate matches based on the supplied search term, compared with the full text of LEI records.
    ///
    /// This method sends a request to the `fuzzycompletions` endpoint to fetch approximate matches. The `field` and `q` parameters
    /// are mandatory for this endpoint. The `q` parameter, which contains the search term, can hold a maximum of 255 characters.
    /// The response includes references to LEI records that contain strings similar to the search term.
    ///
    /// # Parameters
    ///
    /// * `field` - A string representing the field to search in. Any value other than those listed below will result in an invalid request:
    ///    * `entity.legalName` - Search only in the primary, official/registered legal name of the Legal Entity itself
    ///    * `fulltext` - Search in all fields of the Legal Entity's LEI Record
    ///    * `owns` - Search in LEI Records of "child" LEI Records
    ///    * `ownedBy` - Search in LEI Records of "parent" LEI Records.
    /// * `q` - A string slice representing the search term (maximum 255 characters).
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
    /// let fuzzy_completion: GleifApiResponse<Vec<FuzzyCompletion>> = client.fuzzy_completions("entity.legalName", "factbook").send().await?; // strongly typed
    /// let fuzzy_completion: serde_json::Value = client.fuzzy_completions("entity.legalName", "factbook").send().await?; // raw JSON
    /// ```
    pub async fn fuzzy_completions<R>(&self, field: &str, q: &str) -> Result<R>
    where
        R: DeserializeOwned,
    {
        self.request(&format!("fuzzycompletions?field={field}&q={q}"))
            .send()
            .await
    }
}
