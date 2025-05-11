use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct VLeiIssuer {
    id: String,
    name: String,
    jurisdictions: Vec<String>,
}
