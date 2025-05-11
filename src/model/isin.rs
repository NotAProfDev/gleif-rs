// ISIN resource model for GLEIF API
use serde::Deserialize;
// TODO: test this model
// TODO: module documentation

/// ISIN resource object for GLEIF API.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Isin {
    /// The type of the data.
    #[serde(rename = "type")]
    pub data_type: String,
    /// The unique identifier of the data.
    pub id: String,
    /// ISIN attributes.
    pub attributes: IsinAttributes,
}

/// Attributes for an ISIN resource.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct IsinAttributes {
    /// A Legal Entity Identifier (LEI) code, in the format specified by ISO 17442.
    pub lei: String,
    /// The ISIN code.
    pub isin: String,
}
