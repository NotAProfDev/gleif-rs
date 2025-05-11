use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct RegistrationAgent {
    pub id: String,
    pub name: String,
    pub country: String,
}
