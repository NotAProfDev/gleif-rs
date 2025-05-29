//! Model definitions for the GLEIF country endpoint.
//!
//! This module contains the data structures used to deserialize responses from the `/countries` endpoint of the GLEIF API.
//! It provides Rust models for the country resource, matching the JSON structure returned by the API.
//!
//! For endpoint usage and client methods, see [`crate::endpoint::country`] (`src/endpoint/country.rs`).

use serde::{Deserialize, Serialize};

/// A country record as returned by the GLEIF API.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Country {
    /// The type of the data (always "countries").
    pub r#type: String,
    /// ISO 3166-1 alpha-2 code of the country (e.g., "DE").
    pub id: String,
    /// The attributes of the country, including code and name.
    pub attributes: CountryAttributes,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
/// Attributes of a country as returned by the GLEIF API.
pub struct CountryAttributes {
    /// ISO 3166-1 alpha-2 code of the country (e.g., "DE").
    pub code: String,
    /// The full name of the country (e.g., "Germany").
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{model::common::GleifApiResponse, test_utils::test_model_files};
    use serde_json;
    use std::path::Path;

    #[test]
    fn test_deserialize_single_country() {
        let dir = Path::new("tests/data/countries");
        test_model_files(
            |filename| {
                filename.starts_with("country_")
                    && Path::new(filename)
                        .extension()
                        .is_some_and(|ext| ext.eq_ignore_ascii_case("json"))
            },
            |data| serde_json::from_str::<GleifApiResponse<Country>>(data),
            |filename, country| {
                assert!(
                    !country.data.id.is_empty(),
                    "Country id should not be empty in {filename}"
                );
                assert_eq!(
                    country.data.id, country.data.attributes.code,
                    "Country id and code should match in {filename}"
                );
                assert!(
                    !country.data.attributes.name.is_empty(),
                    "Country name should not be empty in {filename}"
                );
            },
            dir,
        );
    }

    #[test]
    fn test_deserialize_countries_list() {
        let dir = Path::new("tests/data/countries");
        test_model_files(
            |filename| filename == "countries_all.json",
            |data| serde_json::from_str::<GleifApiResponse<Vec<Country>>>(data),
            |filename, countries| {
                assert!(
                    !countries.data.is_empty(),
                    "Countries list should not be empty in {filename}"
                );
                for country in &countries.data {
                    assert!(
                        !country.id.is_empty(),
                        "Country id should not be empty in {filename}"
                    );
                }
            },
            dir,
        );
    }
}
