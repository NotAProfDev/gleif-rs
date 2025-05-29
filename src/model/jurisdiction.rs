//! Model definitions for the GLEIF jurisdiction endpoint.
//!
//! This module contains the data structures used to deserialize responses from the `/jurisdictions` endpoint of the GLEIF API.
//! It provides Rust models for the jurisdiction resource, matching the JSON structure returned by the API.
//!
//! For endpoint usage and client methods, see [`crate::endpoint::jurisdiction`] (`src/endpoint/jurisdiction.rs`).

use serde::{Deserialize, Serialize};

/// Represents a jurisdiction as returned by the GLEIF API.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Jurisdiction {
    /// The type of the data (e.g., "jurisdictions").
    #[serde(rename = "type")]
    pub data_type: String,
    /// The unique identifier of the jurisdiction.
    pub id: String,
    /// The attributes of the jurisdiction.
    pub attributes: JurisdictionAttributes,
}

/// Attributes of a jurisdiction as returned by the GLEIF API.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JurisdictionAttributes {
    /// The code of the jurisdiction.
    pub code: String,
    /// The name of the jurisdiction, if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{model::common::GleifApiResponse, test_utils::test_model_files};
    use serde_json;
    use std::path::Path;

    #[test]
    fn test_deserialize_single_jurisdiction() {
        let dir = Path::new("tests/data/jurisdictions");
        test_model_files(
            |filename| {
                filename.starts_with("jurisdiction_")
                    && Path::new(filename)
                        .extension()
                        .is_some_and(|ext| ext.eq_ignore_ascii_case("json"))
            },
            |data| serde_json::from_str::<GleifApiResponse<Jurisdiction>>(data),
            |filename, jurisdiction| {
                assert!(
                    !jurisdiction.data.id.is_empty(),
                    "Jurisdiction id should not be empty in {filename}"
                );
                assert!(
                    !jurisdiction.data.attributes.code.is_empty(),
                    "Jurisdiction code should not be empty in {filename}"
                );
            },
            dir,
        );
    }

    #[test]
    fn test_deserialize_jurisdictions_list() {
        let dir = Path::new("tests/data/jurisdictions");
        test_model_files(
            |filename| filename == "jurisdictions_all.json",
            |data| serde_json::from_str::<GleifApiResponse<Vec<Jurisdiction>>>(data),
            |filename, jurisdictions| {
                assert!(
                    !jurisdictions.data.is_empty(),
                    "Jurisdictions list should not be empty in {filename}"
                );
                for jurisdiction in &jurisdictions.data {
                    assert!(
                        !jurisdiction.id.is_empty(),
                        "Jurisdiction id should not be empty in {filename}"
                    );
                }
            },
            dir,
        );
    }
}
