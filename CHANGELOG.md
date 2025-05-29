# Changelog

All notable changes to the gleif-rs library will be documented in this file.
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.1](https://github.com/NotAProfDev/gleif-rs/compare/v0.2.0...v0.2.1) - 2025-05-29

### Other

- update dependencies

## [0.2.0](https://github.com/NotAProfDev/gleif-rs/compare/v0.1.0...v0.2.0) - 2025-05-29

### Added

- Enhance serialization for GLEIF API models

### Other

- Standardize error handling by replacing specific error types with a unified ParseError structure
- Update serde field names for consistency in FuzzyCompletion and ReportingException models
- Change data_type field to r#type for consistency across models and responses
- update installation instructions to use only Cargo command
- consistent unordered list symbol
- fix documentation to include crate paths for types and methods
- Enhance documentation across GLEIF API modules
- enhance module documentation
- shorten workflow name and update section header in README

## [0.1.0] - 2025-05-15

### Added

- Initial release: complete, type-safe Rust client for the GLEIF v1 API.
- All major API endpoints implemented: entities, relationships, ISINs, jurisdictions, legal forms, and more.
- Ergonomic builder pattern for client configuration and request construction.
- Strongly-typed enums for fields and values to prevent typos and improve reliability.
- Comprehensive error handling with custom error types.
- Extensive documentation and usage examples.
- Unit tests for core components and helpers.
- Formatted all Rust source files using `rustfmt` for consistent code style and readability.
