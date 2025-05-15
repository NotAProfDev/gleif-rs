//! Enum definitions for the GLEIF API.
//!
//! This module contains enumerations used to deserialize and represent various attributes and statuses
//! across multiple GLEIF API endpoints. These enums are designed to match the JSON structure returned
//! by the API and provide a strongly-typed interface for working with GLEIF data.
//!
//! The enums in this module are categorized based on their usage in:
//! - Level 1 (Who is Who): Attributes related to entity names, addresses, categories, and statuses.
//! - Level 2 (Who Owns Whom): Attributes related to relationships between legal entities.
//! - Level 1 and Level 2: Attributes that are relevant to both levels, such as registration statuses and corroboration levels.
//! - Other general attributes used across the GLEIF API.
//!
//! For endpoint-specific usage and client methods, refer to the corresponding modules in [`crate::endpoint`].

use serde::Deserialize;

// -- Enums used by Level 1 (who is who) --

/// Represents the type of the name.
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EntityNameType {
    /// Registered name of the entity in an alternative language in the legal jurisdiction in which the entity is registered.
    AlternativeLanguageLegalName,
    /// A primary legal name previously used by this entity.
    PreviousLegalName,
    /// A "trading as", "brand name" or "operating under" name currently used by this entity in addition to, but not replacing, the (primary) legal, official registered name.
    TradingOrOperatingName,
}

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

/// Represents the type of the transliterated name.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransliteratedNameType {
    /// Legal name of the entity transliterated to ASCII characters, provided by the entity for this purpose.
    PreferredAsciiTransliteratedLegalName,
    /// Legal name of the entity transliterated to ASCII characters, auto-transliterated by the managing LOU.
    AutoAsciiTransliteratedLegalName,
}

/// Represents the type of the other address.
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OtherAddressType {
    /// Registered address of the entity in the legal jurisdiction, in an alternative language used in the legal jurisdiction.
    AlternativeLanguageLegalAddress,
    /// Address of the headquarters of the entity, in an alternative language used in the legal jurisdiction.
    AlternativeLanguageHeadquartersAddress,
}

/// Represents the type of the transliterated address.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransliteratedAdressType {
    /// Registered address of the entity in the legal jurisdiction, transliterated to ASCII characters, auto-transliterated by the managing LOU.
    AutoAsciiTransliteratedLegalAddress,
    /// Address of the headquarters of the entity, transliterated to ASCII characters, auto-transliterated by the managing LOU.
    AutoAsciiTransliteratedHeadquartersAddress,
    /// Registered address of the entity in the legal jurisdiction, transliterated to ASCII characters, provided by the entity for this purpose.
    PreferredAsciiTransliteratedLegalAddress,
    /// Address of the headquarters of the entity, transliterated to ASCII characters, provided by the entity for this purpose.
    PreferredAsciiTransliteratedHeadquartersAddress,
}

/// Represents the sub-category of an entity.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EntitySubCategory {
    /// The legal entity is a central government subsector which consists of the institutional units plus non-profit institution (NPI) controlled by the central government institutions.
    CentralGovernment,
    /// The legal entity is a state government subsector which consists of the institutional units that exercise some government functions at a level below that of the central government and above that of the government institutional units existing at a local level.
    StateGovernment,
    /// The legal entity is a local government subsector which consists of local governments that are separate institutional units.
    LocalGovernment,
    /// The legal entity is a social security scheme which covers a wide variety of programmes, providing benefits in cash or in kind for old age, invalidity or death, survivors, sickness and maternity, work injury, unemployment, family allowance, health care, etc.
    SocialSecurity,
}

/// Represents the type of the associated entity.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AssociatedEntityType {
    /// The legal entity is a fund, and the associated entity is the manager of the fund.
    FundFamily,
}

/// Represents the status of an entity.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum EntityStatus {
    /// As of the last report or update, the legal entity reported that it was legally registered and operating.
    Active,
    /// It has been determined that the entity that was assigned the LEI is no longer legally registered and/or operating, as indicated by a legal entity event Reason.
    Inactive,
    /// The entity status is not applicable.
    Null,
}

/// Represents the reason for the expiration of an entity.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExpirationReason {
    /// The entity ceased to operate.
    Dissolved,
    /// The entity was acquired or merged with another entity.
    CorporateAction,
    /// The reason for expiry is neither of `DISSOLVED` nor `CORPORATE_ACTION`.
    Other,
}

/// Represents the group type of an event.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GroupType {
    /// This event is part of a reverse takeover event.
    ReverseTakeover,
    /// Single/standalone legal entity event with no combination of multiple events.
    Standalone,
    /// Legal entity event including change of the legal name triggered by change of the legal form.
    ChangeLegalFormAndName,
    /// Legal entity event including change of the legal entity status triggered by change of the legal form.
    ComplexChangeLegalForm,
}

/// Represents the type of an event.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EventType {
    /// Change in the legal name of the legal entity.
    ChangeLegalName,
    /// Change in the trade- or doing business name of the legal entity.
    ChangeOtherNames,
    /// Change in the legal address of the legal entity.
    ChangeLegalAddress,
    /// Change in the headquarters address of the legal entity.
    ChangeHqAddress,
    /// Change in the legal form of the legal entity.
    ChangeLegalForm,
    /// Distribution of securities issued by another legal entity. The distributed securities may either be of a newly created or of an existing legal entity. For example, spin-off, demerger, unbundling, divestment.
    Demerger,
    /// A special case of demerger where the shareholders of the original entity are compensated for the value loss of the original entity via shares of the new entity or via dividend.
    Spinoff,
    /// Absorption is a form of merger where there is a combination of two or more companies into an 'existing company'. In the case of absorption, only one company survives and all others lose their identity.
    Absorption,
    /// The acquiring legal entity purchases an international branch entity.
    AcquisitionBranch,
    /// The transfer of all of the assets and liabilities of an International Branch to the new subsidiary entity in exchange for the transfer of securities representing the capital of the subsidiary entity receiving the transfer.
    TransformationBranchToSubsidiary,
    /// The transfer of all of the assets and liabilities of a subsidiary to an International Branch entity in exchange for the transfer of securities representing the capital of the International Branch entity receiving the transfer.
    TransformationSubsidiaryToBranch,
    /// Change in legal form from a Fund legal entity structure with one or more than one sub-funds/compartments to a Fund legal entity structure without sub-funds/compartments.
    TransformationUmbrellaToStandalone,
    /// A break-up is a corporate action in which a single company splits into two or more independent, separately-run companies. Regulators also can mandate break-ups of companies for anti-trust reasons.
    Breakup,
    /// The consolidation of Legal Entities or assets through various types of financial transactions, including mergers, acquisitions, consolidations, tender offers, purchase of assets and management acquisitions.
    MergersAndAcquisitions,
    /// Legal status of a legal entity unable to pay creditors. Bankruptcy usually involves a formal court ruling. Securities may become valueless (event completed).
    Bankruptcy,
    /// Distribution of cash, assets or both of a legal entity. Debt may be paid in order of priority based on preferred claims to assets specified by the security (event completed).
    Liquidation,
    /// A procedure that allows a legal entity to settle debts by paying only a proportion of the amount that it owes to creditors or to come to some other arrangement with its creditors over the payment of its debts (event completed).
    VoluntaryArrangement,
    /// The entry of a decree or order by a court or agency or supervisory authority having jurisdiction in the premises the appointment of a trustee-in-bankruptcy or similar official for such party in any insolvency, readjustment of debt, marshalling of assets and liabilities, or similar proceedings, or for the winding up or liquidation of their respective affairs (event completed).
    Insolvency,
    /// (i) A voluntary termination of operations, (ii) a general assignment for the benefit of the legal entity’s creditors or (iii) any other liquidation, dissolution or winding up of the legal entity (excluding a Liquidity Event), whether voluntary or involuntary (event completed).
    Dissolution,
}

/// Represents the status of an event.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EventStatus {
    /// The legal entity event activity and processing is in progress.
    InProgress,
    /// The legal entity event as announced will not occur.
    WithdrawnCancelled,
    /// The legal entity event activity and processing have been completed as of the `LegalEntityEventEffectiveDate`.
    Completed,
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

// -- Enums used by Level 2 (who owns whom) --

/// A unique code designating the specific category of a directional relationship between two legal entities.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RelationshipType {
    /// The `StartNode` is directly consolidated by the `EndNode`. The `StartNode` "child" entity has its accounts fully consolidated by the `EndNode` "parent" entity, in the sense given by the accounting standard(s) specified in `RelationshipQualifiers`; the `EndNode` entity is the closest fully consolidating parent to the `StartNode` entity in any applicable hierarchical ownership structure.
    IsDirectlyConsolidatedBy,
    /// The `StartNode` is ultimately consolidated by `EndNode`. The `StartNode` "child" entity has its accounts fully consolidated by the `EndNode` "parent" entity, in the sense given by the accounting standard(s) specified in `RelationshipQualifiers`; the `EndNode` entity is the most distant fully consolidating parent from the `StartNode` entity in any applicable hierarchical ownership structure.
    IsUltimatelyConsolidatedBy,
    /// `StartNode` is an international branch of the legal entity designated by `EndNode` (in jurisdiction country of `StartNode`). The `EndNode` is the Head Office and SHALL be an LEI.
    IsInternationalBranchOf,
    /// `StartNode` is a fund managed by a main management entity. The `EndNode` is legally responsible for the constitution and operation of the fund.
    #[serde(rename = " IS_FUND-MANAGED_BY")]
    IsFundManagedBy,
    /// `StartNode` is a sub-fund to an umbrella fund. The `EndNode` is a legal entity with one or more than one sub-funds/compartments where each sub-fund/compartment has its own investment objectives, separate investment policies and strategies, segregation of assets, separate investors and which has segregated liability between sub-funds/compartments.
    IsSubfundOf,
    /// `StartNode` is a Feeder Fund, that is (almost) exclusively invested in a single other fund. The `EndNode` is the Master Fund that has identical investment strategies.
    IsFeederTo,
}

/// The particular type of period, for example, the duration of the relationship itself, the filing or validity period of any documents demonstrating the relationship, or the accounting period they refer to.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RelationshipPeriodType {
    /// The dates in this instance of `RelationshipPeriod` indicate the accounting period covered by the most recent validation documents for this relationship.
    AccountingPeriod,
    /// The dates in this instance of `RelationshipPeriod` indicate the duration of validity of the relationship itself, as distinct from any administrative or reporting aspects.
    RelationshipPeriod,
    /// The dates in this instance of `RelationshipPeriod` indicate the validity period of a regulatory filing, accounting document, or other document demonstrating the relationship's validity.
    DocumentFilingPeriod,
}

/// Represents the status of a relationship.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum RelationshipStatus {
    /// As of the last report or update, the reporting legal entity reported that it is legally registered and/or operating, AND that the relationship detailed in this `RelationshipRecord` is still valid.
    Active,
    /// It has been determined that the relationship ended, e.g. because entity that reported this relationship is no longer legally registered and/or operating; or the relationship is no longer valid for other reasons.
    Inactive,
    /// The relationship status is not applicable.
    Null,
}

/// Designates the optional list of additional qualitative attributes that help to categorize the relationship.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum QualifierDimension {
    /// The accounting standard applied to determine the definition of e.g. ultimate or direct accounting consolidating parent for the relationship detailed in this `RelationshipRecord`. The relevant accounting standard is that applicable to the `EndNode` (the "parent" entity).
    AccountingStandard,
}

/// Specifies the additional qualitative attributes that help to categorize the relationship.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum QualifierCategoryType {
    /// United States-Generally Accepted Accounting Principles.
    UsGaap,
    /// International Financial Reporting Standard (developed by the International Accounting Standards Board – IASB see [http://www.ifrs.org](http://www.ifrs.org)).
    Ifrs,
    /// A financial reporting (accounting) standard not otherwise listed in the latest version of the relationship data file format.
    OtherAccountingStandard,
    /// Used for entities consolidated under the International Public Sector Accounting Standard (IPSAS 35) or National Government or Federal Government accounting standards specifically developed for Government entities in their state or local jurisdiction.
    GovernmentAccountingStandard,
}

/// Specifies the method of measurement (or set of rules) used to quantitatively categorize the relationship.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MeasurementMethodType {
    /// Accounting consolidation holds when "[in the] financial statements of a group [...] the assets, liabilities, equity, income, expenses and cash flows of the parent and its subsidiaries are presented as those of a single economic entity (please see [http://www.iasplus.com/en/standards/ias/ias27-2011](http://www.iasplus.com/en/standards/ias/ias27-2011)).
    AccountingConsolidation,
}

/// Specifies the units, where applicable, of a measurement made on a relationship.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum QuantifierUnitsType {
    Percentage,
}

// -- Enums used by Level 1 (who is who) and Level 2 (who owns whom) --

/// Registration Statuses for LEI records in the GLEIF database.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RegistrationStatus {
    /// An application for an LEI or relationship data report that has been submitted and which is being processed and validated.
    PendingValidation,
    /// An LEI Registration that has been validated and issued, and which identifies an entity having a legal entity status of `ACTIVE` as of the Date of First LEI Assignment or the Date of Last Update of the LEI Data Record.
    Issued,
    /// An LEI Registration or relationship data report that has been determined to be a duplicate registration of the same legal entity as another LEI Registration; the `DUPLICATE` status is assigned to the non-surviving registration (i.e. the LEI that should no longer be used).
    Duplicate,
    /// An LEI registration or relationship data report that has not been renewed by the `NextRenewalDate` and is not known by public sources to have a legal entity status of `INACTIVE`.
    Lapsed,
    /// An LEI registration for an entity that has been merged into another legal entity, such that this legal entity no longer exists as an operating entity. (Deprecated as of `LEI-CDF 3.0`)
    Merged,
    /// An LEI registration status or relationship data report for an entity that has been affected by certain legal entity events.
    Retired,
    /// An LEI registration or relationship data report that was determined to be erroneous or invalid after it was issued.
    Annulled,
    /// An LEI registration that was abandoned prior to issuance of an LEI,
    Cancelled,
    /// An LEI registration or relationship data report that has been transferred to a different LOU as the managing LOU.
    Transferred,
    /// An LEI registration or relationship data report that has been requested to be transferred to another LOU. The request is being processed at the sending LOU.
    PendingTransfer,
    /// An LEI registration or relationship data report that is about to be transferred to a different LOU, after which its registration status will revert to a non-pending status.
    PendingArchival,
    /// A relationship data report that has been validated and published, and which is reported by an entity that was an operating legal entity as of the last update.
    Published,
}

/// Represents the validation documents for an event.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CorroborationDocuments {
    /// A consolidated financial (accounting) statement, prepared and submitted to the relevant authority.
    AccountsFiling,
    /// A regulatory filing providing public information on legal entities and/or their relationships.
    RegulatoryFiling,
    /// Other documents supporting the validation of legal entities and/or their relationships.
    SupportingDocuments,
    /// Contract(s) attesting to the validity of egal entities and/or their relationships.
    Contracts,
    /// Other official document(s) attesting to the validity of legal entities and/or their relationships.
    OtherOfficialDocuments,
}

/// Represents the corroboration level of a registration.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CorroborationLevel {
    /// The validation of the reference data provided by the registrant has not yet occurred.
    Pending,
    /// Based on the validation procedures in use by the LOU responsible for the record, the information associated with this record has significant reliance on the information that a submitter provided due to the unavailability of corroborating information.
    EntitySuppliedOnly,
    /// Based on the validation procedures in use by the LOU responsible for the record, the information supplied by the registrant can be partially corroborated by public authoritative sources, while some of the record is dependent upon the information that the registrant submitted, either due to conflicts with authoritative information, or due to data unavailability.
    PartiallyCorroborated,
    /// Based on the validation procedures in use by the LOU responsible for the record, there is sufficient information contained in authoritative public sources to corroborate the information that the submitter has provided for the record.
    FullyCorroborated,
}

// -- Other Enums --

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ELFStatus {
    /// The entity is active.
    Actv,
    /// The entity is inactive.
    Inac,
}
