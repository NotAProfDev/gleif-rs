//! Model definitions for the GLEIF auto completion endpoint.
//!
//! This module contains the data structures used to deserialize responses from the `/auto-completions` endpoint of the GLEIF API.
//! It provides Rust models for the auto completion resource, matching the JSON structure returned by the API.
//!
//! For endpoint usage and client methods, see [`crate::endpoint::auto_completion`] (`src/endpoint/auto_completion.rs`).

use serde::{Deserialize, Serialize};

/// Represents a single auto completion result from the GLEIF API.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoCompletion {
    /// The type of the data (usually "autocompletions").
    #[serde(rename = "type")]
    pub data_type: String,
    /// Attributes of the auto completion result.
    pub attributes: AutoCompletionAttributes,
}

/// Attributes for an auto completion result, such as the matched value and highlighting.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoCompletionAttributes {
    /// The matched value for the auto completion.
    pub value: String,
    /// The highlighting information for the match.
    pub highlighting: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{model::common::GleifApiResponse, test_utils::test_model_files};
    use serde_json;
    use std::path::Path;

    #[test]
    fn test_deserialize_auto_completions_list() {
        let dir = Path::new("tests/data/auto_completions");
        test_model_files(
            |filename| filename == "autocompletions_all.json",
            |data| serde_json::from_str::<GleifApiResponse<Vec<AutoCompletion>>>(data),
            |filename, completions| {
                assert!(
                    !completions.data.is_empty(),
                    "Auto completions list should not be empty in {filename}"
                );
                for completion in &completions.data {
                    assert!(
                        !completion.attributes.value.is_empty(),
                        "Auto completion value should not be empty in {filename}"
                    );
                }
            },
            dir,
        );
    }
}
