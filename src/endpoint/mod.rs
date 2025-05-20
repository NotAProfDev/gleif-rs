//! # GLEIF API Endpoints
//!
//! This module provides high-level, ergonomic access to the [Global Legal Entity Identifier Foundation (GLEIF) API](https://www.gleif.org/en/lei-data/gleif-api).
//!
//! It offers endpoint-specific methods via the [`GleifClient`](crate::client::GleifClient), wrapping the
//! lower-level [`GleifRequestBuilder`](crate::request_builder::GleifRequestBuilder) for a more convenient and type-safe experience. All methods are asynchronous and return a `Result` type, making them suitable for use in async Rust applications.
//!
//! ## Submodules
//!
//! Each submodule corresponds to a specific GLEIF API endpoint or data type:
//!
//! - [`auto_completion`](crate::endpoint::auto_completion): Auto-complete search queries.
//! - [`country`](crate::endpoint::country): Country information.
//! - [`entity_legal_form`](crate::endpoint::entity_legal_form): Legal entity structures.
//! - [`field`](crate::endpoint::field): Data fields.
//! - [`field_modification`](crate::endpoint::field_modification): Field change history.
//! - [`fuzzy_completion`](crate::endpoint::fuzzy_completion): Fuzzy search completions.
//! - [`isin`](crate::endpoint::isin): ISIN data.
//! - [`jurisdiction`](crate::endpoint::jurisdiction): Jurisdiction details.
//! - [`lei_issuer`](crate::endpoint::lei_issuer): LEI issuers.
//! - [`lei_record`](crate::endpoint::lei_record): LEI records.
//! - [`official_organizational_role`](crate::endpoint::official_organizational_role): Organizational roles.
//! - [`region`](crate::endpoint::region): Regional data.
//! - [`registration_agent`](crate::endpoint::registration_agent): Registration agents.
//! - [`registration_authority`](crate::endpoint::registration_authority): Registration authorities.
//! - [`relationship_record`](crate::endpoint::relationship_record): Entity relationships.
//! - [`reporting_exception`](crate::endpoint::reporting_exception): Reporting exceptions.
//! - [`vlei_issuer`](crate::endpoint::vlei_issuer): vLEI issuer data.
//!
//! ## Usage Examples
//!
//! ### Querying LEI Records
//!
//! You can easily search for multiple records, like LEIs or LEI issuers, and apply filters or sorting.
//!
//! ```rust
//! use gleif_rs::{
//!     client::GleifClient,
//!     error::GleifError,
//!     field::Field,
//!     model::LeiRecordList,
//!     value::{EntityCategory, RegistrationStatus},
//! };
//!
//! # async fn example() -> Result<(), GleifError> {
//! let client = GleifClient::new();
//!
//! // Fetching LEI records with filters
//! let results: LeiRecordList = client.lei_records()
//!     .filter_eq(Field::RegistrationStatus, RegistrationStatus::Issued) // use enum for precision
//!     .filter_eq("entity.legalAddress.country", "DE") // or a string for flexibility
//!     .filter_eq(Field::EntityCategory, EntityCategory::Fund)
//!     .sort(Field::EntityLegalName)
//!     .page_size(10) // Get 10 records per page
//!     .send()
//!     .await?;
//!
//! println!("Found {} records", results.data.len());
//!
//! // Fetching LEI issuers with filters
//! let issuers: serde_json::Value = client.lei_issuers()
//!     .filter_eq("status", "ACTIVE")
//!     .page_size(5)
//!     .send()
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Fetching a Single Record
//!
//! If you know the LEI, you can fetch a specific record directly.
//!
//! ```rust
//! use gleif_rs::{client::GleifClient, error::GleifError, model::LeiRecord};
//!
//! # async fn example() -> Result<(), GleifError> {
//! let client = GleifClient::new();
//! let lei = "5493000IBP32UQZ0KL24"; // a specific LEI
//!
//! // Get the LEI record for the given LEI
//! let record: LeiRecord = client.lei_record_by_id(lei).await?;
//! println!("Entity name: {:?}", record.data.attributes.entity.legal_name);
//!
//! // You can also fetch related data, like parent relationships
//! let parent_relationship: serde_json::Value = client.direct_parent_relationship(lei).await?;
//! if let Some(relationship_data) = parent_relationship.get("data") {
//!     println!("Has direct parent: {}", !relationship_data.is_null());
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ### Exploring Relationships
//!
//! Discover how entities are connected, such as finding all direct children of an organization.
//!
//! ```rust
//! use gleif_rs::{client::GleifClient, error::GleifError, model::RelationshipRecordList};
//!
//! # async fn example() -> Result<(), GleifError> {
//! let client = GleifClient::new();
//! let lei = "5493000IBP32UQZ0KL24";
//! let children: RelationshipRecordList =
//!     client.direct_children(lei).page_size(100).send().await?;
//! for record in children.data {
//!     println!("Child LEI: {}", record.id);
//! }
//! println!("Total children on this page: {}", children.data.len());
//! # Ok(())
//! # }
//! ```
//!
//! ### Specialized Endpoints
//!
//! Some endpoints offer more specific lookups, like finding legal entity forms by country.
//!
//! ```rust
//! use gleif_rs::{client::GleifClient, error::GleifError, field::Field};
//!
//! # async fn example() -> Result<(), GleifError> {
//! let client = GleifClient::new();
//!
//! // Find all entity legal forms for Great Britain (GB)
//! let legal_forms: serde_json::Value = client
//!     .entity_legal_forms()
//!     .filter_eq(Field::EntityLegalAddressCountry, "GB") // Use the Field enum for accuracy
//!     .send()
//!     .await?;
//!
//! for form in legal_forms["data"].as_array().unwrap() {
//!     let code = form["id"].as_str().unwrap();
//!     let name = form["attributes"]["name"].as_str().unwrap();
//!     println!("Form Code: {}, Name: {}", code, name);
//! }
//!
//! // Look up a specific country by its code (using a string literal)
//! let country_info: serde_json::Value = client.country_by_id("US").await?;
//! println!("Country Name: {}", country_info["attributes"]["name"]);
//! # Ok(())
//! # }
//! ```
//!
//! All methods are async and return [`Result`](crate::error::Result) with [`GleifError`](crate::error::GleifError) on failure.

pub mod auto_completion;
pub mod country;
pub mod entity_legal_form;
pub mod field;
pub mod field_modification;
pub mod fuzzy_completion;
pub mod isin;
pub mod jurisdiction;
pub mod lei_issuer;
pub mod lei_record;
pub mod official_organizational_role;
pub mod region;
pub mod registration_agent;
pub mod registration_authority;
pub mod relationship_record;
pub mod reporting_exception;
pub mod vlei_issuer;
