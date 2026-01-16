//! Google API Discovery format support for gnostic-models.
//!
//! This crate provides Protocol Buffer models and parsing for Google API Discovery format.

pub mod parser;
pub mod document;
pub mod list;

/// Generated Protocol Buffer code for Discovery format.
pub mod discovery {
    include!(concat!(env!("OUT_DIR"), "/discovery.v1.rs"));
}

pub use document::*;
pub use list::*;
