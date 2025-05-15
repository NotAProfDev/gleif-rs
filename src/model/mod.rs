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
pub type AutoCompletionList = common::GleifApiResponse<Vec<auto_completion::AutoCompletion>>;
pub type Country = common::GleifApiResponse<country::Country>;
pub type CountryList = common::GleifApiResponse<Vec<country::Country>>;
pub type EntityLegalForm = common::GleifApiResponse<entity_legal_form::EntityLegalForm>;
pub type EntityLegalFormList = common::GleifApiResponse<Vec<entity_legal_form::EntityLegalForm>>;
pub type Field = common::GleifApiResponse<field::Field>;
pub type FieldList = common::GleifApiResponse<Vec<field::Field>>;
pub type FieldModificationList =
    common::GleifApiResponse<Vec<field_modification::FieldModification>>;
pub type FuzzyCompletionList = common::GleifApiResponse<Vec<fuzzy_completion::FuzzyCompletion>>;
pub type IsinList = common::GleifApiResponse<Vec<isin::Isin>>;
pub type Jurisdiction = common::GleifApiResponse<jurisdiction::Jurisdiction>;
pub type JurisdictionList = common::GleifApiResponse<Vec<jurisdiction::Jurisdiction>>;
pub type LeiIssuer = common::GleifApiResponse<lei_issuer::LeiIssuer>;
pub type LeiIssuerList = common::GleifApiResponse<Vec<lei_issuer::LeiIssuer>>;
pub type LeiIssuerJurisdictionList =
    common::GleifApiResponse<Vec<lei_issuer::LeiIssuerJurisdiction>>;
pub type LeiRecord = common::GleifApiResponse<lei_record::LeiRecord>;
pub type LeiRecordList = common::GleifApiResponse<Vec<lei_record::LeiRecord>>;
pub type OfficialOrganizationalRole =
    common::GleifApiResponse<official_organizational_role::OfficialOrganizationalRole>;
pub type OfficialOrganizationalRoleList =
    common::GleifApiResponse<Vec<official_organizational_role::OfficialOrganizationalRole>>;
pub type Region = common::GleifApiResponse<region::Region>;
pub type RegionList = common::GleifApiResponse<Vec<region::Region>>;
pub type RegistrationAgent = common::GleifApiResponse<registration_agent::RegistrationAgent>;
pub type RegistrationAgentList =
    common::GleifApiResponse<Vec<registration_agent::RegistrationAgent>>;
pub type RegistrationAuthority =
    common::GleifApiResponse<registration_authority::RegistrationAuthority>;
pub type RegistrationAuthorityList =
    common::GleifApiResponse<Vec<registration_authority::RegistrationAuthority>>;
pub type RelationshipRecord = common::GleifApiResponse<relationship_record::RelationshipRecord>;
pub type RelationshipRecordList =
    common::GleifApiResponse<Vec<relationship_record::RelationshipRecord>>;
pub type ReportingException = common::GleifApiResponse<reporting_exception::ReportingException>;
pub type VLeiIssuer = common::GleifApiResponse<vlei_issuer::VLeiIssuer>;
pub type VLeiIssuerList = common::GleifApiResponse<Vec<vlei_issuer::VLeiIssuer>>;
