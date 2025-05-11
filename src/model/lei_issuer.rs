use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct LeiIssuer {
    pub id: String,
    pub name: String,
    pub country: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct LeiIssuerJurisdictions {
    pub id: String,
    pub name: String,
    pub country: String,
}
