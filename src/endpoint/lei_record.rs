//! # GLEIF API LEI Records Endpoints
//!
//! This module provides functionality for interacting with the `lei-records` endpoint of the GLEIF API.
//!
//! The `lei-records` endpoint allows access to all Legal Entity Identifier (LEI) records by their ID (the LEI Code itself),
//! by applying filters, and by exploring relationships to other LEI records (links and Level 2 Data). It supports both direct and hierarchical queries, enabling users to traverse entity relationships.
//!
//! LEI records provide essential information about legal entities, including their hierarchical relationships,
//! parent and child entities, and associated entities such as fund managers or successor entities. These records are critical for regulatory, compliance, and financial applications.
//!
//! ## Endpoints
//!
//! - `/lei-records/{lei}`: Fetches a specific LEI record by its identifier.
//! - `/lei-records`: Retrieves a list of LEI records, optionally filtered by specific criteria.
//! - `/lei-records/{lei}/ultimate-parent`: Fetches the LEI record for the ultimate parent of a specific entity.
//! - `/lei-records/{lei}/direct-parent`: Fetches the LEI record for the direct parent of a specific entity.
//! - `/lei-records/{lei}/ultimate-children`: Fetches LEI records for the ultimate children of a specific entity.
//! - `/lei-records/{lei}/direct-children`: Fetches LEI records for the direct children of a specific entity.
//! - `/lei-records/{lei}/associated-entity`: Fetches the LEI record for the manager of a specific legal entity (fund).
//! - `/lei-records/{lei}/successor-entity`: Fetches the LEI record for the successor entity of a specific legal entity.
//! - `/lei-records/{lei}/managing-lou`: Fetches the LEI record for the managing Local Operating Unit (LOU) of a specific entity.
//!
//! ## Examples
//!
//! ```rust
//! use gleif_rs::{client::GleifClient};
//! use gleif_rs::model::{LeiRecord, LeiRecordList};
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = GleifClient::new();
//!
//! // Fetching a specific LEI record
//! let record: LeiRecord = client.lei_record_by_id("5493000IBP32UQZ0KL24").await?; // strongly typed
//! let record: serde_json::Value = client.lei_record_by_id("5493000IBP32UQZ0KL24").await?; // raw JSON
//!
//! // Fetching LEI records with filters
//! let records: LeiRecordList = client.lei_records().filter_eq("status", "ACTIVE").send().await?;
//!
//! // Fetching parent and child relationships
//! let parent: LeiRecord = client.ultimate_parent("5493000IBP32UQZ0KL24").await?;
//! let children: LeiRecordList = client.direct_children("5493000IBP32UQZ0KL24").send().await?;
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
    /// Retrieves a specific LEI (Legal Entity Identifier) record by its identifier.
    ///
    /// This method sends a request to the `/lei-records/{lei}` endpoint to fetch the LEI record
    /// associated with the provided identifier. The response may include URLs to retrieve
    /// associated Level 2 (relationship) data, if available, within the `links` section of the response body.
    ///
    /// # Parameters
    ///
    /// - `lei` - A string slice representing the LEI identifier.
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
    /// let record: LeiRecord = client.lei_record_by_id("5493000IBP32UQZ0KL24").await?; // strongly typed
    /// let record: serde_json::Value = client.lei_record_by_id("5493000IBP32UQZ0KL24").await?; // raw JSON
    /// ```
    pub async fn lei_record_by_id<R>(&self, lei: &str) -> Result<R>
    where
        R: DeserializeOwned,
    {
        self.request(&format!("lei-records/{lei}")).send().await
    }

    /// Retrieves a list of LEI (Legal Entity Identifier) records, optionally filtered by specific criteria.
    ///
    /// This method sends a request to the `/lei-records` endpoint to fetch LEI records. If no filtering
    /// parameters are provided, it will return all available LEI records. Pagination parameters can be
    /// used to manage large datasets. The response may include URLs to retrieve associated Level 2
    /// (relationship) data, if available, within the `links` section of the response body.
    ///
    /// # Errors
    ///
    /// This method does not itself return errors. However, errors may occur when sending the request or processing
    /// the response using the returned request builder (e.g., network failures or deserialization issues).
    ///
    /// # Examples
    ///
    /// ```rust, ignore
    /// let records: LeiRecordList = client.lei_records().send().await?; // strongly typed
    /// let records: serde_json::Value = client.lei_records().send().await?; // raw JSON
    /// ```
    #[must_use]
    pub fn lei_records(&self) -> GleifRequestBuilder {
        self.request("lei-records")
    }

    /// Retrieves the LEI (Legal Entity Identifier) record for the ultimate parent of a specific entity.
    ///
    /// This method sends a request to the `/lei-records/{lei}/ultimate-parent` endpoint to fetch the LEI record
    /// of the furthest legal entity preparing consolidated financial statements for the given entity, based on
    /// the accounting definition of consolidation applying to that parent. The response may include URLs to
    /// retrieve associated Level 2 (relationship) data, if available, within the `links` section of the response body.
    ///
    /// # Parameters
    ///
    /// - `lei` - A string slice representing the LEI identifier.
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
    /// let record: LeiRecord = client.ultimate_parent("5493000IBP32UQZ0KL24").await?; // strongly typed
    /// let record: serde_json::Value = client.ultimate_parent("5493000IBP32UQZ0KL24").await?; // raw JSON
    /// ```
    pub async fn ultimate_parent<R>(&self, lei: &str) -> Result<R>
    where
        R: DeserializeOwned,
    {
        self.request(&format!("lei-records/{lei}/ultimate-parent"))
            .send()
            .await
    }

    /// Retrieves the LEI (Legal Entity Identifier) record for the direct parent of a specific entity.
    ///
    /// This method sends a request to the `/lei-records/{lei}/direct-parent` endpoint to fetch the LEI record
    /// of the closest legal entity preparing consolidated financial statements for the given entity, based on
    /// the accounting definition of consolidation applying to that parent. The response may include URLs to
    /// retrieve associated Level 2 (relationship) data, if available, within the `links` section of the response body.
    ///
    /// # Parameters
    ///
    /// - `lei` - A string slice representing the LEI identifier.
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
    /// let record: LeiRecord = client.direct_parent("5493000IBP32UQZ0KL24").await?; // strongly typed
    /// let record: serde_json::Value = client.direct_parent("5493000IBP32UQZ0KL24").await?; // raw JSON
    /// ```
    pub async fn direct_parent<R>(&self, lei: &str) -> Result<R>
    where
        R: DeserializeOwned,
    {
        self.request(&format!("lei-records/{lei}/direct-parent"))
            .send()
            .await
    }

    /// Retrieves LEI (Legal Entity Identifier) records for the ultimate children of a specific entity.
    ///
    /// This method sends a request to the `/lei-records/{lei}/ultimate-children` endpoint to fetch LEI records
    /// of the furthest level legal entities wholly consolidated in the consolidated financial statements for
    /// the given entity, based on the accounting definition of consolidation applying to this parent.
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
    /// let records: LeiRecordList = client.ultimate_children("5493000IBP32UQZ0KL24").send().await?; // strongly typed
    /// let records: serde_json::Value = client.ultimate_children("5493000IBP32UQZ0KL24").send().await?; // raw JSON
    /// ```
    #[must_use]
    pub fn ultimate_children(&self, lei: &str) -> GleifRequestBuilder {
        self.request(&format!("lei-records/{lei}/ultimate-children"))
    }

    /// Retrieves LEI (Legal Entity Identifier) records for the direct children of a specific entity.
    ///
    /// This method sends a request to the `/lei-records/{lei}/direct-children` endpoint to fetch LEI records
    /// of the closest level legal entities wholly consolidated in the consolidated financial statements for
    /// the given entity, based on the accounting definition of consolidation applying to this parent.
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
    /// let records: LeiRecordList = client.direct_children("5493000IBP32UQZ0KL24").send().await?; // strongly typed
    /// let records: serde_json::Value = client.direct_children("5493000IBP32UQZ0KL24").send().await?; // raw JSON
    /// ```
    #[must_use]
    pub fn direct_children(&self, lei: &str) -> GleifRequestBuilder {
        self.request(&format!("lei-records/{lei}/direct-children"))
    }

    /// Retrieves the LEI (Legal Entity Identifier) record for the manager of a specific legal entity (fund).
    ///
    /// This method sends a request to the `/lei-records/{lei}/associated-entity` endpoint to fetch the LEI record
    /// of the fund management entity associated with the given LEI. If no fund management entity is reported,
    /// the request will return a resource not found response.
    ///
    /// # Parameters
    ///
    /// - `lei` - A string slice representing the LEI identifier.
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
    /// let record: LeiRecord = client.associated_entity("5493000IBP32UQZ0KL24").await?; // strongly typed
    /// let record: serde_json::Value = client.associated_entity("5493000IBP32UQZ0KL24").await?; // raw JSON
    /// ```
    pub async fn associated_entity<R>(&self, lei: &str) -> Result<R>
    where
        R: DeserializeOwned,
    {
        self.request(&format!("lei-records/{lei}/associated-entity"))
            .send()
            .await
    }

    /// Retrieves the LEI (Legal Entity Identifier) record for the successor entity of a specific legal entity.
    ///
    /// This method sends a request to the `/lei-records/{lei}/successor-entity` endpoint to fetch the LEI record
    /// of the entity that continues or replaces the registration of the given entity. The response may include
    /// URLs to retrieve associated Level 2 (relationship) data, if available, within the `links` section of the response body.
    ///
    /// # Parameters
    ///
    /// - `lei` - A string slice representing the LEI identifier.
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
    /// let record: LeiRecord = client.successor_entity("5493000IBP32UQZ0KL24").await?; // strongly typed
    /// let record: serde_json::Value = client.successor_entity("5493000IBP32UQZ0KL24").await?; // raw JSON
    /// ```
    pub async fn successor_entity<R>(&self, lei: &str) -> Result<R>
    where
        R: DeserializeOwned,
    {
        self.request(&format!("lei-records/{lei}/successor-entity"))
            .send()
            .await
    }

    /// Retrieves the LEI (Legal Entity Identifier) record for the managing Local Operating Unit (LOU) of a specific entity.
    ///
    /// This method sends a request to the `/lei-records/{lei}/managing-lou` endpoint to fetch the LEI record
    /// of the LEI Issuer responsible for administering the given LEI registration. The response may include
    /// URLs to retrieve associated Level 2 (relationship) data, if available, within the `links` section of the response body.
    ///
    /// # Parameters
    ///
    /// - `lei` - A string slice representing the LEI identifier.
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
    /// let record: LeiRecord = client.managing_lou("5493000IBP32UQZ0KL24").await?; // strongly typed
    /// let record: serde_json::Value = client.managing_lou("5493000IBP32UQZ0KL24").await?; // raw JSON
    /// ```
    pub async fn managing_lou<R>(&self, lei: &str) -> Result<R>
    where
        R: DeserializeOwned,
    {
        self.request(&format!("lei-records/{lei}/managing-lou"))
            .send()
            .await
    }
}
