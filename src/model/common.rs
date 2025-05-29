//! Common model definitions for the GLEIF API.
//!
//! This module contains shared data structures used across multiple GLEIF API endpoints. These models
//! are designed to deserialize JSON:API-compliant responses from the GLEIF API, including metadata,
//! pagination, and relationship links.
//!
//! For endpoint-specific usage and client methods, refer to the corresponding modules in [`crate::endpoint`].

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Generic JSON:API response envelope for GLEIF API.
///
/// This struct is used for all GLEIF API endpoints, with `T` representing the type of the `data` field (single resource, Vec, or enum for polymorphic resources).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GleifApiResponse<T> {
    /// Metadata for the response (golden copy, pagination, etc).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    /// Pagination and navigation links for the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<PaginationLinks>,
    /// The main data payload (single resource, list, or enum).
    pub data: T,
}

/// Represents metadata in the Gleif API response.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Meta {
    /// The golden copy information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub golden_copy: Option<GoldenCopy>,
    /// The pagination information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Pagination>,
}

/// Represents the golden copy information.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoldenCopy {
    /// The publish date of the golden copy.
    pub publish_date: DateTime<Utc>,
}

/// Represents pagination information.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    /// The current page number.
    pub current_page: u32,
    /// The number of items per page.
    pub per_page: u32,
    /// The starting item number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<u32>,
    /// The ending item number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<u32>,
    /// The total number of items.
    pub total: u32,
    /// The last page number.
    pub last_page: u32,
}

/// Represents pagination links in a paginated Gleif API response.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaginationLinks {
    /// The URL to the first page of results.
    pub first: String,
    /// The URL to the previous page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    /// The URL to the next page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    /// The URL to the last page of results.
    pub last: String,
}

/// Represents the links for a relationship.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelationshipLinks {
    /// The links data.
    pub links: RelatedLink,
}

/// Represents the data for a relationship link.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct RelatedLink {
    /// The reporting exception.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporting_exception: Option<String>,
    /// The relationship record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_record: Option<String>,
    /// The relationship records.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_records: Option<String>,
    /// The related entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related: Option<String>,
    /// The LEI record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lei_record: Option<String>,
}
