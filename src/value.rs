//! Type-safe enums for common GLEIF API field values.
//!
//! These enums help prevent typos and invalid values when constructing API requests, improving reliability and clarity.

use serde::Deserialize;
use std::fmt;

/// Entity Categories for Legal Entities in the GLEIF database.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EntityCategory {
    /// The legal entity is a general legal entity.
    General,
    /// The legal entity is a branch of another legal entity.
    Branch,
    /// The legal entity is an investment fund defined as a collective investment scheme (or pooled investment) beneficially owned by multiple investors and managed on behalf of those investors by an asset manager or by the fund itself.
    Fund,
    /// The legal entity is an individual acting in a business capacity.
    SoleProprietor,
    /// The legal entity is a non-market producer, part of a central, state or local government, social security funds or a non-profit institution (NPI) controlled by government units.
    ResidentGovernmentEntity,
    /// The legal entity is a non-resident unit created by international agreements or other arrangements for the provision of non-market services or financial intermediation at the international level.
    InternationalOrganization,
}

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

/// Registration Statuses for LEI records in the GLEIF database.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RegistrationStatus {
    /// An application for an LEI that has been submitted and which is being processed and validated.
    PendingValidation,
    /// An LEI Registration that has been validated and issued, and which identifies an entity having a legal entity status of `ACTIVE` as of the Date of First LEI Assignment or the Date of Last Update of the LEI Data Record.
    Issued,
    /// An LEI Registration that has been determined to be a duplicate registration of the same legal entity as another LEI Registration; the `DUPLICATE` status is assigned to the non-surviving registration (i.e. the LEI that should no longer be used).
    Duplicate,
    /// An LEI registration that has not been renewed by the `NextRenewalDate` and is not known by public sources to have a legal entity status of `INACTIVE`.
    Lapsed,
    /// An LEI registration for an entity that has been merged into another legal entity, such that this legal entity no longer exists as an operating entity. (Deprecated as of `LEI-CDF 3.0`)
    Merged,
    /// An LEI registration status for an entity that has been affected by certain legal entity events.
    Retired,
    /// An LEI registration that was determined to be erroneous or invalid after it was issued.
    Annulled,
    /// An LEI registration that was abandoned prior to issuance of an LEI,
    Cancelled,
    /// An LEI registration that has been transferred to a different LOU as the managing LOU.
    Transferred,
    /// An LEI registration that has been requested to be transferred to another LOU. The request is being processed at the sending LOU.
    PendingTransfer,
    /// An LEI registration is about to be transferred to a different LOU, after which its registration status will revert to a non-pending status.
    PendingArchival,
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

/// Conformity Flags for LEI records in the GLEIF database.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ConformityFlag {
    /// If the LEI record is timely renewed and the Level 2 data reporting is complete.
    Conforming,
    /// If the LEI record is not timely renewed, and/or Level 2 data reporting is incomplete.
    NonConforming,
    /// If the LEI record is `RETIRED`, `ANNULLED`, or `DUPLICATE`.
    NotApplicable,
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
