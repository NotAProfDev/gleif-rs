//! This module defines the structures for metadata in the API response, including
//! information about the golden copy and pagination details.

use chrono::{DateTime, Utc};
use serde::Deserialize;

/// Represents metadata in the API response.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Meta {
    /// The golden copy information.
    pub golden_copy: Option<GoldenCopy>,
    /// The pagination information.
    pub pagination: Option<Pagination>,
}

/// Represents the golden copy information.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoldenCopy {
    /// The publish date of the golden copy.
    pub publish_date: DateTime<Utc>,
}

/// Represents pagination information.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    /// The current page number.
    pub current_page: u32,
    /// The number of items per page.
    pub per_page: u32,
    /// The starting item number.
    pub from: u32,
    /// The ending item number.
    pub to: u32,
    /// The total number of items.
    pub total: u32,
    /// The last page number.
    pub last_page: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_deserialize_full_meta() {
        let json_data = json!({
            "goldenCopy": {
                "publishDate": "2023-01-01T00:00:00Z"
            },
            "pagination": {
                "currentPage": 1,
                "perPage": 10,
                "from": 1,
                "to": 10,
                "total": 100,
                "lastPage": 10
            }
        });

        let meta: Meta = serde_json::from_value(json_data).unwrap();
        assert!(meta.golden_copy.is_some());
        assert!(meta.pagination.is_some());
    }

    #[test]
    fn test_deserialize_partial_meta() {
        let json_data = json!({
            "pagination": {
                "currentPage": 1,
                "perPage": 10,
                "from": 1,
                "to": 10,
                "total": 100,
                "lastPage": 10
            }
        });

        let meta: Meta = serde_json::from_value(json_data).unwrap();
        assert!(meta.golden_copy.is_none());
        assert!(meta.pagination.is_some());
    }

    #[test]
    fn test_deserialize_invalid_meta() {
        let json_data = json!({
            "invalid_field": "invalid_value"
        });

        let result: Result<Meta, _> = serde_json::from_value(json_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_full_golden_copy() {
        let json_data = json!({
            "publishDate": "2023-01-01T00:00:00Z"
        });

        let golden_copy: GoldenCopy = serde_json::from_value(json_data).unwrap();
        assert_eq!(
            golden_copy.publish_date,
            DateTime::parse_from_rfc3339("2023-01-01T00:00:00Z")
                .unwrap()
                .with_timezone(&Utc)
        );
    }

    #[test]
    fn test_deserialize_full_pagination() {
        let json_data = json!({
            "currentPage": 1,
            "perPage": 10,
            "from": 1,
            "to": 10,
            "total": 100,
            "lastPage": 10
        });

        let pagination: Pagination = serde_json::from_value(json_data).unwrap();
        assert_eq!(pagination.current_page, 1);
        assert_eq!(pagination.per_page, 10);
        assert_eq!(pagination.from, 1);
        assert_eq!(pagination.to, 10);
        assert_eq!(pagination.total, 100);
        assert_eq!(pagination.last_page, 10);
    }
}
