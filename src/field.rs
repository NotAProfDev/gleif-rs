//! Field Module
//!
//! This module defines the `Field` enum, which provides type-safe representations of field names
//! used in GLEIF API queries. By using this enum, developers can avoid typos and reduce reliance
//! on stringly-typed code when constructing API requests.
//!
//! # Features
//!
//! - Comprehensive list of known GLEIF API field names, categorized by their purpose.
//! - Methods for converting fields to their canonical string representations (`as_str`).
//! - Parsing functionality to convert strings into `Field` variants, with optional restrictions
//!   on allowed fields (`parse_with_allowed`).
//! - Integration with formatting and string conversion traits for seamless usage in APIs and logs.
//!
//! # Examples
//!
//! ## Convert a Field to its String Representation
//! ```rust
//! use gleif_rs::field::Field;
//!
//! let field = Field::EntityLegalName;
//! assert_eq!(field.as_str(), "entity.legalName");
//! ```
//!
//! ## Parse a String into a Field
//! ```rust
//! use gleif_rs::field::Field;
//!
//! let field = Field::parse_with_allowed("lei", None).unwrap();
//! assert_eq!(field, Field::Lei);
//! ```

use crate::error::{GleifError, Result};
use std::fmt;

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
    /// Returns [`GleifError::FieldParseError`] if the input string is not a valid field name or is not allowed.
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
}
