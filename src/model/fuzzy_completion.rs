//! Model definitions for the GLEIF fuzzy completion endpoint.
//!
//! This module contains the data structures used to deserialize responses from the `/fuzzy-completions` endpoint of the GLEIF API.
//! It provides Rust models for the fuzzy completion resource, matching the JSON structure returned by the API.
//!
//! For endpoint usage and client methods, see [`crate::endpoint::fuzzy_completion`] (`src/endpoint/fuzzy_completion.rs`).

use crate::model::common::RelatedLink;
use serde::{Deserialize, Serialize};

/// Represents a single fuzzy completion result from the GLEIF API.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FuzzyCompletion {
    /// The type of the fuzzy completion result (e.g., "fuzzy-completions").
    pub r#type: String,
    /// Attributes of the fuzzy completion result, such as the matched value.
    pub attributes: FuzzyCompletionAttributes,
    /// Relationships to related LEI records for this fuzzy completion.
    pub relationships: FuzzyCompletionRelationships,
}

/// Attributes for a fuzzy completion result, such as the matched value.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FuzzyCompletionAttributes {
    /// The matched value for the fuzzy completion result.
    pub value: String,
}

/// Relationships for a fuzzy completion, including related LEI records.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct FuzzyCompletionRelationships {
    /// Related LEI records for this fuzzy completion.
    pub lei_records: FuzzyCompletionLeiRecords,
}

/// Data and links for a related LEI record in a fuzzy completion.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FuzzyCompletionLeiRecords {
    /// Data about the related LEI record, such as its type and ID.
    pub data: FuzzyCompletionLeiRecordsData,
    /// Links to the related LEI record resource.
    pub links: RelatedLink,
}

/// Data for a related LEI record, including type and ID.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FuzzyCompletionLeiRecordsData {
    /// The type of the related resource (usually "lei-records").
    pub r#type: String,
    /// The LEI identifier for the related record.
    pub id: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{model::common::GleifApiResponse, test_utils::test_model_files};
    use serde_json;
    use std::path::Path;

    #[test]
    fn test_deserialize_fuzzy_completions_list() {
        let dir = Path::new("tests/data/fuzzy_completions");
        test_model_files(
            |filename| filename == "fuzzycompletions_all.json",
            |data| serde_json::from_str::<GleifApiResponse<Vec<FuzzyCompletion>>>(data),
            |filename, completions| {
                assert!(
                    !completions.data.is_empty(),
                    "Fuzzy completions list should not be empty in {filename}"
                );
                for completion in &completions.data {
                    assert!(
                        !completion.attributes.value.is_empty(),
                        "Fuzzy completion value should not be empty in {filename}"
                    );
                }
            },
            dir,
        );
    }
}
