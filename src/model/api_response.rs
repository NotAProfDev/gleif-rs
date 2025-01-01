//! This module defines the structure for the API response, including metadata, pagination links, and data.

use super::data::Data;
use super::links::PaginationLinks;
use super::meta::Meta;
use serde::Deserialize;

/// Represents the API response.
#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    /// The metadata of the response.
    pub meta: Option<Meta>,
    /// The pagination links of the response.
    pub links: Option<PaginationLinks>,
    /// The data of the response.
    pub data: Vec<Data>,
}
