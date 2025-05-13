//! Model definitions for the GLEIF registration authority endpoint.
//!
//! This module contains the data structures used to deserialize responses from the `/registration-authorities` endpoint of the GLEIF API.
//! It provides Rust models for the registration authority resource, matching the JSON structure returned by the API.
//!
//! For endpoint usage and client methods, see [`crate::endpoint::registration_authority`] (`src/endpoint/registration_authority.rs`).

use serde::Deserialize;

/// Represents a registration authority as returned by the GLEIF API.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct RegistrationAuthority {
    /// The type of the data (e.g., "registration-authorities").
    #[serde(rename = "type")]
    pub data_type: String,
    /// The unique identifier of the registration authority.
    pub id: String,
    /// The attributes of the registration authority.
    pub attributes: RegistrationAuthorityAttributes,
}

/// Attributes of a registration authority as returned by the GLEIF API.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RegistrationAuthorityAttributes {
    /// The code of the registration authority.
    pub code: String,
    /// The international name of the registration authority, if available.
    pub international_name: Option<String>,
    /// The local name of the registration authority, if available.
    pub local_name: Option<String>,
    /// The international organization name, if available.
    pub international_organization_name: Option<String>,
    /// The local organization name, if available.
    pub local_organization_name: Option<String>,
    /// The website of the registration authority.
    pub website: String,
    /// The jurisdictions associated with the registration authority.
    pub jurisdictions: Vec<RegistrationAuthorityJurisdiction>,
}

/// Represents a registration authority jurisdiction as returned by the GLEIF API.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RegistrationAuthorityJurisdiction {
    /// The country associated with the registration authority.
    pub country: String,
    /// The country code of the registration authority.
    pub country_code: String,
    /// The jurisdiction associated with the registration authority.
    pub jurisdiction: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{model::common::GleifApiResponse, test_utils::test_model_files};
    use serde_json;
    use std::path::Path;

    #[test]
    fn test_deserialize_single_registration_authority() {
        let dir = Path::new("tests/data/registration_authorities");
        test_model_files(
            |filename| {
                filename.starts_with("registration_authority_") && Path::new(filename)
                    .extension()
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("json"))
            },
            |data| serde_json::from_str::<GleifApiResponse<RegistrationAuthority>>(data),
            |filename, authority| {
                assert!(
                    !authority.data.id.is_empty(),
                    "RegistrationAuthority id should not be empty in {filename}"
                );
                assert!(
                    !authority.data.attributes.code.is_empty(),
                    "RegistrationAuthority code should not be empty in {filename}"
                );
            },
            dir,
        );
    }

    #[test]
    fn test_deserialize_registration_authorities_list() {
        let dir = Path::new("tests/data/registration_authorities");
        test_model_files(
            |filename| filename == "registration_authorities_all.json",
            |data| serde_json::from_str::<GleifApiResponse<Vec<RegistrationAuthority>>>(data),
            |filename, authorities| {
                assert!(
                    !authorities.data.is_empty(),
                    "RegistrationAuthorities list should not be empty in {filename}"
                );
                for authority in &authorities.data {
                    assert!(
                        !authority.id.is_empty(),
                        "RegistrationAuthority id should not be empty in {filename}"
                    );
                }
            },
            dir,
        );
    }
}
