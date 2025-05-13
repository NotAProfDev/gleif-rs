//! Model definitions for the GLEIF relationship record endpoint.
//!
//! This module contains the data structures used to deserialize responses from the `/relationship-records` endpoint of the GLEIF API.
//! It provides Rust models for the relationship record resource, matching the JSON structure returned by the API.
//!
//! For endpoint usage and client methods, see [`crate::endpoint::relationship_record`] (`src/endpoint/relationship_record.rs`).

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
    /// The type of the data (e.g., "relationship-records").
    #[serde(rename = "type")]
    pub data_type: String,
    /// The unique identifier of the relationship record.
    pub id: String,
    /// The attributes of the relationship record.
    pub attributes: RelationshipRecordAttributes,
    /// The relationships associated with the relationship record.
    pub relationships: RelationshipRecordRelationships,
}

/// Attributes of a relationship record.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipRecordAttributes {
    /// The start date/time when the relationship is valid.
    pub valid_from: DateTime<Utc>,
    /// The end date/time when the relationship is valid, if available.
    pub valid_to: Option<DateTime<Utc>>,
    /// The details of the relationship.
    pub relationship: RelationshipDetails,
    /// The registration information for the relationship.
    pub registration: RelationshipRegistration,
    /// The extension information for the relationship.
    pub extension: RelationshipExtension,
}

/// Details of a relationship.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipDetails {
    /// The start node of the relationship.
    pub start_node: RelationshipNode,
    /// The end node of the relationship.
    pub end_node: RelationshipNode,
    /// The type of the relationship.
    #[serde(rename = "type")]
    pub relationship_type: RelationshipType,
    /// The status of the relationship.
    pub status: RelationshipStatus,
    /// The periods during which the relationship is/was valid.
    pub periods: Vec<RelationshipPeriod>,
}

/// A node in a relationship (start or end).
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipNode {
    /// The unique identifier of the node.
    pub id: String,
    /// The type of the node.
    #[serde(rename = "type")]
    pub node_type: String,
}

/// A period during which a relationship is/was valid.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipPeriod {
    /// The start date of the period.
    pub start_date: DateTime<Utc>,
    /// The end date of the period, if available.
    pub end_date: Option<DateTime<Utc>>,
    /// The type of the period.
    #[serde(rename = "type")]
    pub period_type: RelationshipPeriodType,
}

/// Registration information for a relationship.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipRegistration {
    /// The initial registration date.
    pub initial_registration_date: DateTime<Utc>,
    /// The last update date, if available.
    pub last_update_date: Option<DateTime<Utc>>,
    /// The registration status.
    pub status: RegistrationStatus,
    /// The next renewal date.
    pub next_renewal_date: DateTime<Utc>,
    /// The managing LOU.
    pub managing_lou: String,
    /// The corroboration level.
    pub corroboration_level: CorroborationLevel,
    /// The corroboration documents.
    pub corroboration_documents: CorroborationDocuments,
    /// The corroboration reference, if available.
    pub corroboration_reference: Option<String>,
}

/// Extension information for a relationship.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipExtension {
    /// The deletion date/time, if the relationship was deleted.
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Relationship links for the start and end nodes.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct RelationshipRecordRelationships {
    /// The links for the start node.
    pub start_node: RelationshipLinks,
    /// The links for the end node.
    pub end_node: RelationshipLinks,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{model::common::GleifApiResponse, test_utils::test_model_files};
    use std::path::Path;

    #[test]
    fn test_deserialize_single_relationship_records() {
        let dir = Path::new("tests/data/relationships");
        test_model_files(
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
        test_model_files(
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
