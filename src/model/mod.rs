//! Model definitions for the GLEIF API.
//!
//! This module contains submodules that define the data structures and enumerations used to interact with
//! various endpoints of the GLEIF API. Each submodule corresponds to a specific endpoint or category of data
//! provided by the API, enabling strongly-typed deserialization and manipulation of GLEIF resources.
//!
//! The submodules include:
//! - `auto_completion`: Models for the auto-completion endpoint.
//! - `common`: Shared data structures used across multiple endpoints.
//! - `country`: Models for country-specific data.
//! - `entity_legal_form`: Models for entity legal forms.
//! - `enums`: Enumerations for attributes and statuses used across the API.
//! - `field`: Models for the field endpoint.
//! - `field_modification`: Models for the field modification endpoint.
//! - `fuzzy_completion`: Models for the fuzzy completion endpoint.
//! - `isin`: Models for ISIN-related data.
//! - `jurisdiction`: Models for jurisdiction-specific data.
//! - `lei_issuer`: Models for LEI issuer data.
//! - `lei_record`: Models for LEI records.
//! - `official_organizational_role`: Models for official organizational roles.
//! - `region`: Models for region-specific data.
//! - `registration_agent`: Models for registration agents.
//! - `registration_authority`: Models for registration authorities.
//! - `relationship_record`: Models for relationship records.
//! - `reporting_exception`: Models for reporting exceptions.
//! - `vlei_issuer`: Models for vLEI issuers.
//!
//! These modules collectively provide a comprehensive representation of the GLEIF API's data model.

pub mod auto_completion;
pub mod common;
pub mod country;
pub mod entity_legal_form;
pub mod enums;
pub mod field;
pub mod field_modification;
pub mod fuzzy_completion;
pub mod isin;
pub mod jurisdiction;
pub mod lei_issuer;
pub mod lei_record;
pub mod official_organizational_role;
pub mod region;
pub mod registration_agent;
pub mod registration_authority;
pub mod relationship_record;
pub mod reporting_exception;
pub mod vlei_issuer;

// Types alias for convenience

/// Type alias for a list of auto-completion responses from the GLEIF API.
pub type AutoCompletionList = common::GleifApiResponse<Vec<auto_completion::AutoCompletion>>;
/// Type alias for a country response from the GLEIF API.
pub type Country = common::GleifApiResponse<country::Country>;
/// Type alias for a list of countries from the GLEIF API.
pub type CountryList = common::GleifApiResponse<Vec<country::Country>>;
/// Type alias for an entity legal form response from the GLEIF API.
pub type EntityLegalForm = common::GleifApiResponse<entity_legal_form::EntityLegalForm>;
/// Type alias for a list of entity legal forms from the GLEIF API.
pub type EntityLegalFormList = common::GleifApiResponse<Vec<entity_legal_form::EntityLegalForm>>;
/// Type alias for a field response from the GLEIF API.
pub type Field = common::GleifApiResponse<field::Field>;
/// Type alias for a list of fields from the GLEIF API.
pub type FieldList = common::GleifApiResponse<Vec<field::Field>>;
/// Type alias for a list of field modifications from the GLEIF API.
pub type FieldModificationList =
    common::GleifApiResponse<Vec<field_modification::FieldModification>>;
/// Type alias for a list of fuzzy completions from the GLEIF API.
pub type FuzzyCompletionList = common::GleifApiResponse<Vec<fuzzy_completion::FuzzyCompletion>>;
/// Type alias for a list of ISINs from the GLEIF API.
pub type IsinList = common::GleifApiResponse<Vec<isin::Isin>>;
/// Type alias for a jurisdiction response from the GLEIF API.
pub type Jurisdiction = common::GleifApiResponse<jurisdiction::Jurisdiction>;
/// Type alias for a list of jurisdictions from the GLEIF API.
pub type JurisdictionList = common::GleifApiResponse<Vec<jurisdiction::Jurisdiction>>;
/// Type alias for a LEI issuer response from the GLEIF API.
pub type LeiIssuer = common::GleifApiResponse<lei_issuer::LeiIssuer>;
/// Type alias for a list of LEI issuers from the GLEIF API.
pub type LeiIssuerList = common::GleifApiResponse<Vec<lei_issuer::LeiIssuer>>;
/// Type alias for a list of LEI issuer jurisdictions from the GLEIF API.
pub type LeiIssuerJurisdictionList =
    common::GleifApiResponse<Vec<lei_issuer::LeiIssuerJurisdiction>>;
/// Type alias for a LEI record response from the GLEIF API.
pub type LeiRecord = common::GleifApiResponse<lei_record::LeiRecord>;
/// Type alias for a list of LEI records from the GLEIF API.
pub type LeiRecordList = common::GleifApiResponse<Vec<lei_record::LeiRecord>>;
/// Type alias for an official organizational role response from the GLEIF API.
pub type OfficialOrganizationalRole =
    common::GleifApiResponse<official_organizational_role::OfficialOrganizationalRole>;
/// Type alias for a list of official organizational roles from the GLEIF API.
pub type OfficialOrganizationalRoleList =
    common::GleifApiResponse<Vec<official_organizational_role::OfficialOrganizationalRole>>;
/// Type alias for a region response from the GLEIF API.
pub type Region = common::GleifApiResponse<region::Region>;
/// Type alias for a list of regions from the GLEIF API.
pub type RegionList = common::GleifApiResponse<Vec<region::Region>>;
/// Type alias for a registration agent response from the GLEIF API.
pub type RegistrationAgent = common::GleifApiResponse<registration_agent::RegistrationAgent>;
/// Type alias for a list of registration agents from the GLEIF API.
pub type RegistrationAgentList =
    common::GleifApiResponse<Vec<registration_agent::RegistrationAgent>>;
/// Type alias for a registration authority response from the GLEIF API.
pub type RegistrationAuthority =
    common::GleifApiResponse<registration_authority::RegistrationAuthority>;
/// Type alias for a list of registration authorities from the GLEIF API.
pub type RegistrationAuthorityList =
    common::GleifApiResponse<Vec<registration_authority::RegistrationAuthority>>;
/// Type alias for a relationship record response from the GLEIF API.
pub type RelationshipRecord = common::GleifApiResponse<relationship_record::RelationshipRecord>;
/// Type alias for a list of relationship records from the GLEIF API.
pub type RelationshipRecordList =
    common::GleifApiResponse<Vec<relationship_record::RelationshipRecord>>;
/// Type alias for a reporting exception response from the GLEIF API.
pub type ReportingException = common::GleifApiResponse<reporting_exception::ReportingException>;
/// Type alias for a vLEI issuer response from the GLEIF API.
pub type VLeiIssuer = common::GleifApiResponse<vlei_issuer::VLeiIssuer>;
/// Type alias for a list of vLEI issuers from the GLEIF API.
pub type VLeiIssuerList = common::GleifApiResponse<Vec<vlei_issuer::VLeiIssuer>>;
