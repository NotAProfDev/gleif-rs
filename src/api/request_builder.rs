use crate::api::{client::ApiClient, error::ApiError};
use log::{debug, error, info};
use reqwest::Method;
use serde::Deserialize;
use std::collections::HashMap;

pub struct RequestBuilder<'a> {
    api_client: &'a ApiClient,
    endpoint: &'a str,
    method: Method,
    query_params: HashMap<String, String>,
}

impl<'a> RequestBuilder<'a> {
    /// Creates a new `RequestBuilder`.
    pub fn new(api_client: &'a ApiClient, endpoint: &'a str, method: Method) -> Self {
        debug!(
            "Creating new request builder for endpoint: {} with method {}",
            endpoint, method
        );
        RequestBuilder {
            api_client,
            endpoint,
            method,
            query_params: HashMap::new(),
        }
    }

    /// Adds a query parameter to the request.
    pub fn filter(mut self, key: &str, value: &str) -> Self {
        debug!("Adding filter: {}={}", key, value);
        self.query_params.insert(key.to_owned(), value.to_owned());
        self
    }

    /// Sends the request asynchronously and returns the deserialized response.
    pub async fn send<T: for<'de> Deserialize<'de>>(&self) -> Result<T, ApiError> {
        let url = self
            .api_client
            .base_url()
            .join(self.endpoint)
            .map_err(|e| {
                error!("Invalid URL: {}", e);
                ApiError::InvalidUrl(e)
            })?;

        info!(
            "Sending {} request to URL: {} with filter {:?}",
            self.method, url, self.query_params
        );
        let mut request = self.api_client.client.request(self.method.clone(), url);

        if !self.query_params.is_empty() {
            request = request.query(&self.query_params);
        }

        let response = request.send().await.map_err(|e| {
            error!("Request error: {}", e);
            ApiError::RequestError(e)
        })?;

        info!(
            "Received response from {} with status: {}: {:?}",
            response.url(),
            response.status(),
            response
        );

        if response.status().is_success() {
            if response
                .headers()
                .get(reqwest::header::CONTENT_TYPE)
                .map_or(false, |v| v == "application/json")
            {
                let json = response.json::<T>().await.map_err(|e| {
                    error!("Deserialization error: {}", e);
                    ApiError::DeserializationError(e)
                })?;
                Ok(json)
            } else {
                Err(ApiError::ResponseTypeError(
                    response
                        .headers()
                        .get(reqwest::header::CONTENT_TYPE)
                        .map_or("No content type", |v| v.to_str().unwrap())
                        .to_owned(),
                ))
            }
        } else {
            Err(ApiError::RequestError(
                response.error_for_status().unwrap_err(),
            ))
        }
    }
}
