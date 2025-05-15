//! Model definitions for the GLEIF official organizational role endpoint.
//!
//! This module contains the data structures used to deserialize responses from the `/official-organizational-roles` endpoint of the GLEIF API.
//! It provides Rust models for the official organizational role resource, matching the JSON structure returned by the API.
//!
//! For endpoint usage and client methods, see [`crate::endpoint::official_organizational_role`] (`src/endpoint/official_organizational_role.rs`).

use crate::model::enums::ELFStatus;
use chrono::NaiveDate;
use serde::Deserialize;

/// Represents an official organizational role as returned by the GLEIF API.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct OfficialOrganizationalRole {
    /// The type of the data (e.g., "officialOrganizationalRoles").
    #[serde(rename = "type")]
    pub data_type: String,
    /// The unique identifier of the official organizational role.
    pub id: String,
    /// The attributes of the official organizational role.
    pub attributes: OfficialOrganizationalRoleAttributes,
}

/// Attributes of an official organizational role as returned by the GLEIF API.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OfficialOrganizationalRoleAttributes {
    /// The code of the official organizational role.
    pub code: String,
    /// The country associated with the official organizational role.
    pub country: String,
    /// The jurisdiction associated with the official organizational role, if available.
    pub jurisdiction: Option<String>,
    /// The country code of the official organizational role.
    pub country_code: String,
    /// The subdivision code of the official organizational role, if available.
    pub subdivision_code: Option<String>,
    /// The date when the official organizational role was created.
    pub date_created: NaiveDate,
    /// The status of the official organizational role (e.g., "ACTV").
    pub status: ELFStatus,
    /// The ELF code associated with the official organizational role.
    pub elf_code: String,
    /// A list of names associated with the official organizational role in different languages.
    pub names: Vec<OfficialOrganizationalRoleName>,
}

/// A name associated with the official organizational role in a specific language.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OfficialOrganizationalRoleName {
    /// The name in the local language.
    pub name: String,
    /// The language of the name.
    pub language: String,
    /// The language code (ISO) of the name.
    pub language_code: String,
    /// The transliterated name of the official organizational role.
    pub transliterated_name: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{model::common::GleifApiResponse, test_utils::test_model_files};
    use serde_json;
    use std::path::Path;

    #[test]
    fn test_deserialize_single_official_organizational_role() {
        let dir = Path::new("tests/data/official_organizational_roles");
        test_model_files(
            |filename| {
                filename.starts_with("official_organizational_role_")
                    && Path::new(filename)
                        .extension()
                        .is_some_and(|ext| ext.eq_ignore_ascii_case("json"))
            },
            |data| serde_json::from_str::<GleifApiResponse<OfficialOrganizationalRole>>(data),
            |filename, role| {
                assert!(
                    !role.data.id.is_empty(),
                    "OfficialOrganizationalRole id should not be empty in {filename}"
                );
                assert!(
                    !role.data.attributes.code.is_empty(),
                    "OfficialOrganizationalRole code should not be empty in {filename}"
                );
                assert!(
                    !role.data.attributes.names.is_empty(),
                    "OfficialOrganizationalRole should have at least one name in {filename}"
                );
            },
            dir,
        );
    }

    #[test]
    fn test_deserialize_official_organizational_roles_list() {
        let dir = Path::new("tests/data/official_organizational_roles");
        test_model_files(
            |filename| filename == "official_organizational_roles_all.json",
            |data| serde_json::from_str::<GleifApiResponse<Vec<OfficialOrganizationalRole>>>(data),
            |filename, roles| {
                assert!(
                    !roles.data.is_empty(),
                    "OfficialOrganizationalRoles list should not be empty in {filename}"
                );
                for role in &roles.data {
                    assert!(
                        !role.id.is_empty(),
                        "OfficialOrganizationalRole id should not be empty in {filename}"
                    );
                }
            },
            dir,
        );
    }
}
