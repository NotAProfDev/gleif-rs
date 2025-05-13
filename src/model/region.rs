//! Model definitions for the GLEIF region endpoint.
//!
//! This module contains the data structures used to deserialize responses from the `/regions` endpoint of the GLEIF API.
//! It provides Rust models for the region resource, matching the JSON structure returned by the API.
//!
//! For endpoint usage and client methods, see [`crate::endpoint::region`] (`src/endpoint/region.rs`).

use serde::Deserialize;

/// Represents a region as returned by the GLEIF API.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Region {
    /// The type of the data (e.g., "regions").
    #[serde(rename = "type")]
    pub data_type: String,
    /// The unique identifier of the region.
    pub id: String,
    /// The attributes of the region.
    pub attributes: RegionAttributes,
}

/// Attributes of a region as returned by the GLEIF API.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct RegionAttributes {
    /// The code of the region.
    pub code: String,
    /// The name of the region, if available.
    pub name: Option<String>,
    /// A list of names associated with the region in different languages.
    pub names: Vec<RegionName>,
}

/// A name associated with the region in a specific language.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct RegionName {
    /// The name in the language.
    pub name: String,
    /// The language code (ISO) of the name.
    pub language: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{model::common::GleifApiResponse, test_utils::test_model_files};
    use serde_json;
    use std::path::Path;

    #[test]
    fn test_deserialize_single_region() {
        let dir = Path::new("tests/data/regions");
        test_model_files(
            |filename| filename.starts_with("region_") && filename.ends_with(".json"),
            |data| serde_json::from_str::<GleifApiResponse<Region>>(data),
            |filename, region| {
                assert!(
                    !region.data.id.is_empty(),
                    "Region id should not be empty in {filename}"
                );
                assert!(
                    !region.data.attributes.code.is_empty(),
                    "Region code should not be empty in {filename}"
                );
                assert!(
                    !region.data.attributes.names.is_empty(),
                    "Region should have at least one name in {filename}"
                );
            },
            dir,
        );
    }

    #[test]
    fn test_deserialize_regions_list() {
        let dir = Path::new("tests/data/regions");
        test_model_files(
            |filename| filename == "regions_all.json",
            |data| serde_json::from_str::<GleifApiResponse<Vec<Region>>>(data),
            |filename, regions| {
                assert!(
                    !regions.data.is_empty(),
                    "Regions list should not be empty in {filename}"
                );
                for region in regions.data.iter() {
                    assert!(
                        !region.id.is_empty(),
                        "Region id should not be empty in {filename}"
                    );
                }
            },
            dir,
        );
    }
}
