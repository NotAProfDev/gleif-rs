//! Endpoint-specific ergonomic methods for the GLEIF API client.
//!
//! This module provides convenient, idiomatic access to all primary GLEIF API endpoints
//! using the generic `GleifRequestBuilder` and consistent naming.
//!
//! # Examples
//!
//! ## Working with collections
//!
//! ```rust
//! use gleif_rs::client::GleifClient;
//! use gleif_rs::field::Field;
//! use gleif_rs::value::{RegistrationStatus, EntityCategory};
//!
//! # async fn example() -> Result<(), gleif_rs::error::GleifError> {
//! let client = GleifClient::new();
//!
//! // Search for LEI records with a mix of string and enum filters
//! let results: serde_json::Value = client.lei_records()
//!     .filter_eq(Field::RegistrationStatus, RegistrationStatus::Issued)
//!     .filter_eq("entity.legalAddress.country", "DE")
//!     .filter_eq(Field::EntityCategory, EntityCategory::Fund)
//!     .sort(Field::EntityLegalName)
//!     .page_size(10)
//!     .send()
//!     .await?;
//!
//! println!("Found {} records", results["data"].as_array().unwrap().len());
//!
//! // Similarly for other collection endpoints (using string literals)
//! let issuers: serde_json::Value = client.lei_issuers()
//!     .filter_eq("status", "ACTIVE")
//!     .page_size(5)
//!     .send()
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Fetching single entities
//!
//! ```rust
//! use gleif_rs::client::GleifClient;
//! use gleif_rs::field::Field;
//!
//! # async fn example() -> Result<(), gleif_rs::error::GleifError> {
//! let client = GleifClient::new();
//! let lei = "5493000IBP32UQZ0KL24";
//!
//! // Fetch a specific LEI record
//! let record: serde_json::Value = client.lei_record_by_id(lei).await?;
//! println!("Entity name: {}", record["attributes"]["entity"][Field::EntityLegalName.to_string()]);
//!
//! // Fetch relationship data
//! let parent: serde_json::Value = client.direct_parent_relationship(lei).await?;
//! if let Some(relationship) = parent.get("data") {
//!     println!("Has direct parent: {}", !relationship.is_null());
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Working with parent-child relationships
//!
//! ```rust
//! use gleif_rs::client::GleifClient;
//!
//! # async fn example() -> Result<(), gleif_rs::error::GleifError> {
//! let client = GleifClient::new();
//! let lei = "5493000IBP32UQZ0KL24";
//!
//! // Get all direct children (first page only)
//! let page: serde_json::Value = client.direct_children(lei)
//!     .page_size(100)
//!     .send()
//!     .await?;
//!
//! let records = page["data"].as_array().unwrap();
//! for record in records {
//!     let child_lei = record["id"].as_str().unwrap();
//!     println!("Child LEI: {child_lei}");
//! }
//! println!("Total children on first page: {}", records.len());
//! # Ok(())
//! # }
//! ```
//!
//! ## Using specialized endpoints
//!
//! ```rust
//! use gleif_rs::client::GleifClient;
//! use gleif_rs::field::Field;
//!
//! # async fn example() -> Result<(), gleif_rs::error::GleifError> {
//! let client = GleifClient::new();
//!
//! // Look up entity legal forms by country (using Field enum)
//! let legal_forms: serde_json::Value = client.entity_legal_forms()
//!     .filter_eq(Field::EntityLegalAddressCountry, "GB")
//!     .send()
//!     .await?;
//!     
//! for form in legal_forms["data"].as_array().unwrap() {
//!     let code = form["id"].as_str().unwrap();
//!     let name = form["attributes"]["name"].as_str().unwrap();
//!     println!("{code}: {name}");
//! }
//!
//! // Look up countries (string literal)
//! let country: serde_json::Value = client.country_by_code("US").await?;
//! println!("Country: {}", country["attributes"]["name"]);
//! # Ok(())
//! # }
//! ```

pub mod isin;
pub mod lei_issuer;
pub mod lei_record;
pub mod metadata;
pub mod relationship_record;
pub mod reporting_exception;
pub mod vlei_issuer;
