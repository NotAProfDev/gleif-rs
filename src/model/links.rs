//! This module defines the structure for pagination links in a paginated API response.

use serde::Deserialize;

/// Represents pagination links in a paginated API response.
#[derive(Debug, Deserialize, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_deserialize_full_pagination_links() {
        let json_data = json!({
            "first": "https://api.example.com/first",
            "prev": "https://api.example.com/prev",
            "next": "https://api.example.com/next",
            "last": "https://api.example.com/last"
        });

        let links: PaginationLinks = serde_json::from_value(json_data).unwrap();
        assert_eq!(links.first, "https://api.example.com/first");
        assert_eq!(links.prev, Some("https://api.example.com/prev".to_string()));
        assert_eq!(links.next, Some("https://api.example.com/next".to_string()));
        assert_eq!(links.last, "https://api.example.com/last");
    }

    #[test]
    fn test_deserialize_partial_pagination_links() {
        let json_data = json!({
            "first": "https://api.example.com/first",
            "last": "https://api.example.com/last"
        });

        let links: PaginationLinks = serde_json::from_value(json_data).unwrap();
        assert_eq!(links.first, "https://api.example.com/first");
        assert_eq!(links.prev, None);
        assert_eq!(links.next, None);
        assert_eq!(links.last, "https://api.example.com/last");
    }

    #[test]
    fn test_deserialize_invalid_pagination_links() {
        let json_data = json!({
            "first": "https://api.example.com/first"
            // Missing required field "last"
        });

        let result: Result<PaginationLinks, _> = serde_json::from_value(json_data);
        assert!(result.is_err());
    }
}
