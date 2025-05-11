//! Endpoints for GLEIF API registration agent metadata.
//! This module provides functionality for interacting with the Registration Agents endpoint of the GLEIF API.
//!
//! Registration Agents are entities that assist legal entities in obtaining and managing Legal Entity Identifiers (LEIs).
//! They act as intermediaries between legal entities and LEI Issuers (Local Operating Units or LOUs).
//!
//! The module includes methods to:
//! - Fetch a list of all Registration Agents, optionally filtered by specific criteria.
//! - Fetch detailed information about a specific Registration Agent using its unique identifier.
//!
//! # Endpoints
//!
//! - `/registration-agents`: Fetches a list of all Registration Agents.
//! - `/registration-agents/{id}`: Fetches detailed information about a specific Registration Agent by its unique ID.
//!
//! # Examples
//!
//! ```rust
//! use gleif_rs::{client::GleifClient};
//! use gleif_rs::model::registration_agent::RegistrationAgent;
//! use gleif_rs::model::common::GleifApiResponse;
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = GleifClient::new();
//!
//! // Fetch all Registration Agents
//! let agents: GleifApiResponse<Vec<RegistrationAgent>> = client.registration_agents().send().await?; // strongly typed
//! let agents: serde_json::Value = client.registration_agents().send().await?; // raw JSON
//!
//! // Fetch a specific Registration Agent by ID
//! let agent: GleifApiResponse<RegistrationAgent> = client.registration_agent_by_id("5d10d4dc9f3764.95022907").await?;
//! # Ok(()) }
//! ```
//!
//! # Errors
//!
//! Errors may occur during network communication, server-side issues, or deserialization of the response.
//! These errors are returned as [`GleifError`] instances.

use crate::{client::GleifClient, error::Result, request_builder::GleifRequestBuilder};
use serde::de::DeserializeOwned;

impl GleifClient {
    /// Fetches a list of all Registration Agents (`/registration-agents`).
    ///
    /// This method sends a request to retrieve all Registration Agents that have consented to have
    /// their information published. The response can be filtered using the following parameters:
    /// - **`leiIssuer`**: The LEI of the associated LEI Issuer.
    /// - **`lei`**: The LEI of the Registration Agent itself.
    ///
    /// # Errors
    ///
    /// This method does not itself return errors. However, errors may occur when sending the request or processing
    /// the response using the returned request builder (e.g., network failures or deserialization issues).
    ///
    /// # Examples
    ///
    /// ```rust, ignore
    /// let agents: GleifApiResponse<Vec<RegistrationAgent>> = client.registration_agents().send().await?; // strongly typed
    /// let agents: serde_json::Value = client.registration_agents().send().await?; // raw JSON
    /// ```
    #[must_use]
    pub fn registration_agents(&self) -> GleifRequestBuilder {
        self.request("registration-agents")
    }

    /// Fetches details of a single Registration Agent by its unique ID (`/registration-agents/{id}`).
    ///
    /// This method sends a request to retrieve information about a specific registration agent by its ID.
    ///
    /// # Parameters
    ///
    /// * `id` - A string slice representing the unique identifier of the registration agent.
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
    /// let agent: GleifApiResponse<RegistrationAgent> = client.registration_agent_by_id("5d10d4dc9f3764.95022907").await?; // strongly typed
    /// let agent: serde_json::Value = client.registration_agent_by_id("5d10d4dc9f3764.95022907").await?; // raw JSON
    /// ```
    pub async fn registration_agent_by_id<R>(&self, id: &str) -> Result<R>
    where
        R: DeserializeOwned,
    {
        self.request(&format!("registration-agents/{id}"))
            .send()
            .await
    }
}
