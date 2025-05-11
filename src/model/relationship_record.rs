//! Strongly-typed model for GLEIF Relationship and Reporting Exception API responses.
//!
//! This module provides deserialization structs for relationship-records and reporting-exceptions endpoints.

use crate::model::common::RelationshipLinks;
use crate::model::enums::{
    CorroborationDocuments, CorroborationLevel, RegistrationStatus, RelationshipPeriodType,
    RelationshipStatus, RelationshipType,
};
use chrono::{DateTime, Utc};
use serde::Deserialize;

/// A single relationship record as returned by the GLEIF API.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct RelationshipRecord {
    #[serde(rename = "type")]
    pub data_type: String,
    pub id: String,
    pub attributes: RelationshipRecordAttributes,
    pub relationships: RelationshipRecordRelationships,
}

/// Attributes of a relationship record.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipRecordAttributes {
    pub valid_from: Option<DateTime<Utc>>,
    pub valid_to: Option<DateTime<Utc>>,
    pub relationship: RelationshipDetails,
    pub registration: Option<RelationshipRegistration>,
    pub extension: Option<RelationshipExtension>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipDetails {
    pub start_node: RelationshipNode,
    pub end_node: RelationshipNode,
    #[serde(rename = "type")]
    pub relationship_type: RelationshipType,
    pub status: RelationshipStatus,
    pub periods: Vec<RelationshipPeriod>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipNode {
    pub id: String,
    #[serde(rename = "type")]
    pub node_type: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipPeriod {
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    #[serde(rename = "type")]
    pub period_type: Option<RelationshipPeriodType>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipRegistration {
    pub initial_registration_date: Option<DateTime<Utc>>,
    pub last_update_date: Option<DateTime<Utc>>,
    pub status: Option<RegistrationStatus>,
    pub next_renewal_date: Option<DateTime<Utc>>,
    pub managing_lou: Option<String>,
    pub corroboration_level: Option<CorroborationLevel>,
    pub corroboration_documents: Option<CorroborationDocuments>,
    pub corroboration_reference: Option<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipExtension {
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct RelationshipRecordRelationships {
    pub start_node: Option<RelationshipLinks>,
    pub end_node: Option<RelationshipLinks>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{model::common::GleifApiResponse, test_utils::test_lei_record_files};
    use std::path::Path;

    #[test]
    fn test_deserialize_single_relationship_records() {
        let dir = Path::new("tests/data/relationships");
        test_lei_record_files(
            |filename| {
                filename.contains("direct-parent-relationship_")
                    || filename.contains("ultimate-parent-relationship_")
            },
            |data| serde_json::from_str::<GleifApiResponse<RelationshipRecord>>(data),
            |filename, record| {
                assert!(
                    !record.data.id.is_empty(),
                    "Relationship record id should not be empty in {filename}"
                );
                assert_eq!(
                    record.data.data_type, "relationship-records",
                    "Unexpected type in {}: {}",
                    filename, record.data.data_type
                );
            },
            dir,
        );
    }

    #[test]
    fn test_deserialize_multi_relationship_records() {
        let dir = Path::new("tests/data/relationships");
        test_lei_record_files(
            |filename| {
                filename.contains("direct-child-relationships_")
                    || filename.contains("ultimate-child-relationships_")
            },
            |data| serde_json::from_str::<GleifApiResponse<Vec<RelationshipRecord>>>(data),
            |filename, list| {
                // An empty list is valid for some relationships (e.g., ultimate children/parents)
                if list.data.is_empty() {
                    return;
                }
                assert!(
                    !list.data[0].id.is_empty(),
                    "LEI should not be empty in first record of {filename}"
                );
            },
            dir,
        );
    }
}
