//! This module defines type-safe enums for common GLEIF API field values.
//!
//! The enums in this module represent specific fields used in the GLEIF API, such as
//! `EntityCategory`, `RegistrationStatus`, and `ConformityFlag`. By using these enums,
//! developers can avoid typos and invalid values when constructing API requests, ensuring
//! better reliability and clarity in the code.
//!
//! Each enum provides:
//! - A `const fn as_str()` method to retrieve the canonical string representation for the API.
//! - Implementations of `fmt::Display` for ergonomic formatting and logging.
//! - Implementations of `AsRef<str>` to allow seamless usage in APIs expecting `&str`.
//!
//! # Examples
//!
//! ```rust
//! use gleif_rs::value::{EntityCategory, RegistrationStatus, ConformityFlag};
//!
//! let category = EntityCategory::Branch;
//! assert_eq!(category.as_str(), "BRANCH");
//! assert_eq!(category.to_string(), "BRANCH");
//!
//! let status = RegistrationStatus::Lapsed;
//! assert_eq!(status.as_str(), "LAPSED");
//!
//! let flag = ConformityFlag::NonConforming;
//! assert_eq!(flag.as_str(), "NON_CONFORMING");
//! ```

use std::fmt;

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

#[cfg(test)]
mod display_tests {
    use super::*;

    #[test]
    fn entity_category_display_and_asref() {
        let cat = EntityCategory::Branch;
        assert_eq!(cat.to_string(), "BRANCH");
        assert_eq!(cat.as_ref(), "BRANCH");
    }

    #[test]
    fn registration_status_display_and_asref() {
        let status = RegistrationStatus::Lapsed;
        assert_eq!(status.to_string(), "LAPSED");
        assert_eq!(status.as_ref(), "LAPSED");
    }

    #[test]
    fn conformity_flag_display_and_asref() {
        let flag = ConformityFlag::NonConforming;
        assert_eq!(flag.to_string(), "NON_CONFORMING");
        assert_eq!(flag.as_ref(), "NON_CONFORMING");
    }
}
