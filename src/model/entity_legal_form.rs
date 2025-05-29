//! Model definitions for the GLEIF entity legal forms endpoint.
//!
//! This module contains the data structures used to deserialize responses from the `/entity-legal-forms` endpoint of the GLEIF API.
//! It provides Rust models for the entity legal form resource, matching the JSON structure returned by the API.
//!
//! For endpoint usage and client methods, see [`crate::endpoint::entity_legal_form`] (`src/endpoint/entity_legal_form.rs`).

use crate::model::enums::ELFStatus;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// A single entity legal form record as returned by the GLEIF API.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntityLegalForm {
    /// The type of the data (always "entityLegalForms").
    pub r#type: String,
    /// The unique identifier of the entity legal form.
    pub id: String,
    /// The attributes of the entity legal form.
    pub attributes: EntityLegalFormAttributes,
}

/// Attributes of an entity legal form as returned by the GLEIF API.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityLegalFormAttributes {
    /// The code of the entity legal form.
    pub code: String,
    /// The country associated with the entity legal form.
    pub country: String,
    /// The jurisdiction of the entity legal form.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<String>,
    /// The country code (ISO) of the entity legal form.
    pub country_code: String,
    /// The subdivision code of the entity legal form.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdivision_code: Option<String>,
    /// The creation date of the entity legal form record.
    pub date_created: NaiveDate,
    /// The status of the entity legal form.
    pub status: ELFStatus,
    /// The list of names for the entity legal form.
    pub names: Vec<EntityLegalFormName>,
}

/// A localized or transliterated name for an entity legal form.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityLegalFormName {
    /// The local name of the entity legal form.
    pub local_name: String,
    /// The language of the name.
    pub language: String,
    /// The language code (ISO) of the name.
    pub language_code: String,
    /// The transliterated name of the entity legal form.
    pub transliterated_name: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{model::common::GleifApiResponse, test_utils::test_model_files};
    use serde_json;
    use std::path::Path;

    #[test]
    fn test_deserialize_single_entity_legal_form() {
        let dir = Path::new("tests/data/entity_legal_forms");
        test_model_files(
            |filename| {
                filename.starts_with("entity_legal_form_")
                    && Path::new(filename)
                        .extension()
                        .is_some_and(|ext| ext.eq_ignore_ascii_case("json"))
            },
            |data| serde_json::from_str::<GleifApiResponse<EntityLegalForm>>(data),
            |filename, elf| {
                assert!(
                    !elf.data.id.is_empty(),
                    "EntityLegalForm id should not be empty in {filename}"
                );
                assert!(
                    !elf.data.attributes.code.is_empty(),
                    "EntityLegalForm code should not be empty in {filename}"
                );
                assert!(
                    !elf.data.attributes.names.is_empty(),
                    "EntityLegalForm should have at least one name in {filename}"
                );
            },
            dir,
        );
    }

    #[test]
    fn test_deserialize_multi_entity_legal_forms() {
        let dir = Path::new("tests/data/entity_legal_forms");
        test_model_files(
            |filename| filename == "entity_legal_forms_all.json",
            |data| serde_json::from_str::<GleifApiResponse<Vec<EntityLegalForm>>>(data),
            |filename, list| {
                if list.data.is_empty() {
                    return;
                }
                assert!(
                    !list.data[0].id.is_empty(),
                    "EntityLegalForm id should not be empty in first record of {filename}"
                );
                assert!(
                    !list.data[0].attributes.code.is_empty(),
                    "EntityLegalForm code should not be empty in first record of {filename}"
                );
            },
            dir,
        );
    }
}
