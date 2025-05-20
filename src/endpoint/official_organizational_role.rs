//! # GLEIF API Official Organizational Roles Metadata Endpoints
//!
//! The Official Organizational Roles (OOR) Code List endpoint contains ISO 5009 OOR codes. The OOR code is an alpha-numeric code
//! of six characters from the basic Latin character set. Official Organizational Roles represent specific roles or positions within
//! legal entities, as defined by ISO 5009. These roles are associated with legal forms across various jurisdictions and are essential for regulatory, compliance, and entity management applications.
//!
//! The module includes methods to:
//! - Fetch a list of all Official Organizational Roles, optionally filtered by specific criteria, with support for pagination.
//! - Fetch detailed information about a specific Official Organizational Role using its unique identifier, including name, country, and related metadata.
//!
//! ## Endpoints
//!
//! - `/official-organizational-roles`: Fetches a list of all Official Organizational Roles. Supports filtering and pagination.
//! - `/official-organizational-roles/{id}`: Fetches detailed information about a specific Official Organizational Role by its ID.
//!
//! ## Examples
//!
//! ```rust
//! use gleif_rs::{client::GleifClient};
//! use gleif_rs::model::{OfficialOrganizationalRole, OfficialOrganizationalRoleList};
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = GleifClient::new();
//!
//! // Fetch all Official Organizational Roles
//! let roles: OfficialOrganizationalRoleList = client.official_organizational_roles().send().await?; // strongly typed
//! let roles: serde_json::Value = client.official_organizational_roles().send().await?; // raw JSON
//!
//! // Fetch a specific Official Organizational Role by ID
//! let role: OfficialOrganizationalRole = client.official_organizational_role_by_id("0CGNG5").await?;
//! # Ok(()) }
//! ```
//!
//! ## Errors
//!
//! Errors may occur during network communication, server-side issues, or deserialization of the response.
//! These errors are returned as [`crate::error::GleifError`] instances. All methods in this module are asynchronous and return a `Result` type, making them suitable for use in async Rust applications.

use crate::{client::GleifClient, error::Result, request_builder::GleifRequestBuilder};
use serde::de::DeserializeOwned;

impl GleifClient {
    /// Retrieves all Official Organizational Roles (OOR) Code List (`/official-organizational-roles`).
    ///
    /// This method sends a request to fetch the complete list of official organizational roles.
    /// The list contains over 2100 official organizational roles (as of November 2024) for nearly
    /// 250 legal forms across 89 jurisdictions.
    ///
    /// # Filtering Options
    ///
    /// The following filters can be applied to narrow down the results:
    /// - **Primary Name**: Filter by the primary name of the role.
    /// - **Transliterated Name**: Filter by the transliterated name of the role.
    /// - **Case-insensitive Name**: Filter by a case-insensitive name match.
    /// - **Matching String**: Filter by a partial string match in the name.
    /// - **Country Code**: Filter by the ISO 3166 country code.
    ///
    /// # Errors
    ///
    /// This method does not itself return errors. However, errors may occur when sending the request or processing
    /// the response using the returned request builder (e.g., network failures or deserialization issues).
    ///
    /// # Examples
    ///
    /// ```rust, ignore
    /// // Retrieve all roles
    /// let roles: OfficialOrganizationalRoleList = client.official_organizational_roles().send().await?;
    ///
    /// // Filter by primary name
    /// let roles: OfficialOrganizationalRoleList = client
    ///     .official_organizational_roles()
    ///     .filter_eq("name", "управног")
    ///     .send()
    ///     .await?;
    ///
    /// // Filter by country code
    /// let roles: OfficialOrganizationalRoleList = client
    ///     .official_organizational_roles()
    ///     .filter_eq("countryCode", "CA")
    ///     .send()
    ///     .await?;
    /// ```
    #[must_use]
    pub fn official_organizational_roles(&self) -> GleifRequestBuilder {
        self.request("official-organizational-roles")
    }

    /// Fetches details of a single official organizational role by OOR code (`/official-organizational-roles/{id}`).
    ///
    /// This method sends a request to retrieve information about a specific official organizational role by its ID.
    ///
    /// # Parameters
    ///
    /// - `id` - A string slice representing the unique identifier of the official organizational role.
    ///
    /// # Errors
    ///
    /// This method returns a [`crate::error::GleifError`] in the following cases:
    /// - The request could not be completed due to network or server issues.
    /// - The response body could not be deserialized into the expected type.
    ///
    /// # Examples
    ///
    /// ```rust, ignore
    /// let role: OfficialOrganizationalRole = client.official_organizational_role_by_id("0CGNG5").await?; // strongly typed
    /// let role: serde_json::Value = client.official_organizational_role_by_id("0CGNG5").await?; // raw JSON
    /// ```
    pub async fn official_organizational_role_by_id<R>(&self, id: &str) -> Result<R>
    where
        R: DeserializeOwned,
    {
        self.request(&format!("official-organizational-roles/{id}"))
            .send()
            .await
    }
}
