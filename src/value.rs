//! # GLEIF API Field Value Enums ([`value`](crate::value)) - Type-Safe Enums for GLEIF API Field values
//!
//! This module provides a collection of type-safe enums that represent predefined,
//! known values for specific fields within the GLEIF API. For many API fields,
//! only a certain set of string values are accepted (e.g., entity categories,
//! registration statuses, conformity flags). Using these enums instead of raw strings
//! helps prevent typos, ensures you are using valid values recognized by the API,
//! and significantly improves code clarity and maintainability.
//!
//! When the GLEIF API expects a specific string like `"BRANCH"` for an entity category,
//! you can use [`EntityCategory::Branch`](EntityCategory) in your Rust code. This leverages the compiler
//! to catch errors early and makes the intent of your code more explicit.
//!
//! Each enum in this module typically corresponds to the set of allowed string values
//! for a single, distinct API field.
//!
//! ## Key Features
//!
//! - **Type Safety:** Guarantees that only valid, known values for specific API fields are used, caught at compile-time.
//! - **Clarity:** Makes the meaning of values explicit (e.g., [`EntityCategory::SoleProprietor`](EntityCategory) is clearer than just `"SOLE_PROPRIETOR"`).
//! - **String Conversion:**
//!     - A `const fn as_str()` method to retrieve the canonical string representation required by the GLEIF API.
//!     - Implementation of [`std::fmt::Display`] for easy use in formatted strings (e.g., for logging or UI).
//! - **String-like Usage:**
//!     - Implementation of [`AsRef<str>`] to allow enum instances to be seamlessly used in functions or APIs expecting `&str`.
//! - **Parsing from Strings:**
//!     - Implementation of [`std::str::FromStr`] to parse the API's string representation back into the corresponding enum variant.
//!
//! # Examples
//!
//! ## 1. Converting Enum Variants to Strings
//!
//! This is essential for providing the correct string values when constructing API requests or for display purposes.
//!
//! ```rust
//! use gleif_rs::value::{EntityCategory, RegistrationStatus, ConformityFlag};
//!
//! // Using as_str() for API interaction
//! let category_api_val = EntityCategory::Branch;
//! assert_eq!(category_api_val.as_str(), "BRANCH");
//!
//! // Using Display trait (e.g., for logging or string formatting)
//! let status_display = RegistrationStatus::Lapsed;
//! assert_eq!(status_display.to_string(), "LAPSED"); // or format!("{}", status_display)
//!
//! let flag_val = ConformityFlag::NonConforming;
//! assert_eq!(flag_val.as_str(), "NON_CONFORMING");
//!
//! println!("Category: {}, Status: {}, Flag: {}",
//!          EntityCategory::Fund.as_str(),
//!          RegistrationStatus::Issued.to_string(),
//!          ConformityFlag::Conforming.as_str());
//! ```
//!
//! ## 2. Parsing from Strings
//!
//! ```rust
//! use gleif_rs::value::{EntityCategory, RegistrationStatus, ConformityFlag};
//! use std::str::FromStr;
//!
//! let cat = EntityCategory::from_str("BRANCH").unwrap();
//! assert_eq!(cat, EntityCategory::Branch);
//! let status = RegistrationStatus::from_str("LAPSED").unwrap();
//! assert_eq!(status, RegistrationStatus::Lapsed);
//! let flag = ConformityFlag::from_str("NON_CONFORMING").unwrap();
//! assert_eq!(flag, ConformityFlag::NonConforming);
//! ```
//!
//! By utilizing the enums in this module, you enhance the robustness and clarity
//! of your interactions with the GLEIF API, catching potential errors related to field values
//! at compile time or through controlled parsing.

use crate::error::GleifError;
use std::{fmt, str::FromStr};

// Re-exporting the enums for external use
pub use crate::model::enums::{ConformityFlag, EntityCategory, RegistrationStatus};

impl EntityCategory {
    /// Returns the canonical string representation for the API.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            EntityCategory::General => "GENERAL",
            EntityCategory::Branch => "BRANCH",
            EntityCategory::Fund => "FUND",
            EntityCategory::SoleProprietor => "SOLE_PROPRIETOR",
            EntityCategory::ResidentGovernmentEntity => "RESIDENT_GOVERNMENT_ENTITY",
            EntityCategory::InternationalOrganization => "INTERNATIONAL_ORGANIZATION",
        }
    }
}
// Enables ergonomic formatting, logging, and string conversion, ensuring the canonical API value is always used.
impl fmt::Display for EntityCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// Allows passing EntityCategory directly to APIs expecting `&str` without allocating a new String.
impl AsRef<str> for EntityCategory {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl FromStr for EntityCategory {
    type Err = GleifError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GENERAL" => Ok(EntityCategory::General),
            "BRANCH" => Ok(EntityCategory::Branch),
            "FUND" => Ok(EntityCategory::Fund),
            "SOLE_PROPRIETOR" => Ok(EntityCategory::SoleProprietor),
            "RESIDENT_GOVERNMENT_ENTITY" => Ok(EntityCategory::ResidentGovernmentEntity),
            "INTERNATIONAL_ORGANIZATION" => Ok(EntityCategory::InternationalOrganization),
            _ => Err(GleifError::ValueParseError(
                "Unknown EntityCategory variant".to_string(),
            )),
        }
    }
}

impl RegistrationStatus {
    /// Returns the canonical string representation for the API.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            RegistrationStatus::PendingValidation => "PENDING_VALIDATION",
            RegistrationStatus::Issued => "ISSUED",
            RegistrationStatus::Duplicate => "DUPLICATE",
            RegistrationStatus::Lapsed => "LAPSED",
            RegistrationStatus::Merged => "MERGED",
            RegistrationStatus::Retired => "RETIRED",
            RegistrationStatus::Annulled => "ANNULLED",
            RegistrationStatus::Cancelled => "CANCELLED",
            RegistrationStatus::Transferred => "TRANSFERRED",
            RegistrationStatus::PendingTransfer => "PENDING_TRANSFER",
            RegistrationStatus::PendingArchival => "PENDING_ARCHIVAL",
            RegistrationStatus::Published => "PUBLISHED",
        }
    }
}

// Enables ergonomic formatting, logging, and string conversion, ensuring the canonical API value is always used.
impl fmt::Display for RegistrationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// Allows passing EntityCategory directly to APIs expecting `&str` without allocating a new String.
impl AsRef<str> for RegistrationStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl FromStr for RegistrationStatus {
    type Err = GleifError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PENDING_VALIDATION" => Ok(RegistrationStatus::PendingValidation),
            "ISSUED" => Ok(RegistrationStatus::Issued),
            "DUPLICATE" => Ok(RegistrationStatus::Duplicate),
            "LAPSED" => Ok(RegistrationStatus::Lapsed),
            "MERGED" => Ok(RegistrationStatus::Merged),
            "RETIRED" => Ok(RegistrationStatus::Retired),
            "ANNULLED" => Ok(RegistrationStatus::Annulled),
            "CANCELLED" => Ok(RegistrationStatus::Cancelled),
            "TRANSFERRED" => Ok(RegistrationStatus::Transferred),
            "PENDING_TRANSFER" => Ok(RegistrationStatus::PendingTransfer),
            "PENDING_ARCHIVAL" => Ok(RegistrationStatus::PendingArchival),
            "PUBLISHED" => Ok(RegistrationStatus::Published),
            _ => Err(GleifError::ValueParseError(
                "Unknown RegistrationStatus variant".to_string(),
            )),
        }
    }
}

impl ConformityFlag {
    /// Returns the canonical string representation for the API.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            ConformityFlag::Conforming => "CONFORMING",
            ConformityFlag::NonConforming => "NON_CONFORMING",
            ConformityFlag::NotApplicable => "NOT_APPLICABLE",
        }
    }
}

// Enables ergonomic formatting, logging, and string conversion, ensuring the canonical API value is always used.
impl fmt::Display for ConformityFlag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// Allows passing EntityCategory directly to APIs expecting `&str` without allocating a new String.
impl AsRef<str> for ConformityFlag {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl FromStr for ConformityFlag {
    type Err = GleifError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CONFORMING" => Ok(ConformityFlag::Conforming),
            "NON_CONFORMING" => Ok(ConformityFlag::NonConforming),
            "NOT_APPLICABLE" => Ok(ConformityFlag::NotApplicable),
            _ => Err(GleifError::ValueParseError(
                "Unknown ConformityFlag variant".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod display_tests {
    use super::*;

    #[test]
    fn entity_category_display_and_asref() {
        let cat = EntityCategory::Branch;
        assert_eq!(cat.to_string(), "BRANCH");
        assert_eq!(cat.as_ref(), "BRANCH");
        assert_eq!(cat.as_str(), "BRANCH");
        assert_eq!(EntityCategory::from_str("BRANCH").unwrap(), cat);
    }

    #[test]
    fn registration_status_display_and_asref() {
        let status = RegistrationStatus::Lapsed;
        assert_eq!(status.to_string(), "LAPSED");
        assert_eq!(status.as_ref(), "LAPSED");
        assert_eq!(status.as_str(), "LAPSED");
        assert_eq!(RegistrationStatus::from_str("LAPSED").unwrap(), status);
    }

    #[test]
    fn conformity_flag_display_and_asref() {
        let flag = ConformityFlag::NonConforming;
        assert_eq!(flag.to_string(), "NON_CONFORMING");
        assert_eq!(flag.as_ref(), "NON_CONFORMING");
        assert_eq!(flag.as_str(), "NON_CONFORMING");
        assert_eq!(ConformityFlag::from_str("NON_CONFORMING").unwrap(), flag);
    }
}
