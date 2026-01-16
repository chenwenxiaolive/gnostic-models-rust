//! JSON Schema writer.

use crate::models::Schema;

/// Writes a schema as JSON.
pub fn write_schema_as_json(schema: &Schema) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(schema)
}

/// Writes a schema as YAML.
pub fn write_schema_as_yaml(schema: &Schema) -> Result<String, serde_yaml::Error> {
    serde_yaml::to_string(schema)
}
