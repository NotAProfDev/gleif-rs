//! Endpoints for GLEIF API entity legal forms metadata.
//!
//! The Entity Legal Forms endpoint contains ISO 20275 ELF Codes, which are used to standardize
//! the identification of legal forms of entities across jurisdictions. This module provides
//! functionality for interacting with the Entity Legal Forms endpoint of the GLEIF API.
//!
//! The module includes methods to:
//! - Fetch a list of all entity legal forms, optionally filtered by specific criteria.
//! - Fetch detailed information about a specific entity legal form using its ELF code.
//!
//! # Endpoints
//!
//! - `/entity-legal-forms`: Fetches a list of all entity legal forms.
//! - `/entity-legal-forms/{id}`: Fetches detailed information about a specific entity legal form by its ELF code.
//!
//! # Examples
//!
//! ```rust
//! use gleif_rs::{client::GleifClient};
//! use gleif_rs::model::{EntityLegalForm, EntityLegalFormList};
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = GleifClient::new();
//!
//! // Fetch all entity legal forms
//! let legal_forms: EntityLegalFormList = client.entity_legal_forms().send().await?; // strongly typed
//! let legal_forms: serde_json::Value = client.entity_legal_forms().send().await?; // raw JSON
//!
//! // Fetch a specific entity legal form by ELF code
//! let legal_form: EntityLegalForm = client.entity_legal_form_by_id("10UR").await?;
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
    /// Fetches a list of all entity legal forms (`/entity-legal-forms`).
    ///
    /// This method sends a request to retrieve all available entity legal forms.
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
    /// let legal_forms: EntityLegalFormList = client.entity_legal_forms().send().await?; // strongly typed
    /// let legal_forms: serde_json::Value = client.entity_legal_forms().send().await?; // raw JSON
    /// ```
    #[must_use]
    pub fn entity_legal_forms(&self) -> GleifRequestBuilder {
        self.request("entity-legal-forms")
    }

    /// Fetches details of a single entity legal form by ELF code (`/entity-legal-forms/{id}`).
    ///
    /// This method sends a request to retrieve information about a specific entity legal form by its ELF code.
    ///
    /// # Parameters
    ///
    /// * `id` - A string slice representing the unique ELF code of the entity legal form.
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
    /// let legal_form: EntityLegalForm = client.entity_legal_form_by_id("10UR").await?; // strongly typed
    /// let legal_form: serde_json::Value = client.entity_legal_form_by_id("10UR").await?; // raw JSON
    /// ```
    pub async fn entity_legal_form_by_id<R>(&self, id: &str) -> Result<R>
    where
        R: DeserializeOwned,
    {
        self.request(&format!("entity-legal-forms/{id}"))
            .send()
            .await
    }
}
