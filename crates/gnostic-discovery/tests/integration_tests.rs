//! Integration tests comparing Rust parsing with Go reference output.

use gnostic_discovery::document::parse_document;
use serde_json::Value;
use std::fs;

const TESTDATA_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../testdata");

/// Load the reference JSON from Go's protojson output
fn load_reference(filename: &str) -> Value {
    let path = format!("{}/{}", TESTDATA_DIR, filename);
    let content = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", path, e));
    serde_json::from_str(&content)
        .unwrap_or_else(|e| panic!("Failed to parse {}: {}", path, e))
}

/// Load and parse a Discovery file
fn load_discovery_file(filename: &str) -> Vec<u8> {
    let path = format!("{}/{}", TESTDATA_DIR, filename);
    fs::read(&path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", path, e))
}

#[test]
fn test_discovery_parse_books() {
    let bytes = load_discovery_file("books-discovery.json");
    let doc = parse_document(&bytes).expect("Failed to parse books-discovery.json");
    let reference = load_reference("books-discovery-reference.json");

    // Compare basic fields
    assert_eq!(doc.name, reference["name"].as_str().unwrap_or(""));
    assert_eq!(doc.id, reference["id"].as_str().unwrap_or(""));
    assert_eq!(doc.description, reference["description"].as_str().unwrap_or(""));
    assert_eq!(doc.protocol, reference["protocol"].as_str().unwrap_or(""));
    assert_eq!(doc.base_url, reference["baseUrl"].as_str().unwrap_or(""));
    assert_eq!(doc.service_path, reference["servicePath"].as_str().unwrap_or(""));
    assert_eq!(doc.batch_path, reference["batchPath"].as_str().unwrap_or(""));
}

#[test]
fn test_discovery_basic_fields_present() {
    let bytes = load_discovery_file("books-discovery.json");
    let doc = parse_document(&bytes).expect("Failed to parse books-discovery.json");

    // Verify essential fields are parsed
    assert!(!doc.name.is_empty(), "name should not be empty");
    assert!(!doc.description.is_empty(), "description should not be empty");
    assert!(!doc.protocol.is_empty(), "protocol should not be empty");
    assert!(!doc.base_url.is_empty(), "base_url should not be empty");
}
