//! Google API Discovery document parsing.

use gnostic_compiler::{Context, ErrorGroup, read_info_from_bytes, read_bytes_for_file};
use std::sync::Arc;
use serde_yaml::Value as Yaml;

use crate::discovery::Document;
use crate::parser::Parser;

/// Parses a Discovery document from JSON bytes.
pub fn parse_document(bytes: &[u8]) -> Result<Document, ErrorGroup> {
    let yaml = read_info_from_bytes("", bytes)
        .map_err(|e| ErrorGroup::new(vec![e.into()]))?;

    let node = if let Yaml::Sequence(ref content) = yaml {
        if content.len() == 1 {
            &content[0]
        } else {
            &yaml
        }
    } else {
        &yaml
    };

    let context = Arc::new(Context::root("$"));
    Parser::parse_document(node, &context)
}

/// Parses a Discovery document from a file path or URL.
pub fn parse_document_from_file(path: &str) -> Result<Document, ErrorGroup> {
    let bytes = read_bytes_for_file(path)
        .map_err(|e| ErrorGroup::new(vec![e.into()]))?;
    parse_document(&bytes)
}
