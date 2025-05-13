//! Model definitions for the GLEIF field modification endpoint.
//!
//! This module contains the data structures used to deserialize responses from the `/field-modifications` endpoint of the GLEIF API.
//! It provides Rust models for the field modification resource, matching the JSON structure returned by the API.
//!
//! For endpoint usage and client methods, see [`crate::endpoint::field_modification`] (`src/endpoint/field_modification.rs`).

use chrono::{DateTime, Utc};
use serde::Deserialize;

/// Represents a field modification record as returned by the GLEIF API.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct FieldModification {
    /// The type of the data (e.g., "fieldModifications").
    #[serde(rename = "type")]
    pub data_type: String,
    /// The unique identifier of the field modification.
    pub id: String,
    /// The attributes of the field modification.
    pub attributes: FieldModificationAttributes,
}

/// Attributes of a field modification as returned by the GLEIF API.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FieldModificationAttributes {
    /// The LEI associated with the modification.
    pub lei: String,
    /// The type of record being modified.
    pub record_type: String,
    /// The type of modification performed.
    pub modification_type: String,
    /// The field that was modified.
    pub field: String,
    /// The date and time of the modification.
    pub date: DateTime<Utc>,
    /// The old value of the field, if available.
    pub value_old: Option<String>,
    /// The new value of the field.
    pub value_new: String,
    /// Additional context for the modification, if available.
    pub context: Option<FieldModificationContext>,
}

/// Context information for a field modification, if available.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FieldModificationContext {
    /// The type of relationship, if applicable.
    pub relationship_type: Option<String>,
    /// The end node of the relationship, if applicable.
    pub end_node: Option<String>,
    /// The exception category, if applicable.
    pub exception_category: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{model::common::GleifApiResponse, test_utils::test_model_files};
    use serde_json;
    use std::path::Path;

    #[test]
    fn test_deserialize_multi_field_modifications() {
        let dir = Path::new("tests/data/field_modifications");
        test_model_files(
            |filename| filename.starts_with("field_modifications_") && Path::new(filename)
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("json")),
            |data| serde_json::from_str::<GleifApiResponse<Vec<FieldModification>>>(data),
            |filename, list| {
                if list.data.is_empty() {
                    return;
                }
                assert!(
                    !list.data[0].id.is_empty(),
                    "FieldModification id should not be empty in first record of {filename}"
                );
                assert!(
                    !list.data[0].attributes.lei.is_empty(),
                    "FieldModification LEI should not be empty in first record of {filename}"
                );
            },
            dir,
        );
    }
}
