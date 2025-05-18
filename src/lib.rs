//! # GLEIF API Client Library ([`gleif_rs`])
//!
//! This Rust library provides a type-safe, ergonomic client for interacting with the [Global Legal Entity Identifier Foundation (GLEIF) API](https://www.gleif.org/en/lei-data/gleif-api).
//! Whether you're retrieving LEI records, filtering data, or managing API responses, this library ensures seamless integration with strong typing and clear error handling.
//!
//! ## Features
//!
//! - **Simple API Requests:** Easily fetch and filter LEI records via the fluent interface ([`GleifClient::lei_records`](crate::client::GleifClient::lei_records)).
//! - **Type-Safe Fields & Values:** Use enums like [`Field`](crate::field::Field) and [`EntityCategory`](crate::value::EntityCategory) to avoid typos and invalid values.
//! - **Comprehensive Error Handling:** Centralized error management via [`GleifError`](crate::error::GleifError).
//! - **Customizable Requests:** Build and refine API queries with [`GleifRequestBuilder`](crate::request_builder::GleifRequestBuilder).
//! - **Extensible HTTP Client:** Bring your own [`reqwest::Client`] or use middleware for retries, logging, and more.
//!
//! ## Getting Started
//!
//! To use this library, add it to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! gleif_rs = "0.1"
//! ```
//!
//! ### Basic Example: Fetching an LEI Record
//!
//! The following example demonstrates how to fetch a specific LEI record using its ID.
//!
//! ```rust
//! use gleif_rs::{
//!     client::GleifClient,
//!     error::GleifError,
//!     model::LeiRecord,
//! };
//!
//! #[tokio::main]
//! async fn main() -> Result<(), GleifError> {
//!     // Initialize the GLEIF client
//!     let client = GleifClient::new();
//!     // Fetch a specific LEI record by its ID
//!     let lei_record: LeiRecord = client.lei_record_by_id("5493001KJTIIGC8Y1R12").await?;
//!
//!     println!("LEI Record: {lei_record:#?}");
//!     Ok(())
//! }
//! ```
//!
//! ### Filtering LEI Records
//!
//! You can refine your API queries using filters:
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
//! #[tokio::main]
//! async fn main() -> Result<(), GleifError> {
//!     // Initialize the GLEIF client
//!     let client = GleifClient::new();
//!     // Fetch LEI records with specific filters
//!     let lei_records: LeiRecordList = client
//!         .lei_records()
//!         .filter_eq(Field::EntityCategory, EntityCategory::Fund)
//!         .filter_eq(Field::RegistrationStatus, RegistrationStatus::Issued)
//!         .sort(Field::EntityLegalName)
//!         .page_size(10)
//!         .send()
//!         .await?;
//!
//!     println!("Found {} matching records.", lei_records.data.len());
//!     Ok(())
//! }
//! ```
//!
//! ### Advanced Configuration: Custom HTTP Client
//!
//! For greater control over API requests, such as retry policies and timeouts,
//! you can customize the underlying HTTP client:
//!
//! ```rust
//! use gleif_rs::client::GleifClient;
//! use reqwest::Client as ReqwestClient;
//! use reqwest_middleware::ClientBuilder;
//! use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};
//! use std::time::Duration;
//!
//! #[tokio::main]
//! async fn main() {
//!     let reqwest_client = ReqwestClient::builder()
//!         .timeout(Duration::from_secs(30))
//!         .connect_timeout(Duration::from_secs(5))
//!         .build()
//!         .expect("Failed to create reqwest client");
//!     let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
//!     let middleware = ClientBuilder::new(reqwest_client)
//!         .with(RetryTransientMiddleware::new_with_policy(retry_policy))
//!         .build();
//!     let client = GleifClient::from_middleware_client(middleware);
//!     println!("GLEIF client initialized with custom middleware: {}", client.base_url());
//! }
//! ```
//!
//! ## Modules
//!
//! - [`client`](crate::client): Main API client and configuration.
//! - [`endpoint`](crate::endpoint): Endpoint definitions and helpers.
//! - [`error`](crate::error): Error types for API operations.
//! - [`field`](crate::field): Strongly-typed field selectors.
//! - [`model`](crate::model): Data models for API responses.
//! - [`request_builder`](crate::request_builder): Fluent API for building requests.
//! - [`value`](crate::value): Enumerated types for field values.
//!
//! ## Error Handling
//!
//! All API methods return [`Result<T, GleifError>`](crate::error::Result). See the [`error`](crate::error) module for details.
//!
//! ## License
//!
//! This project is licensed under the MIT License. See [LICENSE](https://github.com/gleif-rs/gleif-rs/blob/main/LICENSE) for details.
//!
//! ---
//!
//! Feel free to explore and extend its capabilities based on your needs!

pub mod client;
pub mod endpoint;
pub mod error;
pub mod field;
pub mod model;
pub mod request_builder;
#[cfg(test)]
pub mod test_utils;
pub mod value;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The default base URL for the GLEIF API v1.
pub const DEFAULT_BASE_URL: &str = "https://api.gleif.org/api/v1/";
