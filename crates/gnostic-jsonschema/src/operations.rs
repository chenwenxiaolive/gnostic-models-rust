//! JSON Schema operations.

use crate::models::Schema;

/// Checks if a schema is empty (has no constraints).
pub fn is_empty(schema: &Schema) -> bool {
    schema.schema.is_none()
        && schema.id.is_none()
        && schema.reference.is_none()
        && schema.title.is_none()
        && schema.description.is_none()
        && schema.type_value.is_none()
        && schema.properties.is_none()
        && schema.required.is_none()
        && schema.items.is_none()
        && schema.all_of.is_none()
        && schema.any_of.is_none()
        && schema.one_of.is_none()
        && schema.not.is_none()
}

/// Returns the type of a schema as a string.
pub fn type_name(schema: &Schema) -> Option<String> {
    match &schema.type_value {
        Some(crate::models::StringOrStringArray::String(s)) => Some(s.clone()),
        Some(crate::models::StringOrStringArray::Array(arr)) if !arr.is_empty() => {
            Some(arr[0].clone())
        }
        _ => None,
    }
}
