//! Model definitions for the GLEIF LEI record endpoint.
//!
//! This module contains the data structures used to deserialize responses from the `/lei-records` endpoint of the GLEIF API.
//! It provides Rust models for the LEI record resource, matching the JSON structure returned by the API.
//!
//! For endpoint usage and client methods, see [`crate::endpoint::lei_record`] (`src/endpoint/lei_record.rs`).

use crate::model::{
    common::RelationshipLinks,
    enums::{
        AssociatedEntityType, ConformityFlag, CorroborationDocuments, CorroborationLevel,
        EntityCategory, EntityNameType, EntityStatus, EntitySubCategory, EventStatus, EventType,
        ExpirationReason, GroupType, OtherAddressType, RegistrationStatus, TransliteratedNameType,
    },
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A single LEI record as returned by the GLEIF API.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LeiRecord {
    /// The type of the data.
    #[serde(rename = "type")]
    pub data_type: String,
    /// The unique identifier of the data.
    pub id: String,
    /// The attributes of the data.
    pub attributes: LeiRecordAttributes,
    /// The relationships of the data.
    pub relationships: Relationships,
}

/// Attributes of a LEI record (core data).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeiRecordAttributes {
    /// A Legal Entity Identifier (LEI) code, in the format specified by ISO 17442.
    pub lei: String,
    /// The entity details.
    pub entity: Entity,
    /// The Registration container element contains all information on the legal entity's LEI registration with the `ManagingLOU`.
    pub registration: Registration,
    /// The optional Bank Identifier Codes (BIC).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bic: Option<Vec<String>>,
    /// The optional Market Identifier Codes (MIC).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mic: Option<Vec<String>>,
    /// The optional Open Corporates ID (OCID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocid: Option<String>,
    /// The optional QCC identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qcc: Option<String>,
    /// The optional S&P Global identifiers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spglobal: Option<Vec<String>>,
    /// The conformity flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conformity_flag: Option<ConformityFlag>,
}

/// Represents an entity with various details.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    /// The legal name of the legal entity.
    pub legal_name: Name,
    /// A list of other names (excluding transliterations) for the legal entity.
    pub other_names: Vec<OtherName>,
    /// A list of ASCII-transliterated (i.e. Latin- or Romanized) representations of names for the legal entity.
    pub transliterated_other_names: Vec<TransliteratedOtherName>,
    /// The address of the legal entity as recorded in the registration of the legal entity in its legal jurisdiction.
    pub legal_address: Address,
    /// The address of the headquarters of the legal entity.
    pub headquarters_address: Address,
    /// A list of other addresses for the legal entity, excluding transliterations.
    pub other_addresses: Vec<OtherAddress>,
    /// An optional list of transliterated addresses for the legal entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transliterated_other_addresses: Option<Vec<OtherAddress>>,
    /// Information about the official authority in the legal entity's jurisdiction of legal registration, for example a business registry, and the corresponding identification of the legal entity by that official source.
    pub registered_at: RegistrationAuthority,
    /// The registered name of the entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_as: Option<String>,
    /// The jurisdiction of legal formation of the legal entity; i.e. where the `LegalForm` of the legal entity originates.
    pub jurisdiction: String,
    /// Indicates (where applicable) the category of legal entity identified by this LEI data record, as a more specific category within the broad definition given in ISO 17442. These categories are based on use cases specified in ROC policies, found at <https://www.leiroc.org/leiroc_gls/index.htm>.
    pub category: EntityCategory,
    /// Indicates and specifies further (where applicable) the sub-category (sub-sector) of Legal Entity identified by this LEI Record and already categorized by the `EntityCategory` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<EntitySubCategory>,
    /// The legal form of the legal entity.
    pub legal_form: LegalForm,
    /// Another entity associated with this entity if needed to fully identify this entity or to place it in an appropriate context. (Deprecated as of LEI-CDF 3.0)
    pub associated_entity: AssociatedEntity, //GeneralEntity,
    /// The operational and/or legal registration status of the legal entity.
    pub status: EntityStatus,
    /// The date on which the legal entity was first established, as represented by ISO 8601 (as defined in ISO 17442).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<DateTime<Utc>>,
    /// The date and reason that the legal entity ceased to operate, whether due to dissolution, merger or acquisition. (Deprecated as of LEI-CDF 3.0)
    pub expiration: Expiration,
    /// One Legal Entity that continue or replace this LEI.
    pub successor_entity: SuccessorEntity,
    /// More Legal Entities that continue or replace this LEI.
    pub successor_entities: Vec<SuccessorEntity>,
    /// Container for a finite number of events specified by the ROC in the life of a legal entity that would impact the Reference Data in the Global LEI System.
    pub event_groups: Vec<EventGroup>,
}

/// Represents a name with optional language.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Name {
    /// The name. An element of this type has minimum length of one character and may not contain any of: the carriage return (#xD), line feed (#xA) nor tab (#x9) characters, shall not begin or end with a space (#x20) character, or a sequence of two or more adjacent space characters.
    pub name: String,
    /// The language of this element's text content. An IETF Language Code conforming to the latest RFC from IETF BCP 47. Note that the first characters of an IETF Language Code, up to the hyphen (if any), are all lowercase, and those following the hyphen (if any) are all uppercase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

/// Represents an alternative name with optional language and mandatory type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OtherName {
    /// The name. An element of this type has minimum length of one character and may not contain any of: the carriage return (#xD), line feed (#xA) nor tab (#x9) characters, shall not begin or end with a space (#x20) character, or a sequence of two or more adjacent space characters.
    pub name: String,
    /// The language of this element's text content. An IETF Language Code conforming to the latest RFC from IETF BCP 47. Note that the first characters of an IETF Language Code, up to the hyphen (if any), are all lowercase, and those following the hyphen (if any) are all uppercase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Type of alternative name for the legal entity.
    #[serde(rename = "type")]
    pub name_type: EntityNameType,
}

/// Represents a transliterated name with optional language and mandatory type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransliteratedOtherName {
    /// The name.
    pub name: String,
    /// The language of this element's text content. An IETF Language Code conforming to the latest RFC from IETF BCP 47. Note that the first characters of an IETF Language Code, up to the hyphen (if any), are all lowercase, and those following the hyphen (if any) are all uppercase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Type of alternative name for the legal entity.
    #[serde(rename = "type")]
    pub name_type: TransliteratedNameType,
}

/// Represents an address with various details.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    /// The language in which all of the string-valued components of this address are expressed. An IETF Language Code conforming to the latest RFC from IETF BCP 47. Note that the first characters of an IETF Language Code, up to the hyphen (if any), are all lowercase, and those following the hyphen (if any) are all uppercase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// The mandatory first address line element.
    pub address_lines: Vec<String>,
    /// Optional, additional structured version of an external house number, or range of numbers, contained in one of the address line elements. This could be a numeral, a letter or code made up of mixed characters (e.g. 221B).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_number: Option<String>,
    /// Optional, additional structured version of an internal location number, or range of numbers, contained in one of the address line elements.This could be a numeral, a letter or code made up of mixed characters (e.g. 13) of a floor, suite or apartment within a building.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_number_within_building: Option<String>,
    /// Optional free text address line to hold content from other address lines containing explicit routing information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_routing: Option<String>,
    /// One to three optional additional address line elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_address_line: Option<Vec<String>>,
    /// The mandatory name of the city.
    pub city: String,
    /// The (optional) 4- to 6-character ISO 3166-2 region code of the region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The 2-character ISO 3166-1 country code of the country.
    pub country: String,
    /// The (optional) postal code of this address as specified by the local postal service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
}

/// Represents an address with various details.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OtherAddress {
    /// The type of address represented by this `OtherAddress` instance.
    pub field_type: String,
    /// The language in which all of the string-valued components of this address are expressed. An IETF Language Code conforming to the latest RFC from IETF BCP 47. Note that the first characters of an IETF Language Code, up to the hyphen (if any), are all lowercase, and those following the hyphen (if any) are all uppercase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// The mandatory first address line element.
    pub address_lines: Vec<String>,
    /// Optional, additional structured version of an external house number, or range of numbers, contained in one of the address line elements. This could be a numeral, a letter or code made up of mixed characters (e.g. 221B).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_number: Option<String>,
    /// Optional, additional structured version of an internal location number, or range of numbers, contained in one of the address line elements.This could be a numeral, a letter or code made up of mixed characters (e.g. 13) of a floor, suite or apartment within a building.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_number_within_building: Option<String>,
    /// Optional free text address line to hold content from other address lines containing explicit routing information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_routing: Option<String>,
    /// One to three optional additional address line elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_address_line: Option<Vec<String>>,
    /// The mandatory name of the city.
    pub city: String,
    /// The (optional) 4- to 6-character ISO 3166-2 region code of the region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The 2-character ISO 3166-1 country code of the country.
    pub country: String,
    /// The (optional) postal code of this address as specified by the local postal service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// The type of address represented by this `OtherAddress` instance.
    #[serde(rename = "type")]
    pub address_type: OtherAddressType,
}

/// Represents the registration authority details of an entity.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistrationAuthority {
    /// The reference code of the registration authority, taken from the Registration Authorities Code List maintained by GLEIF.
    pub id: String,
    /// An optional legacy / historical reference code of a registration authority which is not yet entered in the Registration Authorities Code List (RA list) maintained by GLEIF, or the designation of an interim register until such time as an entry from RA list can be delivered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<String>,
    /// The identifier of the entity at the indicated registration authority.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
}

/// Represents the legal form of an entity.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LegalForm {
    /// A current code from the GLEIF-maintained list SHALL be used (ISO 20275). Values of the `LegalFormEnum` code list are maintained by ISO / GLEIF through the Entity Legal Form (ELF), available from <http://www.gleif.org>.
    pub id: String,
    /// An optional legacy code or textual description for the legal entity's legal form, used until a current code from the GLEIF-maintained list can be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<String>,
}

/// Represents an associated entity with optional LEI and name.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssociatedEntity {
    /// The LEI of an entity associated with the LEI of this registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lei: Option<String>,
    /// The name of an entity associated with the LEI of this registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The type of association represented by this `AssociatedEntity` instance.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub association_type: Option<AssociatedEntityType>,
}

/// Represents the expiration details of an entity.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Expiration {
    /// The date that the legal entity ceased to operate, whether due to dissolution, merger or acquisition. (Deprecated as of LEI-CDF 3.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<DateTime<Utc>>,
    /// The reason that a legal entity ceased to exist and/or operate. (Deprecated as of LEI-CDF 3.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<ExpirationReason>,
}

/// Represents the successor entity details.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SuccessorEntity {
    /// The LEI of the successor entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lei: Option<String>,
    /// The name of the successor entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Represents an event group with type and events.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventGroup {
    /// Describes whether or not this legal entity event is part of a group of multiple or a complex event.
    pub group_type: GroupType,
    /// The events in the group.
    pub events: Vec<Event>,
}

/// Container for a single event specified by the ROC in the life of a legal entity that would impact the Reference Data in the Global LEI System.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    /// The type of legal entity event.
    #[serde(rename = "type")]
    pub event_type: EventType,
    /// The date when the legal entity event becomes legally effective.
    pub effective_date: DateTime<Utc>,
    /// The date when the legal entity event was recorded in the Global LEI System..
    pub recorded_date: DateTime<Utc>,
    /// Type of source document(s) used for validating the legal entity event.
    pub validation_documents: CorroborationDocuments,
    /// A reference to a specific document or other source used as the basis of validation for this legal entity event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_reference: Option<String>,
    /// A list of LEI Record Set elements that are expected to be updated in the LEI Record Set as a result of a legal entity event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affected_fields: Option<Vec<AffectedFields>>,
    /// Describes the status of the legal entity event with regard to event timeline.
    pub status: EventStatus,
}

/// Represents an affected field in an event.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AffectedFields {
    /// A single element and its value that is expected to be updated in the LEI Record Set as a result of a legal entity event.
    pub value: String,
    /// The full xpath to a single element that is expected to be updated in the LEI Record Set as a result of a legal entity event.
    pub xpath: String,
}

/// Represents the registration details of an entity.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Registration {
    /// The date of the first LEI assignment, being the date of publication of the identifier and its supporting data record as represented in ISO 8601.
    pub initial_registration_date: DateTime<Utc>,
    /// Date/time the LEI record was most recently updated.
    pub last_update_date: DateTime<Utc>,
    /// The status of the legal entity's LEI registration with the `ManagingLOU`.
    pub status: RegistrationStatus,
    /// The next date by which the LEI registration should be renewed and re-certified by the legal entity.
    pub next_renewal_date: DateTime<Utc>,
    /// The LEI of the LOU that is responsible for administering this LEI registration.
    pub managing_lou: String,
    /// The level of validation of the reference data provided by the registrant.
    pub corroboration_level: CorroborationLevel,
    /// Information about the (primary) official authority in the legal entity's jurisdiction of legal registration, for example a business registry, and the corresponding identification of the legal entity by that official source used to validate the legal entity reference data.
    pub validated_at: ValidationAuthority,
    /// The identifier of the entity at the indicated registration authority.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validated_as: Option<String>,
    /// An optional list of additional registration authorities used by the LEI Issuer to validate the entity data.
    pub other_validation_authorities: Vec<OtherValidationAuthority>,
}

/// Represents the validation authority used by the LOU to validate the entity data.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidationAuthority {
    /// The reference code of the registration authority, taken from the Registration Authorities Code List (RA list) maintained by GLEIF.
    pub id: String,
    /// An optional legacy / historical reference code of a registration authority which is not yet entered in the Registration Authorities Code List (RA list) maintained by GLEIF, or the designation of an interim register until such time as an entry from RA list can be delivered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<String>,
}

/// Represents an additional registration authority used by the LOU to validate the entity data.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OtherValidationAuthority {
    /// Information about the other validation authority.
    pub validated_at: ValidationAuthority,
    /// The identifier of the entity at the indicated registration authority.
    pub validated_as: String,
}

/// Represents the relationships of an entity.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Relationships {
    /// The managing LOU.
    pub managing_lou: RelationshipLinks,
    /// The LEI issuer.
    pub lei_issuer: RelationshipLinks,
    /// The field modifications.
    pub field_modifications: RelationshipLinks,
    /// The direct parent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_parent: Option<RelationshipLinks>,
    /// The ultimate parent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ultimate_parent: Option<RelationshipLinks>,
    /// The head office.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_office: Option<RelationshipLinks>,
    /// The direct children.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_children: Option<RelationshipLinks>,
    /// The ultimate children.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ultimate_children: Option<RelationshipLinks>,
    /// The successor entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successor_entity: Option<RelationshipLinks>,
    /// The successor entities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successor_entities: Option<RelationshipLinks>,
    /// The ISINs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isins: Option<RelationshipLinks>,
    /// The fund manager.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fund_manager: Option<RelationshipLinks>,
    /// The umbrella fund.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub umbrella_fund: Option<RelationshipLinks>,
    /// The managed funds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_funds: Option<RelationshipLinks>,
    /// The branches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branches: Option<RelationshipLinks>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{model::common::GleifApiResponse, test_utils::test_model_files};
    use serde_json;
    use std::path::Path;

    #[test]
    fn test_deserialize_single_lei_records() {
        let dir = Path::new("tests/data/lei_records");
        test_model_files(
            |filename| {
                filename.contains("single_lei_record_")
                    || filename.contains("direct-parent_")
                    || filename.contains("ultimate-parent_")
                    || filename.contains("managing-lou_")
            },
            |data| serde_json::from_str::<GleifApiResponse<LeiRecord>>(data),
            |filename, record| {
                if filename == "851WYGNLUQLFZBSYGB56.json" {
                    assert_eq!(
                        record.data.attributes.lei, "851WYGNLUQLFZBSYGB56",
                        "LEI does not match expected value in {filename}"
                    );
                    assert_eq!(
                        record.data.attributes.entity.legal_name.name,
                        "COMMERZBANK Aktiengesellschaft",
                        "Legal name does not match expected value in {filename}"
                    );
                    assert_eq!(
                        record.data.attributes.registration.status.to_string(),
                        "ISSUED",
                        "Registration status does not match expected value in {filename}"
                    );
                } else {
                    assert!(
                        !record.data.attributes.lei.is_empty(),
                        "LEI should not be empty in {filename}"
                    );
                }
            },
            dir,
        );
    }

    #[test]
    fn test_deserialize_multi_lei_records() {
        let dir = Path::new("tests/data/lei_records");
        test_model_files(
            |filename| {
                filename.contains("multi_lei_records")
                    || filename.contains("direct-children_")
                    || filename.contains("ultimate-children_")
            },
            |data| serde_json::from_str::<GleifApiResponse<Vec<LeiRecord>>>(data),
            |filename, list| {
                // An empty list is valid for some relationships (e.g., ultimate children/parents)
                if list.data.is_empty() {
                    return;
                }
                assert!(
                    !list.data[0].attributes.lei.is_empty(),
                    "LEI should not be empty in first record of {filename}"
                );
            },
            dir,
        );
    }
}
