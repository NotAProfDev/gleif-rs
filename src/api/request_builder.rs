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
            "Sending {} request to URL: {} with query parameters: {:?}",
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
            let content_type = response
                .headers()
                .get(reqwest::header::CONTENT_TYPE)
                .map_or_else(
                    || "No content type".to_string(),
                    |v| v.to_str().unwrap_or("Invalid content type").to_string(),
                );

            if content_type == "application/json" {
                let json = response.json::<T>().await.map_err(|e| {
                    error!("Deserialization error: {}", e);
                    ApiError::DeserializationError(e)
                })?;
                Ok(json)
            } else {
                Err(ApiError::ResponseTypeError(content_type))
            }
        } else {
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

    #[derive(Deserialize, Debug, PartialEq)]
    struct TestResponse {
        message: String,
    }

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

        let api_client = ApiClient::new(&url).expect("Failed to create ApiClient");
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

        let api_client = ApiClient::new(&url).expect("Failed to create ApiClient");
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

    /// Tests the addition of a query parameter to the RequestBuilder.
    #[test]
    fn test_request_builder_filter() {
        let setup = setup();

        let mut request_builder =
            RequestBuilder::new(&setup.api_client, setup.endpoint, setup.method.clone());
        request_builder = request_builder.filter("key", "value");

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
