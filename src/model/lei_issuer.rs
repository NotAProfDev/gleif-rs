//! Model definitions for the GLEIF LEI Issuer endpoint.
//!
//! This module contains the data structures used to deserialize responses from the `/lei-issuers` endpoint of the GLEIF API.
//! It provides Rust models for the LEI Issuer resource, matching the JSON structure returned by the API.
//!
//! For endpoint usage and client methods, see [`crate::endpoint::lei_issuer`] (`src/endpoint/lei_issuer.rs`).

use crate::model::common::RelationshipLinks;
use chrono::{DateTime, Utc};
use serde::Deserialize;

/// Represents a LEI Issuer as returned by the GLEIF API.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct LeiIssuer {
    /// The type of the data (e.g., "leiIssuers").
    #[serde(rename = "type")]
    pub data_type: String,
    /// The unique identifier of the LEI Issuer.
    pub id: String,
    /// The attributes of the LEI Issuer.
    pub attributes: LeiIssuerAttributes,
    /// The relationships associated with the LEI Issuer, if any.
    pub relationships: Option<LeiIssuerRelationships>,
}

/// Attributes of a LEI Issuer as returned by the GLEIF API.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LeiIssuerAttributes {
    /// The LEI code of the issuer.
    pub lei: String,
    /// The name of the LEI Issuer.
    pub name: String,
    /// The marketing name of the LEI Issuer.
    pub marketing_name: String,
    /// The website of the LEI Issuer.
    pub website: String,
    /// The accreditation date of the LEI Issuer.
    pub accreditation_date: DateTime<Utc>,
}

/// Relationships associated with a LEI Issuer, such as the jurisdiction.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LeiIssuerRelationships {
    /// The jurisdiction(s) associated with the LEI Issuer.
    pub jurisdictions: RelationshipLinks,
    /// The fund jurisdiction(s) associated with the LEI Issuer.
    pub fund_jurisdictions: RelationshipLinks,
}

/// Represents a LEI Issuer Jurisdiction as returned by the GLEIF API.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct LeiIssuerJurisdiction {
    /// The type of the data (e.g., "lei-issuer-accredited-jurisdictions").
    #[serde(rename = "type")]
    pub data_type: String,
    /// The unique identifier of the LEI Issuer Jurisdiction.
    pub id: String,
    /// The attributes of the LEI Issuer Jurisdiction.
    pub attributes: LeiIssuerJurisdictionAttributes,
}

/// Attributes of a LEI Issuer Jurisdiction as returned by the GLEIF API.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LeiIssuerJurisdictionAttributes {
    /// The country code of the LEI Issuer Jurisdiction.
    pub country_code: String,
    /// The accreditation status of the LEI Issuer Jurisdiction.
    pub accredited_as: String,
    /// The start date of the accreditation.
    pub start_date: DateTime<Utc>,
    /// The end date of the accreditation, if applicable.
    pub end_date: Option<DateTime<Utc>>,
    /// Whether the LEI Issuer is accredited for funds.
    pub is_accredited_for_funds: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{model::common::GleifApiResponse, test_utils::test_model_files};
    use serde_json;
    use std::path::Path;

    #[test]
    fn test_deserialize_single_lei_issuer() {
        let dir = Path::new("tests/data/lei_issuers");
        test_model_files(
            |filename| {
                (filename.starts_with("lei_issuer_") || filename.starts_with("lei_record_issuer_"))
                    && !filename.starts_with("lei_issuer_jurisdictions_")
                    && !filename.starts_with("lei_issuers_all")
                    && Path::new(filename)
                        .extension()
                        .is_some_and(|ext| ext.eq_ignore_ascii_case("json"))
            },
            |data| serde_json::from_str::<GleifApiResponse<LeiIssuer>>(data),
            |filename, issuer| {
                assert!(
                    !issuer.data.id.is_empty(),
                    "LEI Issuer id should not be empty in {filename}"
                );
                assert!(
                    !issuer.data.attributes.name.is_empty(),
                    "LEI Issuer name should not be empty in {filename}"
                );
            },
            dir,
        );
    }

    #[test]
    fn test_deserialize_lei_issuers_list() {
        let dir = Path::new("tests/data/lei_issuers");
        test_model_files(
            |filename| filename == "lei_issuers_all.json",
            |data| serde_json::from_str::<GleifApiResponse<Vec<LeiIssuer>>>(data),
            |filename, issuers| {
                assert!(
                    !issuers.data.is_empty(),
                    "LEI Issuers list should not be empty in {filename}"
                );
                for issuer in &issuers.data {
                    assert!(
                        !issuer.id.is_empty(),
                        "LEI Issuer id should not be empty in {filename}"
                    );
                }
            },
            dir,
        );
    }

    #[test]
    fn test_deserialize_lei_issuer_jurisdiction_list() {
        let dir = Path::new("tests/data/lei_issuers");
        test_model_files(
            |filename| {
                filename.starts_with("lei_issuer_jurisdictions_")
                    && Path::new(filename)
                        .extension()
                        .is_some_and(|ext| ext.eq_ignore_ascii_case("json"))
            },
            |data| serde_json::from_str::<GleifApiResponse<Vec<LeiIssuerJurisdiction>>>(data),
            |filename, jurisdictions| {
                assert!(
                    !jurisdictions.data.is_empty(),
                    "LEI Issuer Jurisdictions list should not be empty in {filename}"
                );
                for jurisdiction in &jurisdictions.data {
                    assert!(
                        !jurisdiction.id.is_empty(),
                        "LEI Issuer Jurisdiction id should not be empty in {filename}"
                    );
                }
            },
            dir,
        );
    }
}
