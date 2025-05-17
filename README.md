# gleif-rs

[![Build Status](https://github.com/NotAProfDev/gleif-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/NotAProfDev/gleif-rs/actions)
[![Crates.io](https://img.shields.io/crates/v/gleif-rs.svg)](https://crates.io/crates/gleif-rs)
[![Docs.rs](https://docs.rs/gleif-rs/badge.svg)](https://docs.rs/gleif-rs)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

A modern, strongly-typed Rust client for the [GLEIF API](https://www.gleif.org/en/lei-data/gleif-api), offering ergonomic access to Legal Entity Identifier (LEI) records and related resources.

## Features

- **Flexible request building**: Use a builder-pattern API to apply filters, sorting, and pagination.
- **Strongly-typed models**: Access GLEIF endpoints with type safety.
- **Async/await support**: Built on `reqwest` for modern asynchronous programming.
- **Customizable middleware**: Extend functionality with [`reqwest_middleware`](https://docs.rs/reqwest-middleware/), enabling features like retries, rate limiting, logging, or tracing. For example, integrate [`reqwest-retry`](https://docs.rs/reqwest-retry/) for automatic retries or implement custom logic.
- **Dual response formats**: Choose between raw JSON or strongly-typed responses.
- **Robust error handling**: Comprehensive error management for all operations.
- **Real-world testing**: Includes a test suite with real-world data samples.

## Example Usage

### Basic Example

```rust
use gleif_rs::{client::GleifClient, field::Field, value::{EntityCategory, RegistrationStatus}};

#[tokio::main]
async fn main() -> Result<(), gleif_rs::error::GleifError> {
    let client = GleifClient::new();
    let lei = "5493001KJTIIGC8Y1R12";

    // Fetch a single LEI record (strongly typed)
    let record = client.lei_record_by_id::<gleif_rs::model::lei_record::LeiRecord>(lei).await?;
    println!("LEI: {} Legal Name: {}", record.data.attributes.lei, record.data.attributes.entity.legal_name.name);

    // Fetch multiple LEI records with filters (as JSON)
    let records: serde_json::Value = client
        .lei_records()
        .filter_eq(Field::EntityCategory, EntityCategory::Fund)
        .filter_eq(Field::RegistrationStatus, RegistrationStatus::Issued)
        .sort(Field::EntityLegalName)
        .page_size(3)
        .send()
        .await?;
    println!("Records: {:#?}", records);

    Ok(())
}
```

### Adding a Retry Policy

```rust
use gleif_rs::client::GleifClient;
use reqwest::Client as ReqwestClient;
use reqwest_middleware::{ClientBuilder, Result};
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};

let reqwest_client = ReqwestClient::new();
let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
let middleware_client = ClientBuilder::new(reqwest_client)
    .with(RetryTransientMiddleware::new_with_policy(retry_policy))
    .build();
let client = GleifClient::from_reqwest_client(middleware_client);
```

Stack any middleware supported by `reqwest_middleware` to customize request behavior for your use case.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
gleif-rs = "0.1"
```

## Testing

Run all tests with:

```shell
cargo test
```

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

## Acknowledgments

- [GLEIF](https://www.gleif.org/) for the API and data
- [reqwest](https://github.com/seanmonstar/reqwest) and [tokio](https://tokio.rs/) for async HTTP support
- [reqwest_middleware](https://docs.rs/reqwest-middleware/) for extensibility and middleware support
