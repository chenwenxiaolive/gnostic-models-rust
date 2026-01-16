//! JSON Schema reader.

use crate::models::Schema;

/// Parses a JSON Schema from a JSON string.
pub fn read_schema_from_json(json: &str) -> Result<Schema, serde_json::Error> {
    serde_json::from_str(json)
}

/// Parses a JSON Schema from a YAML string.
pub fn read_schema_from_yaml(yaml: &str) -> Result<Schema, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Parses a JSON Schema from bytes (auto-detects JSON or YAML).
pub fn read_schema(bytes: &[u8]) -> Result<Schema, String> {
    let content = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;

    // Try JSON first
    if let Ok(schema) = read_schema_from_json(content) {
        return Ok(schema);
    }

    // Fall back to YAML
    read_schema_from_yaml(content).map_err(|e| e.to_string())
}
