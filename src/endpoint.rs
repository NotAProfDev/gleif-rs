//! Endpoint-specific ergonomic methods for the GLEIF API client.
//!
//! This module provides convenient, idiomatic access to all primary GLEIF API endpoints
//! using the generic `GleifRequestBuilder` and consistent naming.
//!
//! # Examples
//!
//! ## Working with collections
//!
//! ```rust
//! use gleif_rs::client::GleifClient;
//!
//! # async fn example() -> Result<(), gleif_rs::error::GleifError> {
//! let client = GleifClient::new();
//!
//! // Search for LEI records with multiple filters
//! let results = client.lei_records()
//!     .filter_eq("entity.status", "ACTIVE")
//!     .filter_eq("entity.legalAddress.country", "DE")
//!     .sort("entity.legalName")
//!     .page_size(10)
//!     .send()
//!     .await?;
//!
//! println!("Found {} records", results["data"].as_array().unwrap().len());
//!
//! // Similarly for other collection endpoints
//! let issuers = client.lei_issuers()
//!     .filter_eq("status", "ACTIVE")
//!     .page_size(5)
//!     .send()
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Fetching single entities
//!
//! ```rust
//! use gleif_rs::client::GleifClient;
//!
//! # async fn example() -> Result<(), gleif_rs::error::GleifError> {
//! let client = GleifClient::new();
//! let lei = "5493000IBP32UQZ0KL24";
//!
//! // Fetch a specific LEI record
//! let record = client.lei_record_by_id(lei).await?;
//! println!("Entity name: {}", record["attributes"]["entity"]["legalName"]);
//!
//! // Fetch relationship data
//! let parent = client.direct_parent_relationship(lei).await?;
//! if let Some(relationship) = parent.get("data") {
//!     println!("Has direct parent: {}", !relationship.is_null());
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Working with parent-child relationships
//!
//! ```rust
//! use gleif_rs::client::GleifClient;
//! use futures_util::StreamExt;
//!
//! # async fn example() -> Result<(), gleif_rs::error::GleifError> {
//! let client = GleifClient::new();
//! let lei = "5493000IBP32UQZ0KL24";
//!
//! // Get all direct children with pagination
//! let children_stream = client.direct_children(lei)
//!     .page_size(100)
//!     .paginated();
//!     
//! futures_util::pin_mut!(children_stream);
//!
//! let mut child_count = 0;
//! while let Some(page_result) = children_stream.next().await {
//!     let page = page_result?;
//!     let records = page["data"].as_array().unwrap();
//!     child_count += records.len();
//!     
//!     for record in records {
//!         let child_lei = record["id"].as_str().unwrap();
//!         println!("Child LEI: {child_lei}");
//!     }
//! }
//! println!("Total children: {child_count}");
//! # Ok(())
//! # }
//! ```
//!
//! ## Using specialized endpoints
//!
//! ```rust
//! use gleif_rs::client::GleifClient;
//!
//! # async fn example() -> Result<(), gleif_rs::error::GleifError> {
//! let client = GleifClient::new();
//!
//! // Look up entity legal forms by country
//! let legal_forms = client.entity_legal_forms()
//!     .filter_eq("country", "GB")
//!     .send()
//!     .await?;
//!     
//! for form in legal_forms["data"].as_array().unwrap() {
//!     let code = form["id"].as_str().unwrap();
//!     let name = form["attributes"]["name"].as_str().unwrap();
//!     println!("{code}: {name}");
//! }
//!
//! // Look up countries
//! let country = client.country_by_code("US").await?;
//! println!("Country: {}", country["attributes"]["name"]);
//! # Ok(())
//! # }
//! ```

use crate::{client::GleifClient, error::Result, request_builder::GleifRequestBuilder};
use serde_json::Value;

impl GleifClient {
    /// List/search LEI records (`/lei-records`).
    pub fn lei_records(&self) -> GleifRequestBuilder {
        self.request("lei-records")
    }

    /// Fetch a single LEI record by LEI (`/lei-records/{lei}`).
    pub async fn lei_record_by_id(&self, lei: &str) -> Result<Value> {
        self.request(&format!("lei-records/{}", lei)).send().await
    }

    /// Fetch direct parent relationship for a LEI (`/lei-records/{lei}/direct-parent-relationship`).
    pub async fn direct_parent_relationship(&self, lei: &str) -> Result<Value> {
        self.request(&format!("lei-records/{}/direct-parent-relationship", lei))
            .send()
            .await
    }

    /// Fetch direct parent LEI record (`/lei-records/{lei}/direct-parent`).
    pub async fn direct_parent(&self, lei: &str) -> Result<Value> {
        self.request(&format!("lei-records/{}/direct-parent", lei))
            .send()
            .await
    }

    /// Fetch ultimate parent relationship for a LEI (`/lei-records/{lei}/ultimate-parent-relationship`).
    pub async fn ultimate_parent_relationship(&self, lei: &str) -> Result<Value> {
        self.request(&format!("lei-records/{}/ultimate-parent-relationship", lei))
            .send()
            .await
    }

    /// Fetch ultimate parent LEI record (`/lei-records/{lei}/ultimate-parent`).
    pub async fn ultimate_parent(&self, lei: &str) -> Result<Value> {
        self.request(&format!("lei-records/{}/ultimate-parent", lei))
            .send()
            .await
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
    pub async fn associated_entity(&self, lei: &str) -> Result<Value> {
        self.request(&format!("lei-records/{}/associated-entity", lei))
            .send()
            .await
    }

    /// Fetch successor entity for a LEI (`/lei-records/{lei}/successor-entity`).
    pub async fn successor_entity(&self, lei: &str) -> Result<Value> {
        self.request(&format!("lei-records/{}/successor-entity", lei))
            .send()
            .await
    }

    /// Fetch managing LOU for a LEI (`/lei-records/{lei}/managing-lou`).
    pub async fn managing_lou(&self, lei: &str) -> Result<Value> {
        self.request(&format!("lei-records/{}/managing-lou", lei))
            .send()
            .await
    }

    /// Fetch LEI issuer for a LEI (`/lei-records/{lei}/lei-issuer`).
    pub async fn lei_issuer_for_lei(&self, lei: &str) -> Result<Value> {
        self.request(&format!("lei-records/{}/lei-issuer", lei))
            .send()
            .await
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
    pub async fn lei_issuer_by_id(&self, lei: &str) -> Result<Value> {
        self.request(&format!("lei-issuers/{}", lei)).send().await
    }

    /// List all fields (`/fields`).
    pub fn fields(&self) -> GleifRequestBuilder {
        self.request("fields")
    }

    /// Fetch a single field by ID (`/fields/{id}`).
    pub async fn field_by_id(&self, id: &str) -> Result<Value> {
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
    pub async fn vlei_issuer_by_id(&self, lei: &str) -> Result<Value> {
        self.request(&format!("vlei-issuers/{}", lei)).send().await
    }

    /// List all countries (`/countries`).
    pub fn countries(&self) -> GleifRequestBuilder {
        self.request("countries")
    }

    /// Fetch a single country by code (`/countries/{code}`).
    pub async fn country_by_code(&self, code: &str) -> Result<Value> {
        self.request(&format!("countries/{}", code)).send().await
    }

    /// List all entity legal forms (`/entity-legal-forms`).
    pub fn entity_legal_forms(&self) -> GleifRequestBuilder {
        self.request("entity-legal-forms")
    }

    /// Fetch a single entity legal form by ELF code (`/entity-legal-forms/{id}`).
    pub async fn entity_legal_form_by_id(&self, id: &str) -> Result<Value> {
        self.request(&format!("entity-legal-forms/{}", id))
            .send()
            .await
    }

    /// List all official organizational roles (`/official-organizational-roles`).
    pub fn official_organizational_roles(&self) -> GleifRequestBuilder {
        self.request("official-organizational-roles")
    }

    /// Fetch a single official organizational role by ID (`/official-organizational-roles/{id}`).
    pub async fn official_organizational_role_by_id(&self, id: &str) -> Result<Value> {
        self.request(&format!("official-organizational-roles/{}", id))
            .send()
            .await
    }

    /// List all jurisdictions (`/jurisdictions`).
    pub fn jurisdictions(&self) -> GleifRequestBuilder {
        self.request("jurisdictions")
    }

    /// Fetch a single jurisdiction by code (`/jurisdictions/{code}`).
    pub async fn jurisdiction_by_code(&self, code: &str) -> Result<Value> {
        self.request(&format!("jurisdictions/{}", code))
            .send()
            .await
    }

    /// List all regions (`/regions`).
    pub fn regions(&self) -> GleifRequestBuilder {
        self.request("regions")
    }

    /// Fetch a single region by code (`/regions/{code}`).
    pub async fn region_by_code(&self, code: &str) -> Result<Value> {
        self.request(&format!("regions/{}", code)).send().await
    }

    /// List all registration authorities (`/registration-authorities`).
    pub fn registration_authorities(&self) -> GleifRequestBuilder {
        self.request("registration-authorities")
    }

    /// Fetch a single registration authority by code (`/registration-authorities/{code}`).
    pub async fn registration_authority_by_code(&self, code: &str) -> Result<Value> {
        self.request(&format!("registration-authorities/{}", code))
            .send()
            .await
    }

    /// List all registration agents (`/registration-agents`).
    pub fn registration_agents(&self) -> GleifRequestBuilder {
        self.request("registration-agents")
    }

    /// Fetch a single registration agent by ID (`/registration-agents/{id}`).
    pub async fn registration_agent_by_id(&self, id: &str) -> Result<Value> {
        self.request(&format!("registration-agents/{}", id))
            .send()
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::Client as ReqwestClient;
    use reqwest_middleware::ClientBuilder;

    fn test_client() -> GleifClient {
        GleifClient::from_reqwest_client(ReqwestClient::new())
    }

    // Tests that endpoint path construction is correct
    #[test]
    fn test_endpoint_path_construction() {
        let c = test_client();

        // Test single entity endpoints
        let lei = "5493000IBP32UQZ0KL24";

        // Compare path construction for record endpoints
        let record_path = format!("lei-records/{}", lei);
        assert_eq!(c.request(&record_path).get_path(), record_path);

        // Test relationship endpoints
        let direct_parent_rel_path = format!("lei-records/{}/direct-parent-relationship", lei);
        assert_eq!(
            c.request(&direct_parent_rel_path).get_path(),
            direct_parent_rel_path
        );

        let direct_parent_path = format!("lei-records/{}/direct-parent", lei);
        assert_eq!(
            c.request(&direct_parent_path).get_path(),
            direct_parent_path
        );

        // Test collection endpoints with correct path access
        let builder = c.direct_child_relationships(lei);
        assert_eq!(
            builder.get_path(),
            format!("lei-records/{}/direct-child-relationships", lei)
        );

        let builder = c.direct_children(lei);
        assert_eq!(
            builder.get_path(),
            format!("lei-records/{}/direct-children", lei)
        );

        // Test other entity types
        let lei_issuer_path = format!("lei-issuers/{}", lei);
        assert_eq!(c.request(&lei_issuer_path).get_path(), lei_issuer_path);

        let vlei_issuer_path = format!("vlei-issuers/{}", lei);
        assert_eq!(c.request(&vlei_issuer_path).get_path(), vlei_issuer_path);

        let country_code = "US";
        let country_path = format!("countries/{}", country_code);
        assert_eq!(c.request(&country_path).get_path(), country_path);
    }

    #[test]
    fn test_complex_query_construction() {
        let c = test_client();

        // Test multiple filters with sorting and pagination
        let builder = c
            .lei_records()
            .filter_eq("entity.status", "ACTIVE")
            .filter_eq("entity.legalAddress.country", "DE")
            .filter_not_in("entity.legalForm.id", ["8888", "9999"])
            .sort("entity.legalName")
            .page_size(25)
            .page_number(2);

        let query = builder.get_query();
        assert_eq!(query.get("filter[entity.status]").unwrap(), "ACTIVE");
        assert_eq!(
            query.get("filter[entity.legalAddress.country]").unwrap(),
            "DE"
        );
        assert_eq!(
            query.get("filter[entity.legalForm.id]").unwrap(),
            "!8888,9999"
        );
        assert_eq!(query.get("sort").unwrap(), "entity.legalName");
        assert_eq!(query.get("page[size]").unwrap(), "25");
        assert_eq!(query.get("page[number]").unwrap(), "2");

        // Test range filters
        let builder = c
            .lei_records()
            .filter_range(
                "registration.initialRegistrationDate",
                "2021-01-01",
                "2021-12-31",
            )
            .filter_gte("registration.nextRenewalDate", "2022-01-01")
            .filter_lt("registration.managingLOU.id", "9");

        let query = builder.get_query();
        assert_eq!(
            query
                .get("filter[registration.initialRegistrationDate]")
                .unwrap(),
            "2021-01-01..2021-12-31"
        );
        assert_eq!(
            query.get("filter[registration.nextRenewalDate]").unwrap(),
            ">=2022-01-01"
        );
        assert_eq!(
            query.get("filter[registration.managingLOU.id]").unwrap(),
            "<9"
        );
    }

    #[test]
    fn test_request_chaining_and_immutability() {
        let c = test_client();

        // Base query
        let base_query = c.lei_records().filter_eq("entity.status", "ACTIVE");

        // Derived query 1
        let query1 = base_query
            .clone()
            .filter_eq("entity.legalAddress.country", "US");

        // Derived query 2 with different filter
        let query2 = base_query
            .clone()
            .filter_eq("entity.legalAddress.country", "DE");

        // Verify base query is unchanged
        assert_eq!(
            base_query.get_query().get("filter[entity.status]").unwrap(),
            "ACTIVE"
        );
        assert!(
            base_query
                .get_query()
                .get("filter[entity.legalAddress.country]")
                .is_none()
        );

        // Verify derived queries have correct filters
        assert_eq!(
            query1
                .get_query()
                .get("filter[entity.legalAddress.country]")
                .unwrap(),
            "US"
        );
        assert_eq!(
            query2
                .get_query()
                .get("filter[entity.legalAddress.country]")
                .unwrap(),
            "DE"
        );
    }

    #[test]
    fn test_client_with_custom_config() {
        // Test that endpoints work with custom client configurations
        let reqwest_client = ReqwestClient::new();
        let middleware_client = ClientBuilder::new(reqwest_client).build();
        let c = GleifClient::from_middleware_client(middleware_client);

        // Verify endpoint methods still work
        let builder = c.lei_records().filter_eq("entity.legalName", "test");
        assert_eq!(
            builder.get_query().get("filter[entity.legalName]").unwrap(),
            "test"
        );
    }

    #[test]
    fn test_child_and_parent_endpoint_paths() {
        let c = test_client();
        let lei = "5493000IBP32UQZ0KL24";

        // Test ultimate parent/child relationship paths
        let builder = c.ultimate_child_relationships(lei);
        assert_eq!(
            builder.get_path(),
            format!("lei-records/{}/ultimate-child-relationships", lei)
        );

        let builder = c.ultimate_children(lei);
        assert_eq!(
            builder.get_path(),
            format!("lei-records/{}/ultimate-children", lei)
        );

        // Test field modifications path
        let builder = c.field_modifications(lei);
        assert_eq!(
            builder.get_path(),
            format!("lei-records/{}/field-modifications", lei)
        );

        // Test ISINs path
        let builder = c.isins(lei);
        assert_eq!(builder.get_path(), format!("lei-records/{}/isins", lei));
    }

    #[test]
    fn test_specialized_endpoint_methods() {
        let c = test_client();
        let country_code = "GB";

        // Test specialized endpoint methods
        let builder = c.entity_legal_forms().filter_eq("country", country_code);
        assert_eq!(
            builder.get_query().get("filter[country]").unwrap(),
            country_code
        );

        let builder = c.jurisdictions().page_size(50);
        assert_eq!(builder.get_query().get("page[size]").unwrap(), "50");

        // Test endpoints with multiple filters combined
        let builder = c
            .registration_authorities()
            .filter_eq("country", country_code)
            .filter_eq("name", "Companies House")
            .sort("name");

        assert_eq!(
            builder.get_query().get("filter[country]").unwrap(),
            country_code
        );
        assert_eq!(
            builder.get_query().get("filter[name]").unwrap(),
            "Companies House"
        );
        assert_eq!(builder.get_query().get("sort").unwrap(), "name");
    }
}
