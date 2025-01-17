//! This module defines the `ApiClient` struct and its associated methods for interacting with an API.
//!
//! The `ApiClient` struct provides functionality to create and configure an API client, including setting headers,
//! setting the timeout duration, and building requests. It uses the `reqwest` crate for HTTP requests and the `log` crate
//! for logging. The module also defines the `TIMEOUT_DURATION` constant for the default timeout duration.

use std::time::Duration;

use crate::api::throttler::Throttler;
use crate::api::{error::ApiError, request_builder::RequestBuilder};
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, USER_AGENT},
    Client, Method, Url,
};
use std::sync::Arc;

/// The default timeout duration for the `ApiClient`.
const TIMEOUT_DURATION: Duration = Duration::from_secs(30);

/// The `ApiClient` struct provides functionality to create and configure an API client,
/// including setting headers, setting the timeout duration, and building requests.
pub struct ApiClient {
    client: Client,
    base_url: Url,
    headers: HeaderMap,
    timeout: Duration,
    throttler: Arc<Throttler>,
}

impl ApiClient {
    /// Creates a new `ApiClient`.
    pub fn new(base_url: &str, rate_limit: u32, interval: u64) -> Result<Self, ApiError> {
        log::debug!("Creating ApiClient with base URL: {}", base_url);
        let client = Client::builder()
            .timeout(TIMEOUT_DURATION)
            .build()
            .map_err(ApiError::ClientBuildError)?;
        let base_url = Url::parse(base_url).map_err(ApiError::InvalidUrl)?;
        let throttler = Arc::new(Throttler::new(rate_limit, interval));
        Ok(ApiClient {
            client,
            base_url,
            headers: HeaderMap::new(),
            timeout: TIMEOUT_DURATION,
            throttler,
        })
    }

    /// Returns a reference to the `Throttler`.
    pub fn throttler(&self) -> Arc<Throttler> {
        Arc::clone(&self.throttler)
    }

    /// Returns a reference to the `reqwest::Client`.
    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Returns a reference to the base URL.
    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    /// Returns a reference to the headers.
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    /// Returns the timeout duration.
    pub fn timeout(&self) -> Duration {
        self.timeout
    }

    /// Sets a header for the `ApiClient`.
    pub fn set_header(&mut self, key: &str, value: &str) -> Result<(), ApiError> {
        log::debug!("Setting header: {}={}", key, value);
        let header_name = HeaderName::from_bytes(key.as_bytes()).map_err(|e| {
            log::error!("Invalid header name: {}", e);
            ApiError::InvalidHeaderName(e)
        })?;

        let header_value = HeaderValue::from_str(value).map_err(|e| {
            log::error!("Invalid header value: {}", e);
            ApiError::InvalidHeaderValue(e)
        })?;

        self.headers.insert(header_name, header_value);
        Ok(())
    }

    /// Sets the User-Agent header for the `ApiClient`.
    pub fn set_user_agent(&mut self, user_agent: &str) -> Result<(), ApiError> {
        log::debug!("Setting User-Agent header: {}", user_agent);
        let user_agent = HeaderValue::from_str(user_agent).map_err(|e| {
            log::error!("Invalid User-Agent header value: {}", e);
            ApiError::InvalidHeaderValue(e)
        })?;

        self.headers.insert(USER_AGENT, user_agent);
        Ok(())
    }

    /// Sets the timeout duration in seconds for the `ApiClient`.
    pub fn set_timeout(&mut self, seconds: u64) {
        log::debug!("Setting timeout duration to {} seconds", seconds);
        assert!(seconds > 0, "Timeout duration must be greater than zero");
        self.timeout = Duration::from_secs(seconds);
    }

    /// Builds a new `RequestBuilder` for the given endpoint and HTTP method.
    pub fn build_request<'a>(&'a self, endpoint: &'a str, method: &str) -> RequestBuilder<'a> {
        let method = match method.to_uppercase().as_str() {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "DELETE" => Method::DELETE,
            _ => panic!("Unsupported HTTP method: {}", method),
        };
        RequestBuilder::new(self, endpoint, method)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{client::ApiClient, error::ApiError};
    use reqwest::{
        header::{HeaderMap, HeaderName, HeaderValue},
        Method, Url,
    };
    use std::time::Duration;

    const TEST_BASE_URL: &str = "http://example.com";

    // Define a struct to hold the test setup
    struct TestSetup {
        api_client: ApiClient,
    }

    /// Creates a new `TestSetup` with a new `ApiClient`.
    fn setup() -> TestSetup {
        let base_url = TEST_BASE_URL;
        let api_client = ApiClient::new(base_url, 10, 60).expect("Failed to create ApiClient");

        TestSetup { api_client }
    }

    /// Tests that a new `ApiClient` can be created successfully.
    #[test]
    fn test_api_client_new_success() {
        let setup = setup();

        assert_eq!(
            setup.api_client.base_url,
            Url::parse(TEST_BASE_URL).unwrap()
        );
        assert_eq!(setup.api_client.timeout, TIMEOUT_DURATION);
    }

    /// Tests that an error is returned when creating a new `ApiClient` with an invalid URL.
    #[test]
    fn test_api_client_new_invalid_url() {
        let base_url = "invalid_url";
        let result = ApiClient::new(base_url, 10, 60);

        assert!(matches!(result, Err(ApiError::InvalidUrl(_))));
    }

    /// Tests that the base URL of the `ApiClient` is returned correctly.
    #[test]
    fn test_api_client_base_url() {
        let setup = setup();

        assert_eq!(
            setup.api_client.base_url(),
            &Url::parse(TEST_BASE_URL).unwrap()
        );
    }

    /// Tests that the headers of the `ApiClient` are returned correctly.
    #[test]
    fn test_api_client_headers() {
        let setup = setup();

        // Verify that the headers function returns a reference to the HeaderMap
        let headers: &HeaderMap = setup.api_client.headers();
        assert!(headers.is_empty());
    }

    /// Tests that the timeout duration of the `ApiClient` is returned correctly.
    #[test]
    fn test_api_client_timeout() {
        let setup = setup();

        // Verify that the timeout function returns the correct timeout duration
        let timeout: Duration = setup.api_client.timeout();
        assert_eq!(timeout, Duration::from_secs(30));
    }

    /// Tests that a header can be set successfully.
    #[test]
    fn test_api_client_set_header_success() {
        let mut setup = setup();

        let key = "X-Custom-Header";
        let value = "CustomValue";

        setup
            .api_client
            .set_header(key, value)
            .expect("Failed to set header");

        let headers: &HeaderMap = &setup.api_client.headers;
        let header_name = HeaderName::from_bytes(key.as_bytes()).unwrap();
        let header_value = HeaderValue::from_str(value).unwrap();

        assert_eq!(headers.get(&header_name), Some(&header_value));
    }

    /// Tests that an error is returned when setting a header with an invalid name.
    #[test]
    fn test_api_client_set_header_invalid_name() {
        let mut setup = setup();

        let key = "Invalid Header Name";
        let value = "CustomValue";

        let result = setup.api_client.set_header(key, value);

        assert!(matches!(result, Err(ApiError::InvalidHeaderName(_))));
    }

    /// Tests that an error is returned when setting a header with an invalid value.
    #[test]
    fn test_api_client_set_header_invalid_value() {
        let mut setup = setup();

        let key = "X-Custom-Header";
        let value = "\nInvalidValue";

        let result = setup.api_client.set_header(key, value);

        assert!(matches!(result, Err(ApiError::InvalidHeaderValue(_))));
    }

    /// Tests that the User-Agent header can be set successfully.
    #[test]
    fn test_api_client_set_user_agent_success() {
        let mut setup = setup();

        let user_agent = "MyApp/1.0";

        setup
            .api_client
            .set_user_agent(user_agent)
            .expect("Failed to set User-Agent header");

        let headers: &HeaderMap = &setup.api_client.headers;
        let header_value = HeaderValue::from_str(user_agent).unwrap();

        assert_eq!(headers.get(USER_AGENT), Some(&header_value));
    }

    /// Tests that an error is returned when setting an invalid User-Agent header value.
    #[test]
    fn test_api_client_set_user_agent_invalid_value() {
        let mut setup = setup();

        let user_agent = "\nInvalidUserAgent";

        let result = setup.api_client.set_user_agent(user_agent);

        assert!(matches!(result, Err(ApiError::InvalidHeaderValue(_))));
    }

    /// Tests that the timeout duration can be set successfully.
    #[test]
    fn test_api_client_set_timeout_success() {
        let mut setup = setup();

        let timeout_seconds = 60;
        setup.api_client.set_timeout(timeout_seconds);

        assert_eq!(
            setup.api_client.timeout,
            Duration::from_secs(timeout_seconds)
        );
    }

    /// Tests that an error is returned when setting the timeout duration to zero.
    #[test]
    #[should_panic(expected = "Timeout duration must be greater than zero")]
    fn test_api_client_set_timeout_zero() {
        let mut setup = setup();

        setup.api_client.set_timeout(0);
    }

    /// Tests that a `RequestBuilder` can be built successfully.
    #[test]
    fn test_api_client_build_request_success() {
        let setup = setup();

        let endpoint = "/test";
        let method = "GET";

        let request_builder = setup.api_client.build_request(endpoint, method);

        assert_eq!(request_builder.endpoint(), endpoint);
        assert_eq!(request_builder.method(), Method::GET);
    }

    /// Tests that an error is returned when building a `RequestBuilder` with an unsupported HTTP method.
    #[test]
    #[should_panic(expected = "Unsupported HTTP method: PATCH")]
    fn test_api_client_build_request_unsupported_method() {
        let setup = setup();

        let endpoint = "/test";
        let method = "PATCH";

        setup.api_client.build_request(endpoint, method);
    }
}
