//! Metadata and reference data endpoints for the GLEIF API client.

use crate::{client::GleifClient, error::Result, request_builder::GleifRequestBuilder};
use serde::de::DeserializeOwned;

impl GleifClient {
    /// Fetches field modifications for a specific LEI (Legal Entity Identifier).
    ///
    /// This method sends a request to the `/lei-records/{lei}/field-modifications` endpoint to retrieve
    /// details about modifications made to the fields of the specified LEI record. The response can be
    /// filtered to include only a subset of field modifications.
    ///
    /// Supported filters:
    /// - `recordType`
    /// - `modificationDate`
    /// - `field`
    /// - `date`
    ///
    /// # Parameters
    ///
    /// * `lei` - A string slice representing the LEI identifier.
    ///
    /// # Errors
    ///
    /// This method does not itself return errors. However, errors may occur when sending the request or processing
    /// the response using the returned request builder (e.g., network failures or deserialization issues).
    ///
    /// # Examples
    ///
    /// ```rust, ignore
    /// let modifications: GleifApiResponse<Vec<LeiFieldModification>> = client.field_modifications("5493000IBP32UQZ0KL24").send().await?; // strongly typed
    /// let modifications: serde_json::Value = client.field_modifications("5493000IBP32UQZ0KL24").send().await?; // raw JSON
    /// ```
    #[must_use]
    pub fn field_modifications(&self, lei: &str) -> GleifRequestBuilder {
        self.request(&format!("lei-records/{lei}/field-modifications"))
    }

    /// Fetches a list of all available data fields for filtering LEI records (`/fields`).
    ///
    /// This endpoint provides detailed documentation of the data fields available in the API.
    /// Pagination parameters can be used to manage the large number of data items.
    ///
    /// # Errors
    ///
    /// This method does not itself return errors. However, errors may occur when sending the request or processing
    /// the response using the returned request builder (e.g., network failures or deserialization issues).
    ///
    /// # Examples
    ///
    /// ```rust, ignore
    /// let fields: GleifApiResponse<Vec<Field>> = client.fields().send().await?; // strongly typed
    /// let fields: serde_json::Value = client.fields().send().await?; // raw JSON
    /// ```
    #[must_use]
    pub fn fields(&self) -> GleifRequestBuilder {
        self.request("fields")
    }

    /// Fetches details of a single data field for filtering LEI records (`/fields/{id}`).
    ///
    /// This method sends a request to retrieve information about a specific field by its ID.
    ///
    /// # Parameters
    ///
    /// * `id` - A string slice representing the unique identifier of the field.
    ///
    /// # Errors
    ///
    /// This method returns a [`GleifError`] in the following cases:
    /// * The request could not be completed due to network or server issues.
    /// * The response body could not be deserialized into the expected type.
    ///
    /// # Examples
    ///
    /// ```rust, ignore
    /// let field: GleifApiResponse<Field> = client.field_by_id("LEIREC_LEGAL_NAME").await?; // strongly typed
    /// let field: serde_json::Value = client.field_by_id("LEIREC_LEGAL_NAME").await?; // raw JSON
    /// ```
    pub async fn field_by_id<R>(&self, id: &str) -> Result<R>
    where
        R: DeserializeOwned,
    {
        self.request(&format!("fields/{id}")).send().await
    }

    /// Retrieves all ISO 3166 Country Codes.
    ///
    /// This method sends a request to the `/countries` endpoint to fetch a list of all ISO 3166 Country Codes.
    /// Pagination parameters can be used to manage the large number of data items.
    ///
    /// # Errors
    ///
    /// This method does not itself return errors. However, errors may occur when sending the request or processing
    /// the response using the returned request builder (e.g., network failures or deserialization issues).
    ///
    /// # Examples
    ///
    /// ```rust, ignore
    /// let countries: GleifApiResponse<Vec<Country>> = client.countries().send().await?; // strongly typed
    /// let countries: serde_json::Value = client.countries().send().await?; // raw JSON
    /// ```
    #[must_use]
    pub fn countries(&self) -> GleifRequestBuilder {
        self.request("countries")
    }

    /// Fetches ISO 3166 Country Code details by ISO 3166 Country Code (`/countries/{id}`).
    ///
    /// This method sends a request to retrieve details of a single country by its ISO 3166 code.
    ///
    /// # Parameters
    ///
    /// * `id` - A string slice representing the ISO 3166 Country Code.
    ///
    /// # Errors
    ///
    /// This method returns a [`GleifError`] in the following cases:
    /// * The request could not be completed due to network or server issues.
    /// * The response body could not be deserialized into the expected type.
    ///
    /// # Examples
    ///
    /// ```rust, ignore
    /// let country: GleifApiResponse<Country> = client.country_by_id("US").await?; // strongly typed
    /// let country: serde_json::Value = client.country_by_id("US").await?; // raw JSON
    /// ```
    pub async fn country_by_id<R>(&self, id: &str) -> Result<R>
    where
        R: DeserializeOwned,
    {
        self.request(&format!("countries/{id}")).send().await
    }

    /// Fetches a list of all entity legal forms (`/entity-legal-forms`).
    ///
    /// This method sends a request to retrieve all available entity legal forms.
    /// Pagination parameters can be used to manage the large number of data items.
    ///
    /// # Errors
    ///
    /// This method does not itself return errors. However, errors may occur when sending the request or processing
    /// the response using the returned request builder (e.g., network failures or deserialization issues).
    ///
    /// # Examples
    ///
    /// ```rust, ignore
    /// let legal_forms: GleifApiResponse<Vec<EntityLegalForm>> = client.entity_legal_forms().send().await?; // strongly typed
    /// let legal_forms: serde_json::Value = client.entity_legal_forms().send().await?; // raw JSON
    /// ```
    #[must_use]
    pub fn entity_legal_forms(&self) -> GleifRequestBuilder {
        self.request("entity-legal-forms")
    }

    /// Fetches details of a single entity legal form by ELF code (`/entity-legal-forms/{id}`).
    ///
    /// This method sends a request to retrieve information about a specific entity legal form by its ELF code.
    ///
    /// # Parameters
    ///
    /// * `id` - A string slice representing the unique ELF code of the entity legal form.
    ///
    /// # Errors
    ///
    /// This method returns a [`GleifError`] in the following cases:
    /// * The request could not be completed due to network or server issues.
    /// * The response body could not be deserialized into the expected type.
    ///
    /// # Examples
    ///
    /// ```rust, ignore
    /// let legal_form: GleifApiResponse<EntityLegalForm> = client.entity_legal_form_by_id("10UR").await?; // strongly typed
    /// let legal_form: serde_json::Value = client.entity_legal_form_by_id("10UR").await?; // raw JSON
    /// ```
    pub async fn entity_legal_form_by_id<R>(&self, id: &str) -> Result<R>
    where
        R: DeserializeOwned,
    {
        self.request(&format!("entity-legal-forms/{id}"))
            .send()
            .await
    }

    /// Retrieves all Official Organizational Roles (OOR) Code List (`/official-organizational-roles`).
    ///
    /// This method sends a request to fetch the complete list of official organizational roles.
    /// The list contains over 2100 official organizational roles (as of November 2024) for nearly
    /// 250 legal forms across 89 jurisdictions.
    ///
    /// # Filtering Options
    ///
    /// The following filters can be applied to narrow down the results:
    /// - **Primary Name**: Filter by the primary name of the role.
    /// - **Transliterated Name**: Filter by the transliterated name of the role.
    /// - **Case-insensitive Name**: Filter by a case-insensitive name match.
    /// - **Matching String**: Filter by a partial string match in the name.
    /// - **Country Code**: Filter by the ISO 3166 country code.
    ///
    /// # Errors
    ///
    /// This method does not itself return errors. However, errors may occur when sending the request or processing
    /// the response using the returned request builder (e.g., network failures or deserialization issues).
    ///
    /// # Examples
    ///
    /// ```rust, ignore
    /// // Retrieve all roles
    /// let roles: GleifApiResponse<Vec<OfficialOrganizationalRole>> = client.official_organizational_roles().send().await?;
    ///
    /// // Filter by primary name
    /// let roles: GleifApiResponse<Vec<OfficialOrganizationalRole>> = client
    ///     .official_organizational_roles()
    ///     .filter_eq("name", "управног")
    ///     .send()
    ///     .await?;
    ///
    /// // Filter by country code
    /// let roles: GleifApiResponse<Vec<OfficialOrganizationalRole>> = client
    ///     .official_organizational_roles()
    ///     .filter_eq("countryCode", "CA")
    ///     .send()
    ///     .await?;
    /// ```
    #[must_use]
    pub fn official_organizational_roles(&self) -> GleifRequestBuilder {
        self.request("official-organizational-roles")
    }

    /// Fetches details of a single official organizational role by OOR code (`/official-organizational-roles/{id}`).
    ///
    /// This method sends a request to retrieve information about a specific official organizational role by its ID.
    ///
    /// # Parameters
    ///
    /// * `id` - A string slice representing the unique identifier of the official organizational role.
    ///
    /// # Errors
    ///
    /// This method returns a [`GleifError`] in the following cases:
    /// * The request could not be completed due to network or server issues.
    /// * The response body could not be deserialized into the expected type.
    ///
    /// # Examples
    ///
    /// ```rust, ignore
    /// let role: GleifApiResponse<OfficialOrganizationalRole> = client.official_organizational_role_by_id("0CGNG5").await?; // strongly typed
    /// let role: serde_json::Value = client.official_organizational_role_by_id("0CGNG5").await?; // raw JSON
    /// ```
    pub async fn official_organizational_role_by_id<R>(&self, id: &str) -> Result<R>
    where
        R: DeserializeOwned,
    {
        self.request(&format!("official-organizational-roles/{id}"))
            .send()
            .await
    }

    /// Fetches a list of all jurisdictions (`/jurisdictions`).
    ///
    /// This method sends a request to retrieve all legal jurisdictions based on the ISO 3166 Country
    /// and Sub-Region Codes. Pagination parameters can be used to manage the large number of data items.
    ///
    /// # Errors
    ///
    /// This method does not itself return errors. However, errors may occur when sending the request or processing
    /// the response using the returned request builder (e.g., network failures or deserialization issues).
    ///
    /// # Examples
    ///
    /// ```rust, ignore
    /// let jurisdictions: GleifApiResponse<Vec<Jurisdiction>> = client.jurisdictions().send().await?; // strongly typed
    /// let jurisdictions: serde_json::Value = client.jurisdictions().send().await?; // raw JSON
    /// ```
    #[must_use]
    pub fn jurisdictions(&self) -> GleifRequestBuilder {
        self.request("jurisdictions")
    }

    /// Fetches details of a single jurisdiction by jurisdiction code (`/jurisdictions/{id}`).
    ///
    /// This method sends a request to retrieve information about a specific jurisdiction by its code.
    ///
    /// # Parameters
    ///
    /// * `id` - A string slice representing the unique jurisdiction code.
    ///
    /// # Errors
    ///
    /// This method returns a [`GleifError`] in the following cases:
    /// * The request could not be completed due to network or server issues.
    /// * The response body could not be deserialized into the expected type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let jurisdiction: GleifApiResponse<Jurisdiction> = client.jurisdiction_by_id("US").await?; // strongly typed
    /// let jurisdiction: serde_json::Value = client.jurisdiction_by_id("US").await?; // raw JSON
    /// ```
    pub async fn jurisdiction_by_id<R>(&self, id: &str) -> Result<R>
    where
        R: DeserializeOwned,
    {
        self.request(&format!("jurisdictions/{id}")).send().await
    }

    /// Retrieves all ISO 3166 Region Codes (`/regions`).
    ///
    /// This method sends a request to fetch a list of all ISO 3166 Region Codes.
    /// Pagination parameters can be used to manage the large number of data items.
    ///
    /// # Errors
    ///
    /// This method does not itself return errors. However, errors may occur when sending the request or processing
    /// the response using the returned request builder (e.g., network failures or deserialization issues).
    ///
    /// # Examples
    ///
    /// ```rust, ignore
    /// let regions: GleifApiResponse<Vec<Region>> = client.regions().send().await?; // strongly typed
    /// let regions: serde_json::Value = client.regions().send().await?; // raw JSON
    /// ```
    #[must_use]
    pub fn regions(&self) -> GleifRequestBuilder {
        self.request("regions")
    }

    /// Fetches details of a single region by ISO 3166 Region Code (`/regions/{id}`).
    ///
    /// This method sends a request to retrieve information about a specific region by its code.
    ///
    /// # Parameters
    ///
    /// * `id` - A string slice representing the unique ISO 3166 Region Code.
    ///
    /// # Errors
    ///
    /// This method returns a [`GleifError`] in the following cases:
    /// * The request could not be completed due to network or server issues.
    /// * The response body could not be deserialized into the expected type.
    ///
    /// # Examples
    ///
    /// ```rust, ignore
    /// let region: GleifApiResponse<Region> = client.region_by_id("AD-03").await?; // strongly typed
    /// let region: serde_json::Value = client.region_by_id("AD-03").await?; // raw JSON
    /// ```
    pub async fn region_by_id<R>(&self, id: &str) -> Result<R>
    where
        R: DeserializeOwned,
    {
        self.request(&format!("regions/{id}")).send().await
    }

    /// Retrieves all Registration Authorities (`/registration-authorities`).
    ///
    /// This method sends a request to fetch the complete list of Registration Authorities based on
    /// the GLEIF Registration Authority (RA) Code List. Pagination parameters can be used to manage
    /// the large number of data items.
    ///
    /// # Errors
    ///
    /// This method does not itself return errors. However, errors may occur when sending the request or processing
    /// the response using the returned request builder (e.g., network failures or deserialization issues).
    ///
    /// # Examples
    ///
    /// ```rust, ignore
    /// let authorities: GleifApiResponse<Vec<RegistrationAuthority>> = client.registration_authorities().send().await?; // strongly typed
    /// let authorities: serde_json::Value = client.registration_authorities().send().await?; // raw JSON
    /// ```
    #[must_use]
    pub fn registration_authorities(&self) -> GleifRequestBuilder {
        self.request("registration-authorities")
    }

    /// Fetches details of a single Registration Authority by RA List Code (`/registration-authorities/{id}`).
    ///
    /// This method sends a request to retrieve information about a specific registration authority by its code.
    ///
    /// # Parameters
    ///
    /// * `id` - A string slice representing the unique RA List Code of the registration authority.
    ///
    /// # Errors
    ///
    /// This method returns a [`GleifError`] in the following cases:
    /// * The request could not be completed due to network or server issues.
    /// * The response body could not be deserialized into the expected type.
    ///
    /// # Examples
    ///
    /// ```rust, ignore
    /// let authority: GleifApiResponse<RegistrationAuthority> = client.registration_authority_by_id("RA000001").await?; // strongly typed
    /// let authority: serde_json::Value = client.registration_authority_by_id("RA000001").await?; // raw JSON
    /// ```
    pub async fn registration_authority_by_id<R>(&self, id: &str) -> Result<R>
    where
        R: DeserializeOwned,
    {
        self.request(&format!("registration-authorities/{id}"))
            .send()
            .await
    }

    /// Fetches a list of all Registration Agents (`/registration-agents`).
    ///
    /// This method sends a request to retrieve all Registration Agents that have consented to have
    /// their information published. The response can be filtered using the following parameters:
    /// - **`leiIssuer`**: The LEI of the associated LEI Issuer.
    /// - **`lei`**: The LEI of the Registration Agent itself.
    ///
    /// # Errors
    ///
    /// This method does not itself return errors. However, errors may occur when sending the request or processing
    /// the response using the returned request builder (e.g., network failures or deserialization issues).
    ///
    /// # Examples
    ///
    /// ```rust, ignore
    /// let agents: GleifApiResponse<Vec<RegistrationAgent>> = client.registration_agents().send().await?; // strongly typed
    /// let agents: serde_json::Value = client.registration_agents().send().await?; // raw JSON
    /// ```
    #[must_use]
    pub fn registration_agents(&self) -> GleifRequestBuilder {
        self.request("registration-agents")
    }

    /// Fetches details of a single Registration Agent by its unique ID (`/registration-agents/{id}`).
    ///
    /// This method sends a request to retrieve information about a specific registration agent by its ID.
    ///
    /// # Parameters
    ///
    /// * `id` - A string slice representing the unique identifier of the registration agent.
    ///
    /// # Errors
    ///
    /// This method returns a [`GleifError`] in the following cases:
    /// * The request could not be completed due to network or server issues.
    /// * The response body could not be deserialized into the expected type.
    ///
    /// # Examples
    ///
    /// ```rust, ignore
    /// let agent: GleifApiResponse<RegistrationAgent> = client.registration_agent_by_id("5d10d4dc9f3764.95022907").await?; // strongly typed
    /// let agent: serde_json::Value = client.registration_agent_by_id("5d10d4dc9f3764.95022907").await?; // raw JSON
    /// ```
    pub async fn registration_agent_by_id<R>(&self, id: &str) -> Result<R>
    where
        R: DeserializeOwned,
    {
        self.request(&format!("registration-agents/{id}"))
            .send()
            .await
    }
}
