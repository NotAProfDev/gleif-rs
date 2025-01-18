//! This module defines the `RequestBuilder` struct and its associated methods for building and sending API requests.
//!
//! The `RequestBuilder` struct provides functionality to create and configure API requests, including setting query parameters,
//! sending the request, and handling the response. It uses the `reqwest` crate for HTTP requests and the `serde` crate for deserialization.

use crate::api::{client::ApiClient, error::ApiError};
use reqwest::Method;
use serde::Deserialize;
use std::collections::HashMap;

/// The `RequestBuilder` struct provides functionality to create and configure API requests,
/// including setting query parameters, sending the request, and handling the response.
pub struct RequestBuilder<'a> {
    api_client: &'a ApiClient,
    endpoint: &'a str,
    method: Method,
    query_params: HashMap<String, String>,
}

impl<'a> RequestBuilder<'a> {
    /// Creates a new `RequestBuilder`.
    pub fn new(api_client: &'a ApiClient, endpoint: &'a str, method: Method) -> Self {
        log::debug!(
            "Creating new request builder for endpoint: {} with method {}",
            endpoint,
            method
        );
        RequestBuilder {
            api_client,
            endpoint,
            method,
            query_params: HashMap::new(),
        }
    }

    /// Returns a reference to the `ApiClient`.
    pub fn api_client(&self) -> &ApiClient {
        self.api_client
    }

    /// Returns the API endpoint.
    pub fn endpoint(&self) -> &str {
        self.endpoint
    }

    /// Returns the HTTP method.
    pub fn method(&self) -> &Method {
        &self.method
    }

    /// Returns a reference to the query parameters.
    pub fn query_params(&self) -> &HashMap<String, String> {
        &self.query_params
    }

    /// Adds a query parameter to the request.
    pub fn filter(mut self, key: &str, value: &str) -> Self {
        log::debug!("Adding filter: {}={}", key, value);
        self.query_params.insert(key.to_owned(), value.to_owned());
        self
    }

    /// Sends the request asynchronously and returns the deserialized response.
    pub async fn send<T: for<'de> Deserialize<'de>>(&self) -> Result<T, ApiError> {
        // Throttle the request to respect the rate limit
        self.api_client.throttler().throttle().await;

        let url = self
            .api_client
            .base_url()
            .join(self.endpoint)
            .map_err(|e| {
                log::error!("Invalid URL: {}", e);
                ApiError::InvalidUrl(e)
            })?;

        log::info!(
            "Sending {} request to URL: {} with query parameters: {:?}",
            self.method,
            url,
            self.query_params
        );
        let mut request = self.api_client.client().request(self.method.clone(), url);

        if !self.query_params.is_empty() {
            request = request.query(&self.query_params);
        }

        let response = request.send().await.map_err(|e| {
            log::error!("Request error: {}", e);
            ApiError::RequestError(e)
        })?;

        log::info!(
            "Received response from {} with status: {}: {:?}",
            response.url(),
            response.status(),
            response
        );

        if response.status().is_success() {
            let content_type = response
                .headers()
                .get(reqwest::header::CONTENT_TYPE)
                .map_or_else(
                    || "No content type".to_string(),
                    |v| v.to_str().unwrap_or("Invalid content type").to_string(),
                );

            if content_type == "application/json" || content_type == "application/vnd.api+json" {
                let response_text = response.text().await.map_err(|e| {
                    log::error!("Failed to read response text: {}", e);
                    ApiError::ResponseError(e)
                })?;

                let json = serde_json::from_str::<T>(&response_text).map_err(|e| {
                    log::error!(
                        "Deserialization error: {}. Response text: {}",
                        e,
                        response_text
                    );
                    ApiError::DeserializationError(e)
                })?;
                Ok(json)
            } else {
                log::error!("Unexpected content type: {}", content_type);
                Err(ApiError::ResponseTypeError(content_type))
            }
        } else {
            log::error!(
                "Request failed with status: {}. URL: {}",
                response.status(),
                response.url()
            );
            Err(ApiError::RequestError(
                response.error_for_status().unwrap_err(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::client::ApiClient;
    use mockito;
    use reqwest::Method;
    use serde::Deserialize;
    use std::collections::HashMap;

    /// The `TestResponse` struct is used for deserializing the response in the tests.
    #[derive(Deserialize, Debug, PartialEq)]
    struct TestResponse {
        message: String,
    }

    /// The `TestSetup` struct holds the test environment setup, including the `ApiClient`, mock server, endpoint, and HTTP method.
    struct TestSetup {
        api_client: ApiClient,
        endpoint: &'static str,
        method: Method,
        server: mockito::ServerGuard,
    }

    /// Sets up the test environment with a mock server and an ApiClient.
    fn setup() -> TestSetup {
        let server = mockito::Server::new();
        let url = server.url();

        let api_client = ApiClient::new(&url, 10, 60).expect("Failed to create ApiClient");
        let endpoint = "/test";
        let method = Method::GET;

        TestSetup {
            api_client,
            endpoint,
            method,
            server,
        }
    }

    /// Sets up the test environment with a asynchronously mock server and an ApiClient.
    async fn setup_async() -> TestSetup {
        let server = mockito::Server::new_async().await;
        let url = server.url();

        let api_client = ApiClient::new(&url, 10, 60).expect("Failed to create ApiClient");
        let endpoint = "/test";
        let method = Method::GET;

        TestSetup {
            api_client,
            endpoint,
            method,
            server,
        }
    }

    /// Tests the creation of a new RequestBuilder.
    #[test]
    fn test_request_builder_new() {
        let setup = setup();

        let request_builder =
            RequestBuilder::new(&setup.api_client, setup.endpoint, setup.method.clone());

        assert_eq!(
            request_builder.api_client.base_url(),
            setup.api_client.base_url()
        );
        assert_eq!(request_builder.endpoint, setup.endpoint);
        assert_eq!(request_builder.method, setup.method);
        assert_eq!(request_builder.query_params, HashMap::new());
    }

    /// Tests the getter methods of the RequestBuilder.
    #[test]
    fn test_request_builder_getters() {
        let setup = setup();

        let request_builder =
            RequestBuilder::new(&setup.api_client, setup.endpoint, setup.method.clone());

        // Verify that the api_client getter returns the correct ApiClient reference
        assert_eq!(
            request_builder.api_client().base_url(),
            setup.api_client.base_url()
        );

        // Verify that the endpoint getter returns the correct endpoint
        assert_eq!(request_builder.endpoint(), setup.endpoint);

        // Verify that the method getter returns the correct HTTP method
        assert_eq!(request_builder.method(), &setup.method);

        // Verify that the query_params getter returns an empty HashMap
        assert_eq!(request_builder.query_params(), &HashMap::new());
    }

    /// Tests the addition of a query parameter to the RequestBuilder.
    #[test]
    fn test_request_builder_filter() {
        let setup = setup();

        let request_builder =
            RequestBuilder::new(&setup.api_client, setup.endpoint, setup.method.clone())
                .filter("key", "value");

        let mut expected_query_params = HashMap::new();
        expected_query_params.insert("key".to_string(), "value".to_string());

        assert_eq!(request_builder.query_params, expected_query_params);
    }

    /// Tests sending a request successfully and receiving a valid response.
    #[tokio::test]
    async fn test_request_builder_send_success() {
        let mut setup = setup_async().await;

        let _m = setup
            .server
            .mock("GET", "/test")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"message": "success"}"#)
            .create();

        let request_builder =
            RequestBuilder::new(&setup.api_client, setup.endpoint, setup.method.clone());
        let response: TestResponse = request_builder
            .send()
            .await
            .expect("Failed to send request");

        assert_eq!(
            response,
            TestResponse {
                message: "success".to_string()
            }
        );
    }

    /// Tests sending a request that results in a server error.
    #[tokio::test]
    async fn test_request_builder_send_request_error() {
        let mut setup = setup_async().await;

        let _m = setup.server.mock("GET", "/test").with_status(500).create();

        let request_builder =
            RequestBuilder::new(&setup.api_client, setup.endpoint, setup.method.clone());
        let result: Result<TestResponse, ApiError> = request_builder.send().await;

        assert!(matches!(result, Err(ApiError::RequestError(_))));
    }

    /// Tests sending a request that results in a deserialization error.
    #[tokio::test]
    async fn test_request_builder_send_deserialization_error() {
        let mut setup = setup_async().await;

        let _m = setup
            .server
            .mock("GET", "/test")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"invalid": "response"}"#)
            .create();

        let request_builder =
            RequestBuilder::new(&setup.api_client, setup.endpoint, setup.method.clone());
        let result: Result<TestResponse, ApiError> = request_builder.send().await;

        assert!(matches!(result, Err(ApiError::DeserializationError(_))));
    }

    /// Tests sending a request that results in a response type error.
    #[tokio::test]
    async fn test_request_builder_send_response_type_error() {
        let mut setup = setup_async().await;

        let _m = setup
            .server
            .mock("GET", "/test")
            .with_status(200)
            .with_header("content-type", "text/plain")
            .with_body("Not JSON")
            .create();

        let request_builder =
            RequestBuilder::new(&setup.api_client, setup.endpoint, setup.method.clone());
        let result: Result<TestResponse, ApiError> = request_builder.send().await;

        assert!(matches!(result, Err(ApiError::ResponseTypeError(_))));
    }
}
