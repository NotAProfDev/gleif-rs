//! Model definitions for the GLEIF reporting exception endpoint.
//!
//! This module contains the data structures used to deserialize responses from the `/reporting-exceptions` endpoint of the GLEIF API.
//! It provides Rust models for the reporting exception resource, matching the JSON structure returned by the API.
//!
//! For endpoint usage and client methods, see [`crate::endpoint::reporting_exception`] (`src/endpoint/reporting_exception.rs`).

use crate::model::common::RelatedLink;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A single reporting exception as returned by the GLEIF API.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportingException {
    /// The type of the data (should be "reporting-exceptions").
    #[serde(rename = "type")]
    pub data_type: String,
    /// The unique identifier of the reporting exception.
    pub id: String,
    /// The attributes of the reporting exception.
    pub attributes: ReportingExceptionAttributes,
    /// The relationships of the reporting exception.
    pub relationships: ReportingExceptionRelationships,
}

/// Attributes of a reporting exception.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportingExceptionAttributes {
    /// The start date of the exception validity period (nullable).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<DateTime<Utc>>,
    /// The end date of the exception validity period (nullable).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<DateTime<Utc>>,
    /// The LEI to which this exception applies.
    pub lei: String,
    /// The exception category (e.g., `DIRECT_ACCOUNTING_CONSOLIDATION_PARENT`).
    pub category: String,
    /// The reason for the exception (e.g., `NO_KNOWN_PERSON`).
    pub reason: String,
    /// An optional reference for the exception.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

/// Relationships for a reporting exception.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ReportingExceptionRelationships {
    /// The related LEI record.
    #[serde(rename = "lei-record")]
    pub lei_record: ReportingExceptionLeiRecordRelationship,
}

/// Relationship to a LEI record from a reporting exception.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportingExceptionLeiRecordRelationship {
    /// The links object for the related LEI record.
    pub links: RelatedLink,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{model::common::GleifApiResponse, test_utils::test_model_files};
    use std::path::Path;

    #[test]
    fn test_deserialize_sample_reporting_exceptions() {
        test_model_files(
            |filename| {
                Path::new(filename)
                    .extension()
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("json"))
            },
            |data| serde_json::from_str::<GleifApiResponse<ReportingException>>(data),
            |filename, response| {
                let data = &response.data;
                assert_eq!(
                    data.data_type, "reporting-exceptions",
                    "Type mismatch in {filename}"
                );
                assert_eq!(
                    data.attributes.lei.len(),
                    20,
                    "LEI should be 20 characters in {filename}"
                );
                assert!(!data.id.is_empty(), "ID should not be empty in {filename}");
            },
            Path::new("tests/data/reporting_exceptions"),
        );
    }
}
