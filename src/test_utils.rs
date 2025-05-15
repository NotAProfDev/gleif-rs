//! This module provides utility functions for testing the `model` module.
//!
//! The utilities in this module are designed to assist with testing the deserialization
//! and validation of model files. These functions simplify the process of iterating over
//! test files, applying filters, and validating the correctness of the deserialized data.
//!
//! # Example
//!
//! ```rust
//! use std::path::Path;
//! use crate::test_utils::test_model_files;
//!
//! let test_dir = Path::new("path/to/test/files");
//! test_model_files(
//!     |filename| filename.ends_with(".json"),
//!     |data| serde_json::from_str::<MyModel>(data),
//!     |filename, value| assert!(value.is_valid(), "Invalid data in {filename}"),
//!     test_dir,
//! );
//! ```

//! Test utilities for the `model` module.
use std::fs;
use std::path::Path;

/// Helper to test deserialization and validation of model files.
///
/// # Parameters
/// - `file_filter`: Predicate to select relevant files by filename.
/// - `deserialize`: Function to deserialize file contents to a value.
/// - `validate`: Function to validate the deserialized value and filename.
/// - `test_dir`: Directory containing test files.
pub(crate) fn test_model_files<T, F, D, V, E>(
    file_filter: F,
    deserialize: D,
    validate: V,
    test_dir: &Path,
) where
    F: Fn(&str) -> bool,
    D: Fn(&str) -> Result<T, E>,
    V: Fn(&str, &T),
    E: std::fmt::Display,
{
    assert!(
        test_dir.is_dir(),
        "Test data directory not found: {test_dir:?}"
    );
    let mut checked_files = 0;
    for entry in fs::read_dir(test_dir).expect("Failed to read test data directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
        let filename = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
        if !file_filter(filename) {
            continue;
        }
        let data = fs::read_to_string(&path).expect("Failed to read JSON file");
        let value = match deserialize(&data) {
            Ok(val) => val,
            Err(e) => panic!("Failed to deserialize {path:?}: {e}"),
        };
        validate(filename, &value);
        checked_files += 1;
    }
    assert!(checked_files > 0, "No model files were checked");
}
