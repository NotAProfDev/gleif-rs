//! Model definitions for the GLEIF registration agent endpoint.
//!
//! This module contains the data structures used to deserialize responses from the `/registration-agents` endpoint of the GLEIF API.
//! It provides Rust models for the registration agent resource, matching the JSON structure returned by the API.
//!
//! For endpoint usage and client methods, see [`crate::endpoint::registration_agent`] (`src/endpoint/registration_agent.rs`).

use serde::Deserialize;

/// Represents a registration agent as returned by the GLEIF API.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct RegistrationAgent {
    /// The type of the data (e.g., "registration-agents").
    #[serde(rename = "type")]
    pub data_type: String,
    /// The unique identifier of the registration agent.
    pub id: String,
    /// The attributes of the registration agent.
    pub attributes: RegistrationAgentAttributes,
}

/// Attributes of a registration agent as returned by the GLEIF API.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RegistrationAgentAttributes {
    /// The name of the registration agent.
    pub name: String,
    /// The LEI code of the registration agent.
    pub lei: Option<String>,
    ///The lei issuer of the registration agent.
    pub lei_issuer: String,
    /// The website of the registration agent.
    pub websites: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{model::common::GleifApiResponse, test_utils::test_model_files};
    use serde_json;
    use std::path::Path;

    #[test]
    fn test_deserialize_single_registration_agent() {
        let dir = Path::new("tests/data/registration_agents");
        test_model_files(
            |filename| filename.starts_with("registration_agent_") && filename.ends_with(".json"),
            |data| serde_json::from_str::<GleifApiResponse<RegistrationAgent>>(data),
            |filename, agent| {
                assert!(
                    !agent.data.id.is_empty(),
                    "RegistrationAgent id should not be empty in {filename}"
                );
                assert!(
                    !agent.data.attributes.name.is_empty(),
                    "RegistrationAgent name should not be empty in {filename}"
                );
            },
            dir,
        );
    }

    #[test]
    fn test_deserialize_registration_agents_list() {
        let dir = Path::new("tests/data/registration_agents");
        test_model_files(
            |filename| filename == "registration_agents_all.json",
            |data| serde_json::from_str::<GleifApiResponse<Vec<RegistrationAgent>>>(data),
            |filename, agents| {
                assert!(
                    !agents.data.is_empty(),
                    "RegistrationAgents list should not be empty in {filename}"
                );
                for agent in agents.data.iter() {
                    assert!(
                        !agent.id.is_empty(),
                        "RegistrationAgent id should not be empty in {filename}"
                    );
                }
            },
            dir,
        );
    }
}
