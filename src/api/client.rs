use crate::api::error::GleifError;
use crate::model::api_response::ApiResponse;
use reqwest::{Client, StatusCode};

pub struct GleifClient {
    client: Client,
    base_url: String,
}

impl GleifClient {
    pub fn new() -> Self {
        GleifClient {
            client: Client::new(),
            base_url: "https://api.gleif.org/api/v1/".to_string(),
        }
    }

    async fn send(&self, url: &str) -> Result<ApiResponse, GleifError> {
        // Send the request and map any network errors to GleifError::NetworkError
        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(GleifError::NetworkError)?;

        // Match on the status code of the response
        match response.status() {
            StatusCode::OK => {
                // Deserialize the response body into ApiResponse and map any deserialization errors
                let api_response = response
                    .json::<ApiResponse>()
                    .await
                    .map_err(GleifError::DeserializationError)?;
                Ok(api_response)
            }
            status_code => {
                // Return an error if the status code is not OK
                Err(GleifError::RequestFailed(status_code))
            }
        }
    }

    pub async fn get_info_by_lei(&self, lei: &str) -> Result<ApiResponse, GleifError> {
        let url = format!("{}lei-records/{}", self.base_url, lei);
        let result = self.send(&url).await?;
        Ok(result)
    }
}
