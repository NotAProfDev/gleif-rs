use crate::api::{client::ApiClient, error::ApiError};
use log::{error, info};
use reqwest::Method;
use serde::Deserialize;
use std::collections::HashMap;

pub struct RequestBuilder<'a> {
    api_client: &'a ApiClient,
    endpoint: String,
    method: Method,
    query_params: HashMap<String, String>,
    body: Option<String>, // For future use - sending data
}

impl<'a> RequestBuilder<'a> {
    pub fn new(api_client: &'a ApiClient, endpoint: &str, method: Method) -> Self {
        RequestBuilder {
            api_client,
            endpoint: endpoint.to_owned(),
            method,
            query_params: HashMap::new(),
            body: None,
        }
    }

    pub fn filter(mut self, key: &str, value: &str) -> Self {
        self.query_params.insert(key.to_owned(), value.to_owned());
        self
    }

    pub fn set_body(mut self, body: &'a str) -> Self {
        self.body = Some(body.to_owned());
        self
    }

    pub async fn send<T: for<'de> Deserialize<'de>>(self) -> Result<T, ApiError> {
        let url = self
            .api_client
            .base_url()
            .join(&self.endpoint)
            .map_err(ApiError::InvalidUrl)?;

        info!("Sending request to URL: {}", url);
        let mut request = self.api_client.client.request(self.method, url.clone());

        if !self.query_params.is_empty() {
            request = request.query(&self.query_params);
        }

        if let Some(body) = self.body {
            request = request.body(body);
        }

        let response = request.send().await.map_err(|e| {
            error!("Request error: {}", e);
            ApiError::RequestError(e)
        })?;

        info!("Received response: {:?}", response);

        let json = response.json::<T>().await.map_err(|e| {
            error!("Deserialization error: {}", e);
            ApiError::DeserializationError(e)
        })?;

        Ok(json)
    }
}
