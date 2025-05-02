//! Endpoint-specific ergonomic methods for the GLEIF API client.
//!
//! This module provides convenient, idiomatic access to all primary GLEIF API endpoints
//! using the generic `GleifRequestBuilder` and consistent naming.

use crate::{client::GleifClient, error::GleifError, request_builder::GleifRequestBuilder};
use serde_json::Value;

impl GleifClient {
    /// List/search LEI records (`/lei-records`).
    pub fn lei_records(&self) -> GleifRequestBuilder {
        self.request("lei-records")
    }

    /// Fetch a single LEI record by LEI (`/lei-records/{lei}`).
    pub async fn lei_record_by_id(&self, lei: &str) -> Result<Value, GleifError> {
        self.request(&format!("lei-records/{}", lei)).send().await
    }

    /// Fetch direct parent relationship for a LEI (`/lei-records/{lei}/direct-parent-relationship`).
    pub async fn direct_parent_relationship(&self, lei: &str) -> Result<Value, GleifError> {
        self.request(&format!("lei-records/{}/direct-parent-relationship", lei)).send().await
    }

    /// Fetch direct parent LEI record (`/lei-records/{lei}/direct-parent`).
    pub async fn direct_parent(&self, lei: &str) -> Result<Value, GleifError> {
        self.request(&format!("lei-records/{}/direct-parent", lei)).send().await
    }

    /// Fetch ultimate parent relationship for a LEI (`/lei-records/{lei}/ultimate-parent-relationship`).
    pub async fn ultimate_parent_relationship(&self, lei: &str) -> Result<Value, GleifError> {
        self.request(&format!("lei-records/{}/ultimate-parent-relationship", lei)).send().await
    }

    /// Fetch ultimate parent LEI record (`/lei-records/{lei}/ultimate-parent`).
    pub async fn ultimate_parent(&self, lei: &str) -> Result<Value, GleifError> {
        self.request(&format!("lei-records/{}/ultimate-parent", lei)).send().await
    }

    /// Fetch direct child relationships for a LEI (`/lei-records/{lei}/direct-child-relationships`).
    pub fn direct_child_relationships(&self, lei: &str) -> GleifRequestBuilder {
        self.request(&format!("lei-records/{}/direct-child-relationships", lei))
    }

    /// Fetch direct children LEI records (`/lei-records/{lei}/direct-children`).
    pub fn direct_children(&self, lei: &str) -> GleifRequestBuilder {
        self.request(&format!("lei-records/{}/direct-children", lei))
    }

    /// Fetch ultimate child relationships for a LEI (`/lei-records/{lei}/ultimate-child-relationships`).
    pub fn ultimate_child_relationships(&self, lei: &str) -> GleifRequestBuilder {
        self.request(&format!("lei-records/{}/ultimate-child-relationships", lei))
    }

    /// Fetch ultimate children LEI records (`/lei-records/{lei}/ultimate-children`).
    pub fn ultimate_children(&self, lei: &str) -> GleifRequestBuilder {
        self.request(&format!("lei-records/{}/ultimate-children", lei))
    }

    /// Fetch associated entity (fund manager) for a LEI (`/lei-records/{lei}/associated-entity`).
    pub async fn associated_entity(&self, lei: &str) -> Result<Value, GleifError> {
        self.request(&format!("lei-records/{}/associated-entity", lei)).send().await
    }

    /// Fetch successor entity for a LEI (`/lei-records/{lei}/successor-entity`).
    pub async fn successor_entity(&self, lei: &str) -> Result<Value, GleifError> {
        self.request(&format!("lei-records/{}/successor-entity", lei)).send().await
    }

    /// Fetch managing LOU for a LEI (`/lei-records/{lei}/managing-lou`).
    pub async fn managing_lou(&self, lei: &str) -> Result<Value, GleifError> {
        self.request(&format!("lei-records/{}/managing-lou", lei)).send().await
    }

    /// Fetch LEI issuer for a LEI (`/lei-records/{lei}/lei-issuer`).
    pub async fn lei_issuer_for_lei(&self, lei: &str) -> Result<Value, GleifError> {
        self.request(&format!("lei-records/{}/lei-issuer", lei)).send().await
    }

    /// Fetch field modifications for a LEI (`/lei-records/{lei}/field-modifications`).
    pub fn field_modifications(&self, lei: &str) -> GleifRequestBuilder {
        self.request(&format!("lei-records/{}/field-modifications", lei))
    }

    /// Fetch ISINs for a LEI (`/lei-records/{lei}/isins`).
    pub fn isins(&self, lei: &str) -> GleifRequestBuilder {
        self.request(&format!("lei-records/{}/isins", lei))
    }

    /// List all LEI issuers (`/lei-issuers`).
    pub fn lei_issuers(&self) -> GleifRequestBuilder {
        self.request("lei-issuers")
    }

    /// Fetch a single LEI issuer by LEI (`/lei-issuers/{lei}`).
    pub async fn lei_issuer_by_id(&self, lei: &str) -> Result<Value, GleifError> {
        self.request(&format!("lei-issuers/{}", lei)).send().await
    }

    /// List all fields (`/fields`).
    pub fn fields(&self) -> GleifRequestBuilder {
        self.request("fields")
    }

    /// Fetch a single field by ID (`/fields/{id}`).
    pub async fn field_by_id(&self, id: &str) -> Result<Value, GleifError> {
        self.request(&format!("fields/{}", id)).send().await
    }

    /// Fuzzy completions (`/fuzzycompletions`). Requires `field` and `q` params.
    pub fn fuzzycompletions(&self) -> GleifRequestBuilder {
        self.request("fuzzycompletions")
    }

    /// List all vLEI issuers (`/vlei-issuers`).
    pub fn vlei_issuers(&self) -> GleifRequestBuilder {
        self.request("vlei-issuers")
    }

    /// Fetch a single vLEI issuer by LEI (`/vlei-issuers/{lei}`).
    pub async fn vlei_issuer_by_id(&self, lei: &str) -> Result<Value, GleifError> {
        self.request(&format!("vlei-issuers/{}", lei)).send().await
    }

    /// List all countries (`/countries`).
    pub fn countries(&self) -> GleifRequestBuilder {
        self.request("countries")
    }

    /// Fetch a single country by code (`/countries/{code}`).
    pub async fn country_by_code(&self, code: &str) -> Result<Value, GleifError> {
        self.request(&format!("countries/{}", code)).send().await
    }

    /// List all entity legal forms (`/entity-legal-forms`).
    pub fn entity_legal_forms(&self) -> GleifRequestBuilder {
        self.request("entity-legal-forms")
    }

    /// Fetch a single entity legal form by ELF code (`/entity-legal-forms/{id}`).
    pub async fn entity_legal_form_by_id(&self, id: &str) -> Result<Value, GleifError> {
        self.request(&format!("entity-legal-forms/{}", id))
            .send()
            .await
    }

    /// List all official organizational roles (`/official-organizational-roles`).
    pub fn official_organizational_roles(&self) -> GleifRequestBuilder {
        self.request("official-organizational-roles")
    }

    /// Fetch a single official organizational role by ID (`/official-organizational-roles/{id}`).
    pub async fn official_organizational_role_by_id(&self, id: &str) -> Result<Value, GleifError> {
        self.request(&format!("official-organizational-roles/{}", id))
            .send()
            .await
    }

    /// List all jurisdictions (`/jurisdictions`).
    pub fn jurisdictions(&self) -> GleifRequestBuilder {
        self.request("jurisdictions")
    }

    /// Fetch a single jurisdiction by code (`/jurisdictions/{code}`).
    pub async fn jurisdiction_by_code(&self, code: &str) -> Result<Value, GleifError> {
        self.request(&format!("jurisdictions/{}", code))
            .send()
            .await
    }

    /// List all regions (`/regions`).
    pub fn regions(&self) -> GleifRequestBuilder {
        self.request("regions")
    }

    /// Fetch a single region by code (`/regions/{code}`).
    pub async fn region_by_code(&self, code: &str) -> Result<Value, GleifError> {
        self.request(&format!("regions/{}", code)).send().await
    }

    /// List all registration authorities (`/registration-authorities`).
    pub fn registration_authorities(&self) -> GleifRequestBuilder {
        self.request("registration-authorities")
    }

    /// Fetch a single registration authority by code (`/registration-authorities/{code}`).
    pub async fn registration_authority_by_code(&self, code: &str) -> Result<Value, GleifError> {
        self.request(&format!("registration-authorities/{}", code))
            .send()
            .await
    }

    /// List all registration agents (`/registration-agents`).
    pub fn registration_agents(&self) -> GleifRequestBuilder {
        self.request("registration-agents")
    }

    /// Fetch a single registration agent by ID (`/registration-agents/{id}`).
    pub async fn registration_agent_by_id(&self, id: &str) -> Result<Value, GleifError> {
        self.request(&format!("registration-agents/{}", id))
            .send()
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::Client as ReqwestClient;

    fn test_client() -> GleifClient {
        GleifClient::from_reqwest_client(ReqwestClient::new())
    }

    #[test]
    fn test_endpoint_builders() {
        let c = test_client();
        let _ = c.lei_records().filter_eq("entity.legalName", "citibank");
        let _ = c.lei_issuers().sort("name");
        let _ = c.fields().page_size(10);
        let _ = c
            .fuzzycompletions()
            .param("field", "fulltext")
            .param("q", "test");
        let _ = c.vlei_issuers().page_number(1);
        let _ = c.countries().sort("name");
        let _ = c.entity_legal_forms().filter_eq("country", "DE");
        let _ = c
            .official_organizational_roles()
            .filter_eq("name", "manager");
        let _ = c.jurisdictions().page_size(5);
        let _ = c.regions().sort("code");
        let _ = c.registration_authorities().filter_eq("country", "US");
        let _ = c
            .registration_agents()
            .filter_eq("leiIssuer", "EVK05KS7XY1DEII3R011");
    }
}
