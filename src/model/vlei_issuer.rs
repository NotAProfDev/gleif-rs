//! Model definitions for the GLEIF vLEI Issuer endpoint.
//!
//! This module contains the data structures used to deserialize responses from the `/vlei-issuers` endpoint of the GLEIF API.
//! It provides Rust models for the vLEI Issuer resource, matching the JSON structure returned by the API.
//!
//! For endpoint usage and client methods, see [`crate::endpoint::vlei_issuer`] (`src/endpoint/vlei_issuer.rs`).

use chrono::{DateTime, Utc};
use serde::Deserialize;

/// Represents a vLEI Issuer as returned by the GLEIF API.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct VLeiIssuer {
    /// The type of the data (e.g., "vlei-issuers").
    #[serde(rename = "type")]
    pub data_type: String,
    /// The unique identifier of the vLEI Issuer.
    pub id: String,
    /// The attributes of the vLEI Issuer.
    pub attributes: VLeiIssuerAttributes,
}

/// Attributes of a vLEI Issuer as returned by the GLEIF API.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct VLeiIssuerAttributes {
    /// The LEI code of the issuer.
    pub lei: String,
    /// The name of the vLEI Issuer.
    pub name: String,
    /// The marketing name of the vLEI Issuer.
    pub marketing_name: String,
    /// The website of the vLEI Issuer.
    pub website: String,
    /// The qualification date of the vLEI Issuer.
    pub qualification_date: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{model::common::GleifApiResponse, test_utils::test_model_files};
    use serde_json;
    use std::path::Path;

    #[test]
    fn test_deserialize_single_vlei_issuer() {
        let dir = Path::new("tests/data/vlei_issuers");
        test_model_files(
            |filename| {
                filename.starts_with("vlei_issuer_")
                    && Path::new(filename)
                        .extension()
                        .is_some_and(|ext| ext.eq_ignore_ascii_case("json"))
            },
            |data| serde_json::from_str::<GleifApiResponse<VLeiIssuer>>(data),
            |filename, issuer| {
                assert!(
                    !issuer.data.id.is_empty(),
                    "VLeiIssuer id should not be empty in {filename}"
                );
                assert!(
                    !issuer.data.attributes.name.is_empty(),
                    "VLeiIssuer name should not be empty in {filename}"
                );
            },
            dir,
        );
    }

    #[test]
    fn test_deserialize_vlei_issuers_list() {
        let dir = Path::new("tests/data/vlei_issuers");
        test_model_files(
            |filename| filename == "vlei_issuers_all.json",
            |data| serde_json::from_str::<GleifApiResponse<Vec<VLeiIssuer>>>(data),
            |filename, issuers| {
                assert!(
                    !issuers.data.is_empty(),
                    "VLeiIssuer list should not be empty in {filename}"
                );
                for issuer in &issuers.data {
                    assert!(
                        !issuer.id.is_empty(),
                        "VLeiIssuer id should not be empty in {filename}"
                    );
                }
            },
            dir,
        );
    }
}
