//! OpenAPI v2 (Swagger) support for gnostic-models.
//!
//! This crate provides Protocol Buffer models and parsing for OpenAPI v2/Swagger specifications.

pub mod parser;
pub mod document;

/// Generated Protocol Buffer code for OpenAPI v2.
pub mod openapi_v2 {
    include!(concat!(env!("OUT_DIR"), "/openapi.v2.rs"));
}

pub use document::*;
pub use openapi_v2::Document;
