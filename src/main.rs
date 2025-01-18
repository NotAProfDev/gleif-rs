mod api;
mod model;

use api::client::ApiClient;
use log::{debug, LevelFilter};
use model::api_response::ApiResponse;

// TODO: Examples and integration tests and change to lib

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the logger with explicit log level
    env_logger::Builder::from_default_env()
        .filter_level(LevelFilter::Debug)
        .init();

    debug!("Starting the application...");

    let client = ApiClient::new("https://api.gleif.org/api/v1/", 10, 60).unwrap();
    let response: ApiResponse = client.build_request("lei-records", "get").send().await?;

    println!("{:?}", response);
    Ok(())
}
