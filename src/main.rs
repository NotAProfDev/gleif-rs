mod api;
mod model;

use api::client::ApiClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ApiClient::new("https://leilookup.gleif.org/api/v2");
    // let response = client.get_data_by_lei("254900OPPU84GM83MG36").await?;
    // println!("{:?}", response);
    Ok(())
}
