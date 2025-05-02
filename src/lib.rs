pub mod client;
pub mod endpoint;
pub mod error;
pub mod request_builder;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The default base URL for the GLEIF API v1.
pub const DEFAULT_BASE_URL: &str = "https://api.gleif.org/api/v1";
