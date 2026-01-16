//! OpenAPI v3 support for gnostic-models.
//!
//! This crate provides Protocol Buffer models and parsing for OpenAPI v3 specifications.

pub mod parser;
pub mod document;

/// Generated Protocol Buffer code for OpenAPI v3.
pub mod openapi_v3 {
    include!(concat!(env!("OUT_DIR"), "/openapi.v3.rs"));
}

pub use document::*;
pub use openapi_v3::Document;
