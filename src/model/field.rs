use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Field {
    pub id: String,
    pub name: String,
    pub country: String,
}
