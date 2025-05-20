//! # GLEIF API ISIN Endpoint
//!
//! This module provides functionality for interacting with the ISINs (International Securities Identification Numbers)
//! endpoint of the GLEIF API.
//!
//! ISINs are unique identifiers for securities, such as stocks and bonds, and are mapped to the legal entities
//! that issue them. This module allows users to retrieve ISINs associated with a specific Legal Entity Identifier (LEI),
//! supporting regulatory, compliance, and financial data integration use cases.
//!
//! ## Endpoints
//!
//! - `/lei-records/{lei}/isins`: Fetches all ISINs reported as issued by the entity identified by the given LEI. The endpoint supports pagination for large result sets.
//!
//! ## Examples
//!
//! Fetching ISINs for a specific LEI:
//!
//! ```rust
//! use gleif_rs::{client::GleifClient};
//! use gleif_rs::model::IsinList;
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = GleifClient::new();
//! let isins: IsinList = client.isins("5493000IBP32UQZ0KL24").send().await?; // strongly typed
//! let isins: serde_json::Value = client.isins("5493000IBP32UQZ0KL24").send().await?; // raw JSON
//! # Ok(()) }
//! ```
//!
//! This module provides a convenient interface for accessing ISIN data, enabling users to explore securities
//! issued by legal entities efficiently. All methods in this module are asynchronous and return a `Result` type, making them suitable for use in async Rust applications.

use crate::{client::GleifClient, request_builder::GleifRequestBuilder};

impl GleifClient {
    /// Retrieves ISINs (International Securities Identification Numbers) reported as issued by the entity
    /// identified by this LEI (Legal Entity Identifier) registration.
    ///
    /// This method sends a request to the `/lei-records/{lei}/isins` endpoint to fetch ISINs associated
    /// with the provided LEI identifier. The response includes all ISINs mapped to this LEI record.
    /// Pagination parameters can be used to manage large datasets.
    ///
    /// # Parameters
    ///
    /// - `lei` - A string slice representing the LEI identifier.
    ///
    /// # Errors
    ///
    /// This method does not itself return errors. However, errors may occur when sending the request or processing
    /// the response using the returned request builder (e.g., network failures or deserialization issues).
    ///
    /// # Examples
    ///
    /// ```rust, ignore
    /// let isins: IsinList = client.isins("5493000IBP32UQZ0KL24").send().await?; // strongly typed
    /// let isins: serde_json::Value = client.isins("5493000IBP32UQZ0KL24").send().await?; // raw JSON
    /// ```
    #[must_use]
    pub fn isins(&self, lei: &str) -> GleifRequestBuilder {
        self.request(&format!("lei-records/{lei}/isins"))
    }
}
