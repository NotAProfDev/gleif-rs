//! # GLEIF API Auto-Completion Endpoint
//!
//! This module provides functionality for interacting with the Auto-Completion endpoint of the GLEIF API.
//!
//! Auto-Completion is a search tool that returns suggestions for search terms based on their similarity
//! to the submitted search term. It is particularly useful for implementing type-ahead or predictive
//! search functionalities in applications, and helps users quickly find relevant LEI records.
//!
//! The module includes methods to:
//! - Perform an auto-completion search on specific fields of LEI records using a search term.
//! - Retrieve suggested search terms based on the supplied search term and field criteria.
//!
//! ## Endpoints
//!
//! - `/autocompletions`: Fetches suggested search terms for a given search term and field. This endpoint supports filtering by field and query, and returns a list of possible completions with highlights.
//!
//! ## Examples
//!
//! ```rust
//! use gleif_rs::{client::GleifClient};
//! use gleif_rs::model::AutoCompletionList;
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = GleifClient::new();
//!
//! // Fetch auto-completions for the term "Global" in the "fulltext" field
//! let results: AutoCompletionList = client
//!     .auto_completions("fulltext", "Global")
//!     .await?;
//!
//! // Fetch auto-completions and retrieve raw JSON
//! let results: serde_json::Value = client
//!     .auto_completions("fulltext", "Global")
//!     .await?;
//! # Ok(()) }
//! ```
//!
//! ## Errors
//!
//! Errors may occur during network communication, server-side issues, or deserialization of the response.
//! These errors are returned as [`crate::error::GleifError`] instances. All methods in this module are asynchronous and return a `Result` type, making them suitable for use in async Rust applications.

use crate::{client::GleifClient, error::Result};
use serde::de::DeserializeOwned;

impl GleifClient {
    /// Retrieves a list of auto-completed strings based on the supplied search term.
    ///
    /// This method sends a request to the `autocompletions` endpoint to fetch suggested search queries.
    /// The `field` and `q` parameters are mandatory for this endpoint. The `q` parameter, which contains the search term,
    /// can hold a maximum of 255 characters. The response includes a list of possibly relevant strings containing the
    /// search term, along with a highlighted version showing where the term appears in the retrieved strings.
    ///
    /// # Parameters
    ///
    /// - `field` - A string representing the field to search in. Any value other than those listed below will result in an invalid request:
    ///   - `fulltext` - Search in all fields of the Legal Entity's LEI Record
    ///   - `owns` - Search in LEI Records of "child" LEI Records
    ///   - `ownedBy` - Search in LEI Records of "parent" LEI Records
    /// - `q` - A string slice representing the search term (maximum 255 characters).
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
    /// let auto_completion: AutoCompletionList = client.auto_completions("fulltext", "Global").send().await?; // strongly typed
    /// let auto_completion: serde_json::Value = client.auto_completions("fulltext", "Global").send().await?; // raw JSON
    /// ```
    pub async fn auto_completions<R>(&self, field: &str, q: &str) -> Result<R>
    where
        R: DeserializeOwned,
    {
        self.request(&format!("autocompletions?field={field}&q={q}"))
            .send()
            .await
    }
}
