//! OpenAPI v3 document parsing.

use gnostic_compiler::{Context, ErrorGroup, read_info_from_bytes, read_bytes_for_file};
use std::sync::Arc;
use yaml_rust2::Yaml;

use crate::openapi_v3::Document;
use crate::parser::Parser;

/// Parses an OpenAPI v3 document from YAML/JSON bytes.
pub fn parse_document(bytes: &[u8]) -> Result<Document, ErrorGroup> {
    let yaml = read_info_from_bytes("", bytes)
        .map_err(|e| ErrorGroup::new(vec![e.into()]))?;

    // Handle document node wrapper
    let node = if let Yaml::Array(ref content) = yaml {
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

/// Parses an OpenAPI v3 document from a file path or URL.
pub fn parse_document_from_file(path: &str) -> Result<Document, ErrorGroup> {
    let bytes = read_bytes_for_file(path)
        .map_err(|e| ErrorGroup::new(vec![e.into()]))?;
    parse_document(&bytes)
}

/// Converts a Document to YAML bytes.
pub fn yaml_value(_doc: &Document) -> Vec<u8> {
    // This would require implementing ToYaml trait for all types
    // For now, return empty
    Vec::new()
}
