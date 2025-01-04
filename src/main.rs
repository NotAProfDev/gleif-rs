mod api;
mod model;

use api::client::GleifClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = GleifClient::new();
    let response = client.get_info_by_lei("254900OPPU84GM83MG36").await?;
    println!("{:?}", response);
    Ok(())
}
