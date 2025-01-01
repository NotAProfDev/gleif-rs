mod model;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let b =
        reqwest::get("https://api.gleif.org/api/v1/lei-records?filter[lei]=261700K5E45DJCF5Z735")
            .await?
            .text()
            .await?;
    println!("Got {:?}", b);
    Ok(())
}
