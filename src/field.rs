//! # API Field Definitions ([`Field`]) - Type-Safe Enum of GLEIF API Field Names
//!
//! This module provides the [`Field`] enum, a crucial tool for representing GLEIF API field names
//! in a type-safe manner. Interacting with APIs often involves specifying field names as strings
//! for filtering, sorting, or selecting data. The [`Field`] enum mitigates common issues
//! like typos and inconsistencies that arise from using raw strings ("stringly-typed" code).
//!
//! By using [`Field`] variants, you leverage the Rust compiler to catch errors at compile time,
//! improve code readability, and make your queries more maintainable, especially if GLEIF
//! API field names were to evolve.
//!
//! ## Core Purpose & Usage
//!
//! The [`Field`] enum is primarily intended for:
//!
//! - Constructing filter expressions for API queries (e.g., filter by `entity.legalName`).
//! - Specifying sort parameters (e.g., sort by `registration.initialRegistrationDate`).
//!
//! ## Key Features
//!
//! - **Type Safety:** Prevents typos in field names during query construction, catching errors at compile time.
//! - **Comprehensive Coverage:** Includes a wide array of known GLEIF API v1 field names. The naming convention
//!   of variants (e.g., `entity.legalName`) helps categorize them by their typical API object.
//! - **String Conversion:**
//!     - Easily get the canonical string representation required by the API using the [`as_str`](Field::as_str) method.
//!     - Implements [`std::fmt::Display`], so it can be seamlessly used in formatted strings (e.g., for logging).
//! - **Parsing from Strings:**
//!     - Supports parsing from string slices using the [`std::str::FromStr`] trait (e.g., `"lei".parse::<Field>()`).
//!     - Provides advanced parsing with [`parse_with_allowed`](Field::parse_with_allowed),
//!       allowing you to restrict parsing to a specific subset of fields, useful for validating user input
//!       or context-specific field lists.
//! - **Ergonomic Integration:** Designed for easy use within your API client logic.
//!
//! # Examples
//!
//! ## 1. Converting a [`Field`] to its String Representation
//!
//! This is essential when you need to pass the field name to the underlying API.
//!
//! ```rust
//! use gleif_rs::field::Field;
//!
//! // Using as_str()
//! let field_enum = Field::EntityLegalName;
//! assert_eq!(field_enum.as_str(), "entity.legalName");
//!
//! // Using Display trait (e.g., for logging or string formatting)
//! let display_str = format!("Querying field: {}", Field::Lei);
//! assert_eq!(display_str, "Querying field: lei");
//! ```
//!
//! ## 2. Parsing a String into a [`Field`]
//!
//! This is useful when processing input that specifies field names as strings.
//!
//! ```rust
//! use gleif_rs::field::Field;
//!
//! // Assuming Field implements FromStr
//! let parsed_field = Field::from_str("lei");
//! assert_eq!(parsed_field.unwrap(), Field::Lei);
//!
//! let invalid_parse = Field::from_str("nonExistentField");
//! assert!(invalid_parse.is_err());
//! ```
//!
//! ## 3. Advanced Parsing with Allowed Fields ([`parse_with_allowed`](Field::parse_with_allowed))
//!
//! This method allows restricting which fields are considered valid during parsing.
//!
//! ```rust
//! use gleif_rs::field::Field;
//!
//! // Only allow parsing 'lei' or 'entity.legalName'
//! let allowed_fields = [Field::Lei, Field::EntityLegalName];
//!
//! let field_allowed = Field::parse_with_allowed("lei", Some(&allowed_fields)).unwrap();
//! assert_eq!(field_allowed, Field::Lei);
//!
//! // Attempting to parse a field not in the allowed list will fail
//! let field_not_allowed = Field::parse_with_allowed("entity.status", Some(&allowed_fields));
//! assert!(field_not_allowed.is_err());
//! ```
//!
//! By incorporating [`Field`] into your application, you create more robust, readable,
//! and maintainable code for interacting with the GLEIF API.

use crate::error::{GleifError, Result};
use std::{fmt, str::FromStr};

/// Enum for known GLEIF API field names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Field {
    // Core identifiers
    /// The Legal Entity Identifier code (20-character alphanumeric code)
    Lei,
    /// The Business Identifier Code (`BIC`/`SWIFT` code)
    Bic,
    /// The International Securities Identification Number (`ISIN`)
    Isin,

    // Entity fields
    /// Legal name of the entity (official registered name)
    EntityLegalName,
    /// Other names the entity is known by (trade names, etc.)
    EntityOtherNames,
    /// Legal form of the entity (`LLC`, `Inc`, `GmbH`, etc.)
    EntityLegalForm,
    /// ID code for the legal form
    EntityLegalFormId,
    /// Legal form code according to the entity's local jurisdiction
    EntityLegalFormCode,
    /// Category of the legal entity (`FUND`, `BRANCH`, etc.)
    EntityCategory,
    /// Country code of the entity's legal address
    EntityLegalAddressCountry,
    /// First line of the entity's legal address
    EntityLegalAddressLine1,
    /// City of the entity's legal address
    EntityLegalAddressCity,
    /// Postal code of the entity's legal address
    EntityLegalAddressPostalCode,
    /// Country code of the entity's headquarters address
    EntityHqAddressCountry,
    /// First line of the entity's headquarters address
    EntityHqAddressLine1,
    /// City of the entity's headquarters address
    EntityHqAddressCity,
    /// Postal code of the entity's headquarters address
    EntityHqAddressPostalCode,
    /// Business register number for the entity
    EntityBusinessRegisterNumber,
    /// Legal jurisdiction of the entity
    EntityJurisdiction,

    // Registration fields
    /// Status of the LEI registration (`ISSUED`, `LAPSED`, etc.)
    RegistrationStatus,
    /// Initial registration date of the LEI
    RegistrationInitialRegistrationDate,
    /// Last update date of the LEI record
    RegistrationLastUpdateDate,
    /// Next renewal date for the LEI
    RegistrationNextRenewalDate,
    /// The LEI issuer (Local Operating Unit) that manages this LEI
    RegistrationManagingLou,
    /// Data quality conformity status flag
    ConformityFlag,

    // Relationship fields (Level 2 data)
    /// Filter for relationships where the entity owns others
    Owns,
    /// Filter for relationships where the entity is owned by others
    OwnedBy,
    /// Start date of a relationship
    RelationshipStartDate,
    /// End date of a relationship
    RelationshipEndDate,
    /// Status of a relationship (`ACTIVE`, `INACTIVE`)
    RelationshipStatus,
    /// Type of relationship between entities
    RelationshipType,

    // Cross-field search
    /// Searches all text fields in an LEI record
    Fulltext,
}

impl Field {
    /// Returns the canonical string representation for the API.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            // Core identifiers
            Field::Lei => "lei",
            Field::Bic => "bic",
            Field::Isin => "isin",

            // Entity fields
            Field::EntityLegalName => "entity.legalName",
            Field::EntityOtherNames => "entity.otherNames",
            Field::EntityLegalForm => "entity.legalForm",
            Field::EntityLegalFormId => "entity.legalForm.id",
            Field::EntityLegalFormCode => "entity.legalForm.code",
            Field::EntityCategory => "entity.category",
            Field::EntityLegalAddressCountry => "entity.legalAddress.country",
            Field::EntityLegalAddressLine1 => "entity.legalAddress.line1",
            Field::EntityLegalAddressCity => "entity.legalAddress.city",
            Field::EntityLegalAddressPostalCode => "entity.legalAddress.postalCode",
            Field::EntityHqAddressCountry => "entity.headquartersAddress.country",
            Field::EntityHqAddressLine1 => "entity.headquartersAddress.line1",
            Field::EntityHqAddressCity => "entity.headquartersAddress.city",
            Field::EntityHqAddressPostalCode => "entity.headquartersAddress.postalCode",
            Field::EntityBusinessRegisterNumber => "entity.registeredAs",
            Field::EntityJurisdiction => "entity.jurisdiction",

            // Registration fields
            Field::RegistrationStatus => "registration.status",
            Field::RegistrationInitialRegistrationDate => "registration.initialRegistrationDate",
            Field::RegistrationLastUpdateDate => "registration.lastUpdateDate",
            Field::RegistrationNextRenewalDate => "registration.nextRenewalDate",
            Field::RegistrationManagingLou => "registration.managingLou",
            Field::ConformityFlag => "conformity_flag",

            // Relationship fields
            Field::Owns => "owns",
            Field::OwnedBy => "ownedBy",
            Field::RelationshipStartDate => "relationship.startDate",
            Field::RelationshipEndDate => "relationship.endDate",
            Field::RelationshipStatus => "relationship.status",
            Field::RelationshipType => "relationship.type",

            // Cross-field search
            Field::Fulltext => "fulltext",
        }
    }

    /// Parse a string to a Field, optionally restricting to a set of allowed values.
    ///
    /// # Errors
    ///
    /// Returns [`crate::error::GleifError::FieldParseError`] if the input string is not a valid field name or is not allowed.
    pub fn parse_with_allowed(input: &str, allowed: Option<&[Field]>) -> Result<Field> {
        let parsed = match input {
            "lei" => Field::Lei,
            "bic" => Field::Bic,
            "isin" => Field::Isin,
            "entity.legalName" => Field::EntityLegalName,
            "entity.otherNames" => Field::EntityOtherNames,
            "entity.legalForm" => Field::EntityLegalForm,
            "entity.legalForm.id" => Field::EntityLegalFormId,
            "entity.legalForm.code" => Field::EntityLegalFormCode,
            "entity.category" => Field::EntityCategory,
            "entity.legalAddress.country" => Field::EntityLegalAddressCountry,
            "entity.legalAddress.line1" => Field::EntityLegalAddressLine1,
            "entity.legalAddress.city" => Field::EntityLegalAddressCity,
            "entity.legalAddress.postalCode" => Field::EntityLegalAddressPostalCode,
            "entity.headquartersAddress.country" => Field::EntityHqAddressCountry,
            "entity.headquartersAddress.line1" => Field::EntityHqAddressLine1,
            "entity.headquartersAddress.city" => Field::EntityHqAddressCity,
            "entity.headquartersAddress.postalCode" => Field::EntityHqAddressPostalCode,
            "entity.registeredAs" => Field::EntityBusinessRegisterNumber,
            "entity.jurisdiction" => Field::EntityJurisdiction,
            "registration.status" => Field::RegistrationStatus,
            "registration.initialRegistrationDate" => Field::RegistrationInitialRegistrationDate,
            "registration.lastUpdateDate" => Field::RegistrationLastUpdateDate,
            "registration.nextRenewalDate" => Field::RegistrationNextRenewalDate,
            "registration.managingLou" => Field::RegistrationManagingLou,
            "conformity_flag" => Field::ConformityFlag,
            "owns" => Field::Owns,
            "ownedBy" => Field::OwnedBy,
            "relationship.startDate" => Field::RelationshipStartDate,
            "relationship.endDate" => Field::RelationshipEndDate,
            "relationship.status" => Field::RelationshipStatus,
            "relationship.type" => Field::RelationshipType,
            "fulltext" => Field::Fulltext,
            _ => {
                return Err(GleifError::FieldParseError(
                    "Unknown field name".to_string(),
                ));
            }
        };
        if let Some(allowed) = allowed {
            if !allowed.contains(&parsed) {
                return Err(GleifError::FieldParseError(
                    "Field not allowed for this operation".to_string(),
                ));
            }
        }
        Ok(parsed)
    }
}

// This allows Field to be used seamlessly in formatting macros, logging, and string conversions.
impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// This enables passing Field directly to APIs expecting `&str` without allocating a new String.
impl AsRef<str> for Field {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

// This allows parsing a Field from a string slice, enabling easy conversion from user input.
impl FromStr for Field {
    type Err = GleifError;

    fn from_str(s: &str) -> Result<Self> {
        Field::parse_with_allowed(s, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_as_str() {
        assert_eq!(Field::Lei.as_str(), "lei");
        assert_eq!(Field::EntityLegalName.as_str(), "entity.legalName");
        assert_eq!(Field::RegistrationStatus.as_str(), "registration.status");
        assert_eq!(Field::Owns.as_str(), "owns");
    }

    #[test]
    fn test_field_display() {
        assert_eq!(Field::Lei.to_string(), "lei");
        assert_eq!(Field::EntityLegalName.to_string(), "entity.legalName");
        assert_eq!(Field::RegistrationStatus.to_string(), "registration.status");
    }

    #[test]
    fn test_field_from_str() {
        use std::str::FromStr;
        assert_eq!(Field::from_str("lei").unwrap(), Field::Lei);
        assert_eq!(
            Field::from_str("entity.legalName").unwrap(),
            Field::EntityLegalName
        );
        assert_eq!(
            Field::from_str("registration.status").unwrap(),
            Field::RegistrationStatus
        );
        assert!(Field::from_str("not_a_field").is_err());
    }
}
