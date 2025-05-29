//! Model definitions for the GLEIF ISIN endpoint.
//!
//! This module contains the data structures used to deserialize responses from the `/lei-records/:id/isins` endpoint
//! of the GLEIF API. It provides Rust models for the ISIN resource, matching the JSON structure returned by the API.
//!
//! For endpoint usage and client methods, see [`crate::endpoint::isin`] (`src/endpoint/isin.rs`).

use serde::{Deserialize, Serialize};

/// ISIN resource object for GLEIF API.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Isin {
    /// The type of the data (e.g., "isins").
    pub r#type: String,
    /// The unique identifier of the ISIN resource.
    pub id: String,
    /// ISIN attributes.
    pub attributes: IsinAttributes,
}

/// Attributes for an ISIN resource as returned by the GLEIF API.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IsinAttributes {
    /// A Legal Entity Identifier (LEI) code, in the format specified by ISO 17442.
    pub lei: String,
    /// The ISIN code.
    pub isin: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{model::common::GleifApiResponse, test_utils::test_model_files};
    use serde_json;
    use std::path::Path;

    #[test]
    fn test_deserialize_isins_list() {
        let dir = Path::new("tests/data/isins");
        test_model_files(
            |filename| {
                filename.starts_with("isins_")
                    && Path::new(filename)
                        .extension()
                        .is_some_and(|ext| ext.eq_ignore_ascii_case("json"))
            },
            |data| serde_json::from_str::<GleifApiResponse<Vec<Isin>>>(data),
            |filename, isins| {
                // An empty isin list can be valid
                if isins.data.is_empty() {
                    return;
                }
                assert!(
                    !isins.data.is_empty(),
                    "ISINs list should not be empty in {filename}"
                );
                for isin in &isins.data {
                    assert!(
                        !isin.id.is_empty(),
                        "ISIN id should not be empty in {filename}"
                    );
                }
            },
            dir,
        );
    }
}
