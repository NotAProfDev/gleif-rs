//! Generic request builder for the GLEIF API client.
//!
//! This module provides the `GleifRequestBuilder` struct, which is a flexible and reusable
//! builder for constructing and executing requests to the GLEIF API. It allows users to
//! customize various aspects of the request, such as HTTP method, endpoint path, query
//! parameters, filters, sorting, and pagination.
//!
//! The builder pattern is used to enable a fluent and ergonomic API for constructing requests.
//! Each method returns a modified instance of the builder, allowing for method chaining.
//!
//! # Features
//!
//! - Add filters for exact matches, ranges, inclusion/exclusion, and comparisons.
//! - Specify sorting and pagination options.
//! - Add custom query parameters.
//! - Execute the request and deserialize the response into a strongly-typed struct or raw JSON.

use crate::{
    client::GleifClient,
    error::{GleifError, Result},
};
use reqwest::Method;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use url::Url;

/// Builder for constructing and executing GLEIF API requests generically.
#[derive(Debug, Clone)]
pub struct GleifRequestBuilder {
    client: GleifClient,
    method: Method,
    path: String,
    query: HashMap<String, String>,
}

impl GleifRequestBuilder {
    /// Create a new builder for the given client, method, and endpoint path.
    #[must_use]
    pub fn new(client: GleifClient, method: Method, path: impl Into<String>) -> Self {
        Self {
            client,
            method,
            path: path.into(),
            query: HashMap::new(),
        }
    }

    /// Returns the current path for this request builder.
    #[must_use]
    pub fn get_path(&self) -> &str {
        &self.path
    }

    /// Returns a reference to the query parameters for this request builder.
    #[must_use]
    pub fn get_query(&self) -> &HashMap<String, String> {
        &self.query
    }

    /// Private helper to insert a filter parameter with the given operator syntax.
    #[must_use]
    fn set_filter(mut self, field: &str, value: String) -> Self {
        self.query.insert(format!("filter[{field}]"), value);
        self
    }

    /// Add a filter for exact match (e.g., `filter[field]=value`).
    #[must_use]
    pub fn filter_eq<F, V>(self, field: F, value: V) -> Self
    where
        F: AsRef<str>,
        V: AsRef<str>,
    {
        self.set_filter(field.as_ref(), value.as_ref().to_string())
    }

    /// Add a filter for NOT match (e.g., `filter[field]=!value`).
    #[must_use]
    pub fn filter_not<F, V>(self, field: F, value: V) -> Self
    where
        F: AsRef<str>,
        V: AsRef<str>,
    {
        self.set_filter(field.as_ref(), format!("!{}", value.as_ref()))
    }

    /// Add a filter for IN (comma-separated values, e.g., `filter[field]=a,b,c`).
    #[must_use]
    pub fn filter_in<F, I, V>(self, field: F, values: I) -> Self
    where
        F: AsRef<str>,
        I: IntoIterator<Item = V>,
        V: AsRef<str>,
    {
        let joined = values
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<String>>()
            .join(",");
        self.set_filter(field.as_ref(), joined)
    }

    /// Add a filter for NOT IN (e.g., `filter[field]=!a,b,c`).
    #[must_use]
    pub fn filter_not_in<F, I, V>(self, field: F, values: I) -> Self
    where
        F: AsRef<str>,
        I: IntoIterator<Item = V>,
        V: AsRef<str>,
    {
        let joined = values
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<String>>()
            .join(",");
        self.set_filter(field.as_ref(), format!("!{joined}"))
    }

    /// Add a filter for range (e.g., `filter[field]=min..max`).
    #[must_use]
    pub fn filter_range<F, V>(self, field: F, min: V, max: V) -> Self
    where
        F: AsRef<str>,
        V: AsRef<str>,
    {
        self.set_filter(
            field.as_ref(),
            format!("{}..{}", min.as_ref(), max.as_ref()),
        )
    }

    /// Add a filter for greater than (e.g., `filter[field]=>value`).
    #[must_use]
    pub fn filter_gt<F, V>(self, field: F, value: V) -> Self
    where
        F: AsRef<str>,
        V: AsRef<str>,
    {
        self.set_filter(field.as_ref(), format!(">{}", value.as_ref()))
    }

    /// Add a filter for greater than or equal (e.g., `filter[field]=>=value`).
    #[must_use]
    pub fn filter_gte<F, V>(self, field: F, value: V) -> Self
    where
        F: AsRef<str>,
        V: AsRef<str>,
    {
        self.set_filter(field.as_ref(), format!(">={}", value.as_ref()))
    }

    /// Add a filter for less than (e.g., `filter[field]=<value`).
    #[must_use]
    pub fn filter_lt<F, V>(self, field: F, value: V) -> Self
    where
        F: AsRef<str>,
        V: AsRef<str>,
    {
        self.set_filter(field.as_ref(), format!("<{}", value.as_ref()))
    }

    /// Add a filter for less than or equal (e.g., `filter[field]=<=value`).
    #[must_use]
    pub fn filter_lte<F, V>(self, field: F, value: V) -> Self
    where
        F: AsRef<str>,
        V: AsRef<str>,
    {
        self.set_filter(field.as_ref(), format!("<={}", value.as_ref()))
    }

    /// Add a sort parameter (e.g., `sort=field`).
    #[must_use]
    pub fn sort<F>(mut self, field: F) -> Self
    where
        F: AsRef<str>,
    {
        self.query
            .insert("sort".to_string(), field.as_ref().to_string());
        self
    }

    /// Set the page number (1-based).
    #[must_use]
    pub fn page_number(mut self, number: usize) -> Self {
        self.query
            .insert("page[number]".to_string(), number.to_string());
        self
    }

    /// Set the page size.
    #[must_use]
    pub fn page_size(mut self, size: usize) -> Self {
        self.query
            .insert("page[size]".to_string(), size.to_string());
        self
    }

    /// Add a generic query parameter.
    #[must_use]
    pub fn param(mut self, key: &str, value: &str) -> Self {
        self.query.insert(key.to_string(), value.to_string());
        self
    }

    /// Private helper to construct the full request URL.
    fn build_url(&self) -> Result<Url> {
        self.client
            .base_url()
            .join(&self.path)
            .map_err(GleifError::UrlParseError)
    }

    /// Private helper to construct the `reqwest_middleware::RequestBuilder` for this request.
    fn build_request(&self, url: Url) -> reqwest_middleware::RequestBuilder {
        self.client
            .client()
            .as_ref()
            .request(self.method.clone(), url)
            .query(&self.query)
    }

    /// Build and execute the request, returning a deserializable response.
    /// The type `R` can be `serde_json::Value` for raw JSON, or a specific
    /// strongly-typed struct (e.g., `GleifApiResponse`<`LeiRecord`>).
    ///
    /// # Errors
    ///
    /// Returns a [`crate::error::GleifError`] if the request fails, the response is not valid JSON
    /// that can be deserialized into `R`, or the URL cannot be constructed.
    pub async fn send<R>(self) -> Result<R>
    where
        R: DeserializeOwned,
    {
        let url = self.build_url()?;
        let req = self.build_request(url);
        let resp = req.send().await.map_err(GleifError::from)?;

        // Attempt to deserialize the response body into the type R
        let parsed_response = resp
            .json::<R>()
            .await
            .map_err(|e| GleifError::from(reqwest_middleware::Error::from(e)))?;
        Ok(parsed_response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::GleifClient;
    use reqwest::Client as ReqwestClient;

    fn test_client() -> GleifClient {
        let reqwest_client = ReqwestClient::new();
        GleifClient::from_reqwest_client(reqwest_client)
    }

    #[test]
    fn test_filter_eq() {
        let builder = GleifRequestBuilder::new(test_client(), Method::GET, "lei-records")
            .filter_eq("field", "value");
        assert_eq!(builder.query.get("filter[field]").unwrap(), "value");
    }

    #[test]
    fn test_filter_not() {
        let builder = GleifRequestBuilder::new(test_client(), Method::GET, "lei-records")
            .filter_not("field", "value");
        assert_eq!(builder.query.get("filter[field]").unwrap(), "!value");
    }

    #[test]
    fn test_filter_in() {
        let builder = GleifRequestBuilder::new(test_client(), Method::GET, "lei-records")
            .filter_in("field", ["a", "b", "c"]);
        assert_eq!(builder.query.get("filter[field]").unwrap(), "a,b,c");
    }

    #[test]
    fn test_filter_not_in() {
        let builder = GleifRequestBuilder::new(test_client(), Method::GET, "lei-records")
            .filter_not_in("field", ["a", "b", "c"]);
        assert_eq!(builder.query.get("filter[field]").unwrap(), "!a,b,c");
    }

    #[test]
    fn test_filter_range() {
        let builder = GleifRequestBuilder::new(test_client(), Method::GET, "lei-records")
            .filter_range("field", "min", "max");
        assert_eq!(builder.query.get("filter[field]").unwrap(), "min..max");
    }

    #[test]
    fn test_filter_gt() {
        let builder = GleifRequestBuilder::new(test_client(), Method::GET, "lei-records")
            .filter_gt("field", "123");
        assert_eq!(builder.query.get("filter[field]").unwrap(), ">123");
    }

    #[test]
    fn test_filter_gte() {
        let builder = GleifRequestBuilder::new(test_client(), Method::GET, "lei-records")
            .filter_gte("field", "123");
        assert_eq!(builder.query.get("filter[field]").unwrap(), ">=123");
    }

    #[test]
    fn test_filter_lt() {
        let builder = GleifRequestBuilder::new(test_client(), Method::GET, "lei-records")
            .filter_lt("field", "123");
        assert_eq!(builder.query.get("filter[field]").unwrap(), "<123");
    }

    #[test]
    fn test_filter_lte() {
        let builder = GleifRequestBuilder::new(test_client(), Method::GET, "lei-records")
            .filter_lte("field", "123");
        assert_eq!(builder.query.get("filter[field]").unwrap(), "<=123");
    }

    #[test]
    fn test_sort_and_pagination_and_param() {
        let builder = GleifRequestBuilder::new(test_client(), Method::GET, "lei-records")
            .sort("field")
            .page_number(2)
            .page_size(10)
            .param("custom", "value");
        assert_eq!(builder.query.get("sort").unwrap(), "field");
        assert_eq!(builder.query.get("page[number]").unwrap(), "2");
        assert_eq!(builder.query.get("page[size]").unwrap(), "10");
        assert_eq!(builder.query.get("custom").unwrap(), "value");
    }
}
