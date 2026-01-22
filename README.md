# gnostic-models-rust

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)

Rust implementation of [gnostic-models](https://github.com/google/gnostic-models) - Protocol Buffer models for OpenAPI and related formats.

## Overview

This project provides Rust libraries for parsing and working with:

- **OpenAPI v3** (OpenAPI Specification 3.0.x)
- **OpenAPI v2** (Swagger 2.0)
- **Google API Discovery** format

The implementation uses [prost](https://github.com/tokio-rs/prost) for Protocol Buffer code generation, maintaining compatibility with the original Go implementation.

## Crates

| Crate | Description |
|-------|-------------|
| `gnostic-compiler` | Core compiler support library (context, error handling, YAML helpers, file reading) |
| `gnostic-extensions` | Extension protocol (prost generated from extension.proto) |
| `gnostic-jsonschema` | JSON Schema Draft 4 support |
| `gnostic-openapiv3` | OpenAPI v3 parsing and Protocol Buffer types |
| `gnostic-openapiv2` | OpenAPI v2 (Swagger) parsing and Protocol Buffer types |
| `gnostic-discovery` | Google API Discovery format support |

## Installation

Add the desired crate to your `Cargo.toml`:

```toml
[dependencies]
gnostic-openapiv3 = { git = "https://github.com/chenwenxiaolive/gnostic-models-rust" }
# or
gnostic-openapiv2 = { git = "https://github.com/chenwenxiaolive/gnostic-models-rust" }
```

## Usage

### Parsing OpenAPI v3

```rust
use gnostic_openapiv3::document::parse_document;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bytes = fs::read("openapi.yaml")?;
    let doc = parse_document(&bytes)?;

    println!("API Title: {}", doc.info.as_ref().map(|i| &i.title).unwrap_or(&String::new()));
    println!("OpenAPI Version: {}", doc.openapi);

    if let Some(paths) = &doc.paths {
        println!("Paths count: {}", paths.path.len());
    }

    Ok(())
}
```

### Parsing OpenAPI v2 (Swagger)

```rust
use gnostic_openapiv2::document::parse_document;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bytes = fs::read("swagger.json")?;
    let doc = parse_document(&bytes)?;

    println!("API Title: {}", doc.info.as_ref().map(|i| &i.title).unwrap_or(&String::new()));
    println!("Swagger Version: {}", doc.swagger);
    println!("Host: {}", doc.host);
    println!("Base Path: {}", doc.base_path);

    Ok(())
}
```

### Parsing from URL

```rust
use gnostic_openapiv3::document::parse_document_from_file;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let doc = parse_document_from_file("https://petstore3.swagger.io/api/v3/openapi.yaml")?;
    println!("Loaded: {}", doc.info.as_ref().map(|i| &i.title).unwrap_or(&String::new()));
    Ok(())
}
```

## Project Structure

```
gnostic-models-rust/
├── Cargo.toml                    # Workspace configuration
├── proto/                        # Protocol Buffer definitions
│   ├── openapiv3.proto
│   ├── openapiv2.proto
│   ├── discovery.proto
│   ├── extension.proto
│   └── google/protobuf/any.proto
├── crates/
│   ├── gnostic-compiler/         # Core library
│   │   └── src/
│   │       ├── context.rs        # Parsing context tracking
│   │       ├── error.rs          # Error types
│   │       ├── helpers.rs        # YAML node utilities
│   │       ├── reader.rs         # File/HTTP reading with cache
│   │       └── extensions.rs     # Extension handler support
│   ├── gnostic-extensions/       # Extension protocol
│   ├── gnostic-jsonschema/       # JSON Schema support
│   ├── gnostic-openapiv3/        # OpenAPI v3
│   ├── gnostic-openapiv2/        # OpenAPI v2
│   └── gnostic-discovery/        # Google Discovery
└── testdata/                     # Test files and references
```

## Dependencies

- **prost** - Protocol Buffer implementation
- **serde_yaml** - YAML parsing
- **serde / serde_json** - JSON serialization
- **hyper** - HTTP client for URL fetching
- **parking_lot** - Thread-safe caching
- **thiserror** - Error handling

## Building

```bash
# Build all crates
cargo build

# Run tests
cargo test

# Build in release mode
cargo build --release

# Check code without building
cargo check
```

## Testing

The project includes integration tests that compare parsing results with the original Go implementation:

```bash
# Run all tests
cargo test

# Run specific crate tests
cargo test -p gnostic-openapiv3
cargo test -p gnostic-openapiv2
cargo test -p gnostic-discovery
```

Test coverage:
- 29 unit tests (gnostic-compiler)
- 10 integration tests (OpenAPI v3, v2, Discovery)

## Compatibility

This implementation aims to be compatible with the Go [gnostic-models](https://github.com/google/gnostic-models) project. Integration tests verify that parsed structures match the Go reference output.

## License

Apache License 2.0 - See [LICENSE](LICENSE) for details.

## Acknowledgments

- Original [gnostic-models](https://github.com/google/gnostic-models) project by Google
- [prost](https://github.com/tokio-rs/prost) for Protocol Buffer support
