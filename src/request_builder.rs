//! Generic request builder for the GLEIF API client.

use crate::{
    client::GleifClient,
    error::{GleifError, Result},
};
use futures_core::stream::Stream;
use futures_util::stream::unfold;
use reqwest::Method;
use serde_json::Value;
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
    pub fn new(client: GleifClient, method: Method, path: impl Into<String>) -> Self {
        Self {
            client,
            method,
            path: path.into(),
            query: HashMap::new(),
        }
    }

    /// Returns the current path for this request builder.
    pub fn get_path(&self) -> &str {
        &self.path
    }

    /// Returns a reference to the query parameters for this request builder.
    pub fn get_query(&self) -> &HashMap<String, String> {
        &self.query
    }

    /// Private helper to insert a filter parameter with the given operator syntax.
    fn set_filter(mut self, field: &str, value: String) -> Self {
        self.query.insert(format!("filter[{}]", field), value);
        self
    }

    /// Add a filter for exact match (e.g., filter[field]=value).
    pub fn filter_eq(self, field: &str, value: &str) -> Self {
        self.set_filter(field, value.to_string())
    }

    /// Add a filter for NOT match (e.g., filter[field]=!value).
    pub fn filter_not(self, field: &str, value: &str) -> Self {
        self.set_filter(field, format!("!{}", value))
    }

    /// Add a filter for IN (comma-separated values, e.g., filter[field]=a,b,c).
    pub fn filter_in<I, S>(self, field: &str, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = values
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<String>>()
            .join(",");
        self.set_filter(field, joined)
    }

    /// Add a filter for NOT IN (e.g., filter[field]=!a,b,c).
    pub fn filter_not_in<I, S>(self, field: &str, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = values
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<String>>()
            .join(",");
        self.set_filter(field, format!("!{}", joined))
    }

    /// Add a filter for range (e.g., filter[field]=min..max).
    pub fn filter_range(self, field: &str, min: &str, max: &str) -> Self {
        self.set_filter(field, format!("{}..{}", min, max))
    }

    /// Add a filter for greater than (e.g., filter[field]=>value).
    pub fn filter_gt(self, field: &str, value: &str) -> Self {
        self.set_filter(field, format!(">{}", value))
    }

    /// Add a filter for greater than or equal (e.g., filter[field]=>=value).
    pub fn filter_gte(self, field: &str, value: &str) -> Self {
        self.set_filter(field, format!(">={}", value))
    }

    /// Add a filter for less than (e.g., filter[field]=<value).
    pub fn filter_lt(self, field: &str, value: &str) -> Self {
        self.set_filter(field, format!("<{}", value))
    }

    /// Add a filter for less than or equal (e.g., filter[field]=<=value).
    pub fn filter_lte(self, field: &str, value: &str) -> Self {
        self.set_filter(field, format!("<={}", value))
    }

    /// Add a sort parameter (e.g., sort=field).
    pub fn sort(mut self, field: &str) -> Self {
        self.query.insert("sort".to_string(), field.to_string());
        self
    }

    /// Set the page number (1-based).
    pub fn page_number(mut self, number: usize) -> Self {
        self.query
            .insert("page[number]".to_string(), number.to_string());
        self
    }

    /// Set the page size.
    pub fn page_size(mut self, size: usize) -> Self {
        self.query
            .insert("page[size]".to_string(), size.to_string());
        self
    }

    /// Add a generic query parameter.
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

    /// Private helper to construct the reqwest_middleware::RequestBuilder for this request.
    fn build_request(&self, url: Url) -> reqwest_middleware::RequestBuilder {
        self.client
            .client()
            .as_ref()
            .request(self.method.clone(), url)
            .query(&self.query)
    }

    /// Build and execute the request, returning the raw JSON response.
    pub async fn send(self) -> Result<Value> {
        let url = self.build_url()?;
        let req = self.build_request(url);
        let resp = req.send().await.map_err(GleifError::from)?;
        let json = resp
            .json::<Value>()
            .await
            .map_err(|e| GleifError::from(reqwest_middleware::Error::from(e)))?;
        Ok(json)
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
