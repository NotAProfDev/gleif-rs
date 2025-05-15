//! This module provides functionality for interacting with the relationship endpoints of the GLEIF API.
//!
//! Relationships in the GLEIF API represent hierarchical connections between legal entities, such as
//! parent-child relationships based on accounting consolidation standards. These relationships are
//! crucial for understanding the structure and ownership of legal entities.
//!
//! The module includes methods to:
//! - Retrieve the direct parent relationship for a specific LEI.
//! - Retrieve the ultimate parent relationship for a specific LEI.
//! - Retrieve the direct child relationships for a specific LEI.
//! - Retrieve the ultimate child relationships for a specific LEI.
//!
//! # Endpoints
//!
//! - `/lei-records/{lei}/direct-parent-relationship`: Fetches the direct parent relationship.
//! - `/lei-records/{lei}/ultimate-parent-relationship`: Fetches the ultimate parent relationship.
//! - `/lei-records/{lei}/direct-child-relationships`: Fetches the direct child relationships.
//! - `/lei-records/{lei}/ultimate-child-relationships`: Fetches the ultimate child relationships.
//!
//! # Errors
//!
//! Errors may occur during network communication, server-side issues, or deserialization of the response.
//! These errors are returned as [`GleifError`] instances.
//!
//! # Examples
//!
//! ```rust
//! use gleif_rs::{client::GleifClient};
//! use gleif_rs::model::{RelationshipRecord, RelationshipRecordList};
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = GleifClient::new();
//!
//! // Fetch the direct parent relationship for a specific LEI
//! let relationship: RelationshipRecord = client.direct_parent_relationship("5493000IBP32UQZ0KL24").await?; // strongly typed
//! let relationship: serde_json::Value = client.direct_parent_relationship("5493000IBP32UQZ0KL24").await?; // raw JSON
//!
//! // Fetch the ultimate parent relationship for a specific LEI
//! let relationship: RelationshipRecord = client.ultimate_parent_relationship("5493000IBP32UQZ0KL24").await?;
//!
//! // Fetch the direct child relationships for a specific LEI
//! let relationships: RelationshipRecordList = client.direct_child_relationships("5493000IBP32UQZ0KL24").send().await?;
//!
//! // Fetch the ultimate child relationships for a specific LEI
//! let relationships: RelationshipRecordList = client.ultimate_child_relationships("5493000IBP32UQZ0KL24").send().await?;
//! # Ok(()) }
//! ```

use crate::{client::GleifClient, error::Result, request_builder::GleifRequestBuilder};
use serde::de::DeserializeOwned;

impl GleifClient {
    /// Retrieves the direct parent relationship for a specific LEI (Legal Entity Identifier).
    ///
    /// This method sends a request to the `/lei-records/{lei}/direct-parent-relationship` endpoint
    /// to fetch the relationship record of the closest legal entity preparing consolidated financial
    /// statements for the given "child" entity, based on the accounting definition of consolidation.
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
    /// let relationship: RelationshipRecord = client.direct_parent_relationship("5493000IBP32UQZ0KL24").await?; // strongly typed
    /// let relationship: serde_json::Value = client.direct_parent_relationship("5493000IBP32UQZ0KL24").await?; // raw JSON
    /// ```
    pub async fn direct_parent_relationship<R>(&self, lei: &str) -> Result<R>
    where
        R: DeserializeOwned,
    {
        self.request(&format!("lei-records/{lei}/direct-parent-relationship"))
            .send()
            .await
    }

    /// Retrieves the ultimate parent relationship for a specific LEI (Legal Entity Identifier).
    ///
    /// This method sends a request to the `/lei-records/{lei}/ultimate-parent-relationship` endpoint
    /// to fetch the relationship record of the furthest legal entity preparing consolidated financial
    /// statements for the given "child" entity, based on the accounting definition of consolidation.
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
    /// let relationship: RelationshipRecord = client.ultimate_parent_relationship("5493000IBP32UQZ0KL24").await?; // strongly typed
    /// let relationship: serde_json::Value = client.ultimate_parent_relationship("5493000IBP32UQZ0KL24").await?; // raw JSON
    /// ```
    pub async fn ultimate_parent_relationship<R>(&self, lei: &str) -> Result<R>
    where
        R: DeserializeOwned,
    {
        self.request(&format!("lei-records/{lei}/ultimate-parent-relationship"))
            .send()
            .await
    }

    /// Retrieves the ultimate child relationships for a specific LEI (Legal Entity Identifier).
    ///
    /// This method sends a request to the `/lei-records/{lei}/ultimate-child-relationships` endpoint
    /// to fetch relationship records indicating the furthest level legal entities wholly consolidated
    /// in the consolidated financial statements for the given entity, based on the accounting definition
    /// of consolidation applying to this parent.
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
    /// let relationships: RelationshipRecordList = client.ultimate_child_relationships("5493000IBP32UQZ0KL24").send().await?; // strongly typed
    /// let relationships: serde_json::Value = client.ultimate_child_relationships("5493000IBP32UQZ0KL24").send().await?; // raw JSON
    /// ```
    #[must_use]
    pub fn ultimate_child_relationships(&self, lei: &str) -> GleifRequestBuilder {
        self.request(&format!("lei-records/{lei}/ultimate-child-relationships"))
    }

    /// Retrieves the direct child relationships for a specific LEI (Legal Entity Identifier).
    ///
    /// This method sends a request to the `/lei-records/{lei}/direct-child-relationships` endpoint
    /// to fetch relationship records indicating the closest level legal entities wholly consolidated
    /// in the consolidated financial statements for the given entity, based on the accounting definition
    /// of consolidation applying to this parent.
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
    /// let relationships: RelationshipRecordList = client.direct_child_relationships("5493000IBP32UQZ0KL24").send().await?; // strongly typed
    /// let relationships: serde_json::Value = client.direct_child_relationships("5493000IBP32UQZ0KL24").send().await?; // raw JSON
    /// ```
    #[must_use]
    pub fn direct_child_relationships(&self, lei: &str) -> GleifRequestBuilder {
        self.request(&format!("lei-records/{lei}/direct-child-relationships"))
    }
}
