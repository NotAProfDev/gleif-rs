use chrono::{DateTime, Utc};
use serde::Deserialize;

/// Generic JSON:API response envelope for GLEIF API.
///
/// This struct is used for all GLEIF API endpoints, with `T` representing the type of the `data` field (single resource, Vec, or enum for polymorphic resources).
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct GleifApiResponse<T> {
    /// Metadata for the response (golden copy, pagination, etc).
    pub meta: Option<Meta>,
    /// Pagination and navigation links for the response.
    pub links: Option<PaginationLinks>,
    /// The main data payload (single resource, list, or enum).
    pub data: T,
}

/// Represents metadata in the Gleif API response.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Meta {
    /// The golden copy information.
    pub golden_copy: Option<GoldenCopy>,
    /// The pagination information.
    pub pagination: Option<Pagination>,
}

/// Represents the golden copy information.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GoldenCopy {
    /// The publish date of the golden copy.
    pub publish_date: DateTime<Utc>,
}

/// Represents pagination information.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    /// The current page number.
    pub current_page: u32,
    /// The number of items per page.
    pub per_page: u32,
    /// The starting item number.
    pub from: Option<u32>,
    /// The ending item number.
    pub to: Option<u32>,
    /// The total number of items.
    pub total: u32,
    /// The last page number.
    pub last_page: u32,
}

/// Represents pagination links in a paginated Gleif API response.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct PaginationLinks {
    /// The URL to the first page of results.
    pub first: String,
    /// The URL to the previous page of results.
    pub prev: Option<String>,
    /// The URL to the next page of results.
    pub next: Option<String>,
    /// The URL to the last page of results.
    pub last: String,
}

/// Represents the links for a relationship.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct RelationshipLinks {
    /// The links data.
    pub links: RelatedLink,
}

/// Represents the data for a relationship link.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct RelatedLink {
    /// The reporting exception.
    pub reporting_exception: Option<String>,
    /// The relationship record.
    pub relationship_record: Option<String>,
    /// The relationship records.
    pub relationship_records: Option<String>,
    /// The related entity.
    pub related: Option<String>,
    /// The LEI record.
    pub lei_record: Option<String>,
}

/// Enum for all top-level GLEIF resource types, for polymorphic deserialization.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum GleifResource {
    #[serde(rename = "lei-records")]
    LeiRecord,
    #[serde(rename = "relationship-records")]
    RelationshipRecord,
    // Add more resource types as needed
}
