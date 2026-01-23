//! JSON Schema data structures.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Represents a JSON Schema number (can be integer or float).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchemaNumber {
    Integer(i64),
    Float(f64),
}

/// Represents either a schema or a boolean.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchemaOrBoolean {
    Schema(Box<Schema>),
    Boolean(bool),
}

/// Represents either a string or an array of strings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrStringArray {
    String(String),
    Array(Vec<String>),
}

/// Named schema - a key-value pair for schema definitions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NamedSchema {
    pub name: String,
    pub value: Schema,
}

/// Named schema or string array.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NamedSchemaOrStringArray {
    Schema(NamedSchema),
    StringArray(Vec<String>),
}

/// JSON Schema structure (Draft 4 compatible).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    /// The $schema keyword.
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,

    /// The id keyword.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Reference to another schema.
    #[serde(rename = "$ref", skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,

    /// Title of the schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Description of the schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Default value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,

    /// Multiple of constraint for numbers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_of: Option<SchemaNumber>,

    /// Maximum value for numbers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<SchemaNumber>,

    /// Whether maximum is exclusive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_maximum: Option<bool>,

    /// Minimum value for numbers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<SchemaNumber>,

    /// Whether minimum is exclusive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_minimum: Option<bool>,

    /// Maximum length for strings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i64>,

    /// Minimum length for strings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<i64>,

    /// Pattern for string validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,

    /// Additional items schema for arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_items: Option<SchemaOrBoolean>,

    /// Items schema for arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<SchemaOrSchemaArray>>,

    /// Maximum items for arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i64>,

    /// Minimum items for arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_items: Option<i64>,

    /// Whether array items must be unique.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_items: Option<bool>,

    /// Maximum properties for objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_properties: Option<i64>,

    /// Minimum properties for objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_properties: Option<i64>,

    /// Required properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,

    /// Additional properties schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<SchemaOrBoolean>,

    /// Property definitions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definitions: Option<HashMap<String, Schema>>,

    /// Properties schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, Schema>>,

    /// Pattern properties schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern_properties: Option<HashMap<String, Schema>>,

    /// Dependencies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<HashMap<String, SchemaOrStringArray>>,

    /// Enumeration of allowed values.
    #[serde(rename = "enum", skip_serializing_if = "Option::is_none")]
    pub enumeration: Option<Vec<serde_json::Value>>,

    /// Type constraint.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_value: Option<StringOrStringArray>,

    /// Format hint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,

    /// All of these schemas must match.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_of: Option<Vec<Schema>>,

    /// Any of these schemas must match.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub any_of: Option<Vec<Schema>>,

    /// One of these schemas must match.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_of: Option<Vec<Schema>>,

    /// Schema must not match.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not: Option<Box<Schema>>,
}

/// Represents either a single schema or an array of schemas.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchemaOrSchemaArray {
    Schema(Schema),
    Array(Vec<Schema>),
}

/// Represents either a schema or an array of strings (for dependencies).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchemaOrStringArray {
    Schema(Schema),
    StringArray(Vec<String>),
}

impl Schema {
    /// Creates a new empty schema.
    pub fn new() -> Self {
        Schema::default()
    }

    /// Creates a schema with a type.
    pub fn with_type(type_name: &str) -> Self {
        Schema {
            type_value: Some(StringOrStringArray::String(type_name.to_string())),
            ..Default::default()
        }
    }

    /// Creates a reference schema.
    pub fn reference(ref_path: &str) -> Self {
        Schema {
            reference: Some(ref_path.to_string()),
            ..Default::default()
        }
    }
}
