//! # GLEIF API Client Library
//!
//! This library provides a client for interacting with the Global Legal Entity Identifier Foundation (GLEIF) API.
//! It includes modules for constructing requests, handling responses, and working with type-safe representations
//! of GLEIF API fields and values.
//!
//! ## Features
//!
//! - **Client Module**: Provides the main entry point for making API requests.
//! - **Endpoint Module**: Defines the available API endpoints.
//! - **Error Handling**: Centralized error types for consistent error management.
//! - **Field and Value Modules**: Type-safe enums for API fields and values to reduce errors and improve code clarity.
//! - **Request Builder**: Utilities for constructing API requests with ease.
//! - **Test Utilities**: Helpers for testing deserialization and validation of API responses.
//!
//! ## Usage
//!
//! Add this library as a dependency in your `Cargo.toml` and use the `client` module to interact with the API.
//!
//! ### Basic Example
//! ```rust
//! use gleif_rs::client::GleifClient;
//! use gleif_rs::model::{LeiRecord, LeiRecordList};
//! use gleif_rs::field::Field;
//! use gleif_rs::value::{RegistrationStatus, EntityCategory};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), gleif_rs::error::GleifError> {
//!     // Example LEI from Bloomberg Finance L.P.
//!     let lei = "5493001KJTIIGC8Y1R12";
//!
//!     // Create a client with default settings
//!     let client = GleifClient::new();
//!     println!("GLEIF client created with base URL: {}", client.base_url());
//!
//!     // Example usage of the client to fetch LEI records raw data
//!     let lei_record_raw: serde_json::Value = client
//!         .lei_record_by_id(lei)
//!         .await?;
//!     println!("LEI Record: {:#?}", lei_record_raw);
//!
//!     // Example usage of the client to fetch LEI records strongly typed
//!     let lei_record_strongly_typed: LeiRecord = client
//!         .lei_record_by_id(lei)
//!         .await?;
//!     println!(
//!         "LEI Record (strongly typed): {:#?}",
//!         lei_record_strongly_typed
//!     );
//!
//!     // Example usage of the client to fetch LEI records with filters
//!     let lei_records: LeiRecordList = client
//!        .lei_records()
//!        .filter_eq(Field::EntityCategory, EntityCategory::Fund)
//!        .filter_eq(Field::RegistrationStatus, RegistrationStatus::Issued)
//!        .sort(Field::EntityLegalName)
//!        .page_size(10)
//!        .send()
//!        .await?;
//!     println!("Found {} records", lei_records.data.len());
//!     Ok(())
//! }
//! ```
//!
//! ### Advanced Client
//! ```rust
//! use gleif_rs::client::GleifClient;
//! use reqwest::Client as ReqwestClient;
//! use reqwest_middleware::ClientBuilder;
//! use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};
//! use std::time::Duration;
//!
//! #[tokio::main]
//! async fn main() {
//!     // Configure a custom reqwest client with timeouts
//!     let reqwest_client = ReqwestClient::builder()
//!         .timeout(Duration::from_secs(30))
//!         .connect_timeout(Duration::from_secs(5))
//!         .build()
//!         .expect("Failed to build reqwest client");
//!
//!     // Configure a retry policy with exponential backoff
//!     let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
//!
//!     // Create middleware with retry capability
//!     let middleware = ClientBuilder::new(reqwest_client)
//!         .with(RetryTransientMiddleware::new_with_policy(retry_policy))
//!         .build();
//!
//!     // Build the client with all configurations
//!     let client = GleifClient::from_middleware_client(middleware);
//!
//!     println!(
//!         "Advanced GLEIF client created with custom middleware and base_url: {}",
//!         client.base_url()
//!     );
//! }
//! ```
//!
//! ## Modules
//!
//! - [`crate::client`]: Main API client.
//! - [`crate::endpoint`]: Definitions of API endpoints.
//! - [`crate::error`]: Error types and handling.
//! - [`crate::field`]: Type-safe representations of API field names.
//! - [`crate::model`]: Data models for API responses.
//! - [`crate::request_builder`]: Utilities for building API requests.
//! - [`crate::test_utils`]: Utilities for testing the library.
//! - [`crate::value`]: Type-safe enums for common API field values.
//!
//! ## Constants
//!
//! - `VERSION`: The library version.
//! - `DEFAULT_BASE_URL`: The default base URL for the GLEIF API v1.

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
