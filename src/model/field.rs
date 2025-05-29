//! Model definitions for the GLEIF field endpoint.
//!
//! This module contains the data structures used to deserialize responses from the `/fields` endpoint of the GLEIF API.
//! It provides Rust models for the field resource, matching the JSON structure returned by the API.
//!
//! For endpoint usage and client methods, see [`crate::endpoint::field`] (`src/endpoint/field.rs`).

use serde::{Deserialize, Serialize};

/// Represents a field definition as returned by the GLEIF API.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Field {
    /// The type of the data (e.g., "fields").
    pub r#type: String,
    /// The unique identifier of the field.
    pub id: String,
    /// The attributes of the field.
    pub attributes: FieldAttributes,
}

/// Attributes of a field as returned by the GLEIF API.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FieldAttributes {
    /// The field name.
    pub field: String,
    /// The display label for the field.
    pub label: String,
    /// The data type of the field.
    pub data_type: String,
    /// The possible enum values for the field, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_values: Option<Vec<String>>,
    /// The resource associated with the field, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    /// Whether the field is sortable.
    pub sortable: bool,
    /// The supported operators for the field, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operators: Option<Vec<String>>,
    /// The contexts in which the field is used.
    pub contexts: Vec<String>,
    /// The `JSONPath` for the field, if any.
    #[serde(rename = "jsonPath", skip_serializing_if = "Option::is_none")]
    pub json_path: Option<String>,
    /// The `XPath` for the field, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xpath: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{model::common::GleifApiResponse, test_utils::test_model_files};
    use serde_json;
    use std::path::Path;

    #[test]
    fn test_deserialize_single_field() {
        let dir = Path::new("tests/data/fields");
        test_model_files(
            |filename| {
                filename.starts_with("field_")
                    && Path::new(filename)
                        .extension()
                        .is_some_and(|ext| ext.eq_ignore_ascii_case("json"))
            },
            |data| serde_json::from_str::<GleifApiResponse<Field>>(data),
            |filename, field| {
                assert!(
                    !field.data.id.is_empty(),
                    "Field id should not be empty in {filename}"
                );
            },
            dir,
        );
    }

    #[test]
    fn test_deserialize_fields_list() {
        let dir = Path::new("tests/data/fields");
        test_model_files(
            |filename| filename == "fields_all.json",
            |data| serde_json::from_str::<GleifApiResponse<Vec<Field>>>(data),
            |filename, fields| {
                assert!(
                    !fields.data.is_empty(),
                    "Fields list should not be empty in {filename}"
                );
                for field in &fields.data {
                    assert!(
                        !field.id.is_empty(),
                        "Field id should not be empty in {filename}"
                    );
                }
            },
            dir,
        );
    }
}
