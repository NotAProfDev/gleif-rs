use std::time::Duration;

use crate::api::{error::ApiError, request_builder::RequestBuilder};
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, USER_AGENT},
    Client, Method, Url,
};

const TIMEOUT_DURATION: Duration = Duration::from_secs(30);

pub struct ApiClient {
    pub client: Client,
    base_url: Url,
    headers: HeaderMap,
    timeout: Duration,
}

impl ApiClient {
    pub fn new(base_url: &str) -> Result<Self, ApiError> {
        let client = Client::builder()
            .timeout(TIMEOUT_DURATION)
            .build()
            .map_err(ApiError::ClientBuildError)?;
        let base_url = Url::parse(base_url).map_err(ApiError::InvalidUrl)?;
        Ok(ApiClient {
            client,
            base_url,
            headers: HeaderMap::new(),
            timeout: TIMEOUT_DURATION,
        })
    }

    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    pub fn set_header(&mut self, key: &str, value: &str) {
        self.headers.insert(
            HeaderName::from_bytes(key.as_bytes()).expect("Invalid header name"),
            HeaderValue::from_str(value).expect("Invalid header value"),
        );
    }

    pub fn set_user_agent(&mut self, user_agent: &str) {
        self.headers.insert(
            USER_AGENT,
            HeaderValue::from_str(user_agent).expect("Invalid header value"),
        );
    }

    pub fn set_timeout(&mut self, seconds: u64) {
        let timeout = Duration::from_secs(seconds);
        self.timeout = timeout;
    }

    pub fn build_request(&self, endpoint: &str, method: Method) -> RequestBuilder {
        RequestBuilder::new(self, endpoint, method)
    }
}
