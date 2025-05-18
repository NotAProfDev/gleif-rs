# gleif-rs

[![Build Status](https://github.com/NotAProfDev/gleif-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/NotAProfDev/gleif-rs/actions)
[![Crates.io](https://img.shields.io/crates/v/gleif-rs.svg)](https://crates.io/crates/gleif-rs)
[![Docs.rs](https://docs.rs/gleif-rs/badge.svg)](https://docs.rs/gleif-rs)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

A modern, strongly-typed Rust client for the [GLEIF API](https://www.gleif.org/en/lei-data/gleif-api), offering ergonomic access to Legal Entity Identifier (LEI) records and related resources.

See the [full API documentation on docs.rs](https://docs.rs/gleif-rs/).

## Features

- **Flexible request building**: Use a builder-pattern API to apply filters, sorting, and pagination.
- **Strongly-typed models**: Access GLEIF endpoints with type safety.
- **Async/await support**: Built on [`reqwest`](https://docs.rs/reqwest) for modern asynchronous programming.
- **Customizable middleware**: Extend functionality with [`reqwest_middleware`](https://docs.rs/reqwest-middleware/), enabling features like retries, rate limiting, logging, or tracing. For example, integrate [`reqwest-retry`](https://docs.rs/reqwest-retry/) for automatic retries or implement custom logic.
- **Dual response formats**: Choose between raw JSON or strongly-typed responses.
- **Robust error handling**: Comprehensive error management for all operations.
- **Real-world testing**: Includes a test suite with real-world data samples.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
gleif-rs = "0.1"
```

Alternatively, you can add it using:

```shell
cargo add gleif-rs
```

## Usage

### Basic Example

This example shows how to use `gleif-rs` to fetch and filter Legal Entity Identifier (LEI) records. It retrieves a single LEI record by querying an entity and displaying its LEI and legal name. You can explore the available information for a LEI record [here](https://search.gleif.org/#/record/5493001KJTIIGC8Y1R12). It also fetches multiple LEI records with filters, selecting Funds with an Issued status, sorting by Entity Legal Name, and limiting results to three, displaying them in JSON format.

```rust
use gleif_rs::{
    client::GleifClient,
    error::GleifError,
    field::Field,
    model::LeiRecord,
    value::{EntityCategory, RegistrationStatus},
};

#[tokio::main]
async fn main() -> Result<(), GleifError> {
    // Setup default client
    let client = GleifClient::new();
    let lei = "5493001KJTIIGC8Y1R12";

    // Fetch a single LEI record (strongly typed)
    let record: LeiRecord = client.lei_record_by_id(lei).await?;
    println!(
        "LEI: {} Legal Name: {}",
        record.data.attributes.lei, record.data.attributes.entity.legal_name.name
    );

    // Fetch multiple LEI records with filters (as JSON)
    let records: serde_json::Value = client
        .lei_records()
        .filter_eq(Field::EntityCategory, EntityCategory::Fund)
        .filter_eq(Field::RegistrationStatus, RegistrationStatus::Issued)
        .sort(Field::EntityLegalName)
        .page_size(3)
        .send()
        .await?;
    println!("Records: {records:#?}");

    Ok(())
}
```

### Adding a Retry Policy via `reqwest-middleware`

This section explains how to add middleware via `reqwest-middleware` for improved request handling. Using `reqwest-retry` as an example, it demonstrates how to apply a retry policy with exponential backoff, making API requests more resilient to temporary failures.

```rust
use gleif_rs::{client::GleifClient, error::GleifError};
use reqwest::Client as ReqwestClient;
use reqwest_middleware::ClientBuilder;
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};

#[tokio::main]
async fn main() -> Result<(), GleifError> {
    // Create a basic Reqwest HTTP client
    let reqwest_client = ReqwestClient::new();
    // Define an exponential backoff retry policy with a maximum of 3 retries
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    // Wrap the Reqwest client with middleware that applies the retry policy
    let middleware_client = ClientBuilder::new(reqwest_client)
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();
    // Initialize the GleifClient using the middleware-enabled HTTP client
    let client = GleifClient::from_middleware_client(middleware_client);
    Ok(())
}
```

Stack any middleware supported by `reqwest_middleware` to customize request behavior for your use case.

## Contributing

We welcome contributions! To get started, please check out our [CONTRIBUTING.md](CONTRIBUTING.md), which outlines the guidelines and best practices for contributing to this project.

If you have ideas, bug reports, or feature requests, feel free to open an issue or submit a pull request. Whether it's fixing a bug, improving documentation, or suggesting enhancements, all contributions are appreciated!

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

## Acknowledgments

- [GLEIF](https://www.gleif.org/) for the API and data
- [reqwest](https://github.com/seanmonstar/reqwest) and [tokio](https://tokio.rs/) for async HTTP support
- [reqwest_middleware](https://docs.rs/reqwest-middleware/) for extensibility and middleware support
