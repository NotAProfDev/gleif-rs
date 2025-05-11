use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Jurisdiction {
    pub id: String,
    pub name: String,
    pub country: String,
}
