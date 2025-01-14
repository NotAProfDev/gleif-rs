//! This module defines the data structures and their deserialization logic
//! for handling various entity-related information. The main data structure
//! is `Data`, which contains attributes and relationships of an entity.
//! The module also includes several supporting structures and enumerations
//! to represent different aspects of an entity, such as its name, address,
//! registration details, and event information.

use chrono::{DateTime, Utc};
use serde::Deserialize;

/// Represents the main data structure.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Data {
    /// The type of the data.
    #[serde(rename = "type")]
    pub data_type: String,
    /// The unique identifier of the data.
    pub id: String,
    /// The attributes of the data.
    pub attributes: Attributes,
    /// The relationships of the data.
    pub relationships: Relationships,
}

/// Represents the attributes of the data.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    /// The Legal Entity Identifier (LEI).
    pub lei: String,
    /// The entity details.
    pub entity: Entity,
    /// The registration details.
    pub registration: Registration,
    /// The Bank Identifier Codes (BIC).
    pub bic: Option<Vec<String>>,
    /// The Market Identifier Codes (MIC).
    pub mic: Option<Vec<String>>,
    /// The Open Corporates ID (OCID).
    pub ocid: Option<String>,
    /// The S&P Global identifiers.
    pub spglobal: Option<Vec<String>>,
    /// The conformity flag.
    pub conformity_flag: Option<ConformityFlag>,
}

/// Represents an entity with various details.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    /// The legal name of the entity.
    pub legal_name: Name,
    /// Other names of the entity.
    pub other_names: Vec<Name>,
    /// Transliterated other names of the entity.
    pub transliterated_other_names: Vec<Name>,
    /// The legal address of the entity.
    pub legal_address: Address,
    /// The headquarters address of the entity.
    pub headquarters_address: Address,
    /// The registration details of the entity.
    pub registered_at: RegisteredAt,
    /// The registered name of the entity.
    pub registered_as: Option<String>,
    /// The jurisdiction of the entity.
    pub jurisdiction: String,
    /// The category of the entity.
    pub category: Category,
    /// The legal form of the entity.
    pub legal_form: LegalForm,
    /// The associated entity details.
    pub associated_entity: GeneralEntity,
    /// The status of the entity.
    pub status: EntityStatus,
    /// The expiration details of the entity.
    pub expiration: Expiration,
    /// The successor entity details.
    pub successor_entity: GeneralEntity,
    /// The successor entities details.
    pub successor_entities: Vec<GeneralEntity>,
    /// The creation date of the entity.
    pub creation_date: Option<DateTime<Utc>>,
    /// The sub-category of the entity.
    pub sub_category: Option<SubCategory>,
    /// Other addresses of the entity.
    pub other_addresses: Vec<Address>,
    /// The event groups associated with the entity.
    pub event_groups: Vec<String>,
}

/// Represents a name with optional language and type.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Name {
    /// The name.
    pub name: String,
    /// The language of the name.
    pub language: Option<String>,
    /// The type of the name.
    #[serde(rename = "type")]
    pub name_type: Option<String>,
}

/// Represents an address with various details.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    /// The language of the address.
    pub language: Option<String>,
    /// The address lines.
    pub address_lines: Vec<String>,
    /// The address number.
    pub address_number: Option<String>,
    /// The address number within the building.
    pub address_number_within_building: Option<String>,
    /// The mail routing information.
    pub mail_routing: Option<String>,
    /// The city of the address.
    pub city: String,
    /// The region of the address.
    pub region: Option<String>,
    /// The country of the address.
    pub country: String,
    /// The postal code of the address.
    pub postal_code: Option<String>,
    /// The type of the address.
    #[serde(rename = "type")]
    pub address_type: Option<String>,
}

/// Represents the registration details of an entity.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RegisteredAt {
    /// The ID of the registration.
    pub id: Option<String>,
    /// Other registration details.
    pub other: Option<String>,
}

/// Represents the category of an entity.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Category {
    Branch,
    Fund,
    SoleProprietor,
    General,
    ResidentGovernmentEntity,
    InternationalOrganization,
}

/// Represents the sub-category of an entity.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(clippy::enum_variant_names)]
pub enum SubCategory {
    CentralGovernment,
    StateGovernment,
    LocalGovernment,
    SocialGovernment,
}

/// Represents an event group with type and events.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EventGroup {
    /// The type of the group.
    pub group_type: GroupType,
    /// The events in the group.
    pub events: Vec<Event>,
}

/// Represents an event with various details.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    /// The validation documents for the event.
    pub validation_documents: ValidationDocuments,
    /// The effective date of the event.
    pub effective_date: String,
    /// The recorded date of the event.
    pub recorded_date: String,
    /// The type of the event.
    #[serde(rename = "type")]
    pub event_type: EventType,
    /// The status of the event.
    pub status: EventStatus,
    /// The affected fields in the event.
    pub affected_fields: Option<Vec<AffectedField>>,
    /// The validation reference for the event.
    pub validation_reference: Option<String>,
}

/// Represents an affected field in an event.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct AffectedField {
    /// The value of the affected field.
    pub value: String,
    /// The XPath of the affected field.
    pub xpath: String,
}

/// Represents the type of an event.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EventType {
    ChangeHqAddress,
    ChangeLegalAddress,
    ChangeLegalForm,
    ChangeLegalName,
    Dissolution,
    Liquidation,
    MergersAndAcquisitions,
}

/// Represents the status of an event.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EventStatus {
    Completed,
    InProgress,
    WithdrawnCancelled,
}

/// Represents the validation documents for an event.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ValidationDocuments {
    AccountsFiling,
    OtherOfficialDocuments,
    RegulatoryFiling,
    SupportingDocuments,
}

/// Represents the group type of an event.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GroupType {
    ChangeLegalFormAndName,
    Standalone,
}

/// Represents the legal form of an entity.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct LegalForm {
    /// The ID of the legal form.
    pub id: String,
    /// Other details of the legal form.
    pub other: Option<String>,
}

/// Represents a general entity with optional LEI and name.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GeneralEntity {
    /// The Legal Entity Identifier (LEI).
    pub lei: Option<String>,
    /// The name of the entity.
    pub name: Option<String>,
}

/// Represents the status of an entity.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum EntityStatus {
    Active,
    Inactive,
    Null,
}

/// Represents the expiration details of an entity.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Expiration {
    /// The expiration date.
    pub date: Option<DateTime<Utc>>,
    /// The reason for expiration.
    pub reason: Option<String>,
}

/// Represents the conformity flag of an entity.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ConformityFlag {
    Conforming,
    NonConforming,
    NotApplicable,
}

/// Represents the registration details of an entity.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Registration {
    /// The initial registration date.
    pub initial_registration_date: DateTime<Utc>,
    /// The last update date.
    pub last_update_date: DateTime<Utc>,
    /// The status of the registration.
    pub status: RegistrationStatus,
    /// The next renewal date.
    pub next_renewal_date: DateTime<Utc>,
    /// The managing LOU.
    pub managing_lou: String,
    /// The corroboration level.
    pub corroboration_level: CorroborationLevel,
    /// The validated legal form.
    pub validated_at: LegalForm,
    /// The validated name.
    pub validated_as: Option<String>,
    /// Other validation authorities.
    pub other_validation_authorities: Vec<OtherValidationAuthority>,
}

/// Represents the status of a registration.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RegistrationStatus {
    Issued,
    Lapsed,
    Annulled,
    PendingTransfer,
    PendingArchival,
    Duplicate,
    Retired,
    Merged,
}

/// Represents the corroboration level of a registration.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CorroborationLevel {
    EntitySuppliedOnly,
    PartiallyCorroborated,
    FullyCorroborated,
}

/// Represents other validation authorities for a registration.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OtherValidationAuthority {
    /// The validated legal form.
    pub validated_at: LegalForm,
    /// The validated name.
    pub validated_as: String,
}

/// Represents the relationships of an entity.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct Relationships {
    /// The managing LOU.
    pub managing_lou: RelationshipLinks,
    /// The LEI issuer.
    pub lei_issuer: RelationshipLinks,
    /// The field modifications.
    pub field_modifications: RelationshipLinks,
    /// The direct parent.
    pub direct_parent: Option<RelationshipLinks>,
    /// The ultimate parent.
    pub ultimate_parent: Option<RelationshipLinks>,
    /// The head office.
    pub head_office: Option<RelationshipLinks>,
    /// The direct children.
    pub direct_children: Option<RelationshipLinks>,
    /// The ultimate children.
    pub ultimate_children: Option<RelationshipLinks>,
    /// The successor entity.
    pub successor_entity: Option<RelationshipLinks>,
    /// The successor entities.
    pub successor_entities: Option<RelationshipLinks>,
    /// The ISINs.
    pub isins: Option<RelationshipLinks>,
    /// The fund manager.
    pub fund_manager: Option<RelationshipLinks>,
    /// The umbrella fund.
    pub umbrella_fund: Option<RelationshipLinks>,
    /// The managed funds.
    pub managed_funds: Option<RelationshipLinks>,
    /// The branches.
    pub branches: Option<RelationshipLinks>,
}

/// Represents the links for a relationship.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct RelationshipLinks {
    /// The links data.
    pub links: RelationshipData,
}

/// Represents the data for a relationship link.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct RelationshipData {
    /// The reporting exception.
    pub reporting_exception: Option<String>,
    /// The relationship record.
    pub relationship_record: Option<String>,
    /// The related entity.
    pub related: Option<String>,
    /// The LEI record.
    pub lei_record: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_data_deserialization() {
        let data = json!({
            "type": "exampleType",
            "id": "exampleId",
            "attributes": {
                "lei": "exampleLEI",
                "entity": {
                    "legalName": {
                        "name": "exampleName",
                        "language": "en",
                        "type": "official"
                    },
                    "otherNames": [],
                    "transliteratedOtherNames": [],
                    "legalAddress": {
                        "language": "en",
                        "addressLines": ["123 Example St"],
                        "city": "Example City",
                        "country": "Example Country"
                    },
                    "headquartersAddress": {
                        "language": "en",
                        "addressLines": ["123 Example St"],
                        "city": "Example City",
                        "country": "Example Country"
                    },
                    "registeredAt": {
                        "id": "exampleId",
                        "other": "exampleOther"
                    },
                    "registeredAs": "exampleRegisteredAs",
                    "jurisdiction": "exampleJurisdiction",
                    "category": "GENERAL",
                    "legalForm": {
                        "id": "exampleId",
                        "other": "exampleOther"
                    },
                    "associatedEntity": {
                        "lei": "exampleLEI",
                        "name": "exampleName"
                    },
                    "status": "ACTIVE",
                    "expiration": {
                        "date": "2023-10-10T00:00:00Z",
                        "reason": "exampleReason"
                    },
                    "successorEntity": {
                        "lei": null,
                        "name": null
                    },
                    "successorEntities": [],
                    "creationDate": "2023-10-10T00:00:00Z",
                    "subCategory": "CENTRAL_GOVERNMENT",
                    "otherAddresses": [],
                    "eventGroups": []
                },
                "registration": {
                    "initialRegistrationDate": "2023-10-10T00:00:00Z",
                    "lastUpdateDate": "2023-10-10T00:00:00Z",
                    "status": "ISSUED",
                    "nextRenewalDate": "2023-10-10T00:00:00Z",
                    "managingLou": "exampleLOU",
                    "corroborationLevel": "FULLY_CORROBORATED",
                    "validatedAt": {
                        "id": "exampleId",
                        "other": "exampleOther"
                    },
                    "validatedAs": "exampleValidatedAs",
                    "otherValidationAuthorities": [
                        {
                            "validatedAt": {
                                "id": "exampleId"
                            },
                            "validatedAs": "exampleValidatedAs"
                        }
                    ]
                },
                "bic": ["exampleBIC"],
                "mic": ["exampleMIC"],
                "ocid": "exampleOCID",
                "spglobal": ["exampleSPGlobal"],
                "conformityFlag": "CONFORMING"
            },
            "relationships": {
                "managing-lou": {
                    "links": {
                        "reporting-exception": "exampleException",
                        "relationship-record": "exampleRecord",
                        "related": "exampleRelated",
                        "lei-record": "exampleLEIRecord"
                    }
                },
                "lei-issuer": {
                    "links": {
                        "reporting-exception": "exampleException",
                        "relationship-record": "exampleRecord",
                        "related": "exampleRelated",
                        "lei-record": "exampleLEIRecord"
                    }
                },
                "field-modifications": {
                    "links": {
                        "reporting-exception": "exampleException",
                        "relationship-record": "exampleRecord",
                        "related": "exampleRelated",
                        "lei-record": "exampleLEIRecord"
                    }
                },
                "direct-parent": null,
                "ultimate-parent": null,
                "head-office": null,
                "direct-children": null,
                "ultimate-children": null,
                "successor-entity": null,
                "successor-entities": null,
                "isins": null,
                "fund-manager": null,
                "umbrella-fund": null,
                "managed-funds": null,
                "branches": null
            }
        });

        let data: Data = serde_json::from_value(data).unwrap();

        assert_eq!(data.data_type, "exampleType");
        assert_eq!(data.id, "exampleId");
        assert_eq!(data.attributes.lei, "exampleLEI");
        assert_eq!(data.attributes.entity.legal_name.name, "exampleName");
        assert_eq!(
            data.attributes.entity.legal_name.language,
            Some("en".to_string())
        );
        assert_eq!(
            data.attributes.entity.legal_name.name_type,
            Some("official".to_string())
        );
        assert_eq!(data.attributes.entity.legal_address.city, "Example City");
        assert_eq!(
            data.attributes.entity.legal_address.country,
            "Example Country"
        );
        assert_eq!(
            data.attributes
                .registration
                .initial_registration_date
                .to_string(),
            "2023-10-10 00:00:00 UTC"
        );
        assert_eq!(
            data.attributes.registration.status,
            RegistrationStatus::Issued
        );
        assert_eq!(
            data.attributes.conformity_flag,
            Some(ConformityFlag::Conforming)
        );
        assert_eq!(
            data.relationships.managing_lou.links.reporting_exception,
            Some("exampleException".to_string())
        );
    }
}
