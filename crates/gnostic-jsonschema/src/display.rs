//! Display functionality for JSON Schema structures.

use std::fmt;
use crate::models::*;

impl fmt::Display for StringOrStringArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StringOrStringArray::String(s) => write!(f, "{}", s),
            StringOrStringArray::Array(arr) => write!(f, "{}", arr.join(", ")),
        }
    }
}

impl StringOrStringArray {
    /// Returns a string description of the value.
    pub fn description(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Schema {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.describe_schema(""))
    }
}

impl Schema {
    /// Returns a string representation of the schema with the given indentation.
    pub fn describe_schema(&self, indent: &str) -> String {
        let mut result = String::new();
        let next_indent = format!("{}  ", indent);
        let double_indent = format!("{}    ", indent);

        if let Some(ref schema) = self.schema {
            result.push_str(&format!("{}$schema: {}\n", indent, schema));
        }
        if let Some(ref id) = self.id {
            result.push_str(&format!("{}id: {}\n", indent, id));
        }
        if let Some(ref multiple_of) = self.multiple_of {
            result.push_str(&format!("{}multipleOf: {:?}\n", indent, multiple_of));
        }
        if let Some(ref maximum) = self.maximum {
            result.push_str(&format!("{}maximum: {:?}\n", indent, maximum));
        }
        if let Some(ref exclusive_maximum) = self.exclusive_maximum {
            result.push_str(&format!("{}exclusiveMaximum: {}\n", indent, exclusive_maximum));
        }
        if let Some(ref minimum) = self.minimum {
            result.push_str(&format!("{}minimum: {:?}\n", indent, minimum));
        }
        if let Some(ref exclusive_minimum) = self.exclusive_minimum {
            result.push_str(&format!("{}exclusiveMinimum: {}\n", indent, exclusive_minimum));
        }
        if let Some(ref max_length) = self.max_length {
            result.push_str(&format!("{}maxLength: {}\n", indent, max_length));
        }
        if let Some(ref min_length) = self.min_length {
            result.push_str(&format!("{}minLength: {}\n", indent, min_length));
        }
        if let Some(ref pattern) = self.pattern {
            result.push_str(&format!("{}pattern: {}\n", indent, pattern));
        }
        if let Some(ref additional_items) = self.additional_items {
            match additional_items {
                SchemaOrBoolean::Schema(s) => {
                    result.push_str(&format!("{}additionalItems:\n", indent));
                    result.push_str(&s.describe_schema(&next_indent));
                }
                SchemaOrBoolean::Boolean(b) => {
                    result.push_str(&format!("{}additionalItems: {}\n", indent, b));
                }
            }
        }
        if let Some(ref items) = self.items {
            result.push_str(&format!("{}items:\n", indent));
            match items.as_ref() {
                SchemaOrSchemaArray::Schema(s) => {
                    result.push_str(&s.describe_schema(&double_indent));
                }
                SchemaOrSchemaArray::Array(arr) => {
                    for (i, s) in arr.iter().enumerate() {
                        result.push_str(&format!("{}{}:\n", next_indent, i));
                        result.push_str(&s.describe_schema(&double_indent));
                    }
                }
            }
        }
        if let Some(ref max_items) = self.max_items {
            result.push_str(&format!("{}maxItems: {}\n", indent, max_items));
        }
        if let Some(ref min_items) = self.min_items {
            result.push_str(&format!("{}minItems: {}\n", indent, min_items));
        }
        if let Some(ref unique_items) = self.unique_items {
            result.push_str(&format!("{}uniqueItems: {}\n", indent, unique_items));
        }
        if let Some(ref max_properties) = self.max_properties {
            result.push_str(&format!("{}maxProperties: {}\n", indent, max_properties));
        }
        if let Some(ref min_properties) = self.min_properties {
            result.push_str(&format!("{}minProperties: {}\n", indent, min_properties));
        }
        if let Some(ref required) = self.required {
            result.push_str(&format!("{}required: {:?}\n", indent, required));
        }
        if let Some(ref additional_properties) = self.additional_properties {
            match additional_properties {
                SchemaOrBoolean::Schema(s) => {
                    result.push_str(&format!("{}additionalProperties:\n", indent));
                    result.push_str(&s.describe_schema(&next_indent));
                }
                SchemaOrBoolean::Boolean(b) => {
                    result.push_str(&format!("{}additionalProperties: {}\n", indent, b));
                }
            }
        }
        if let Some(ref properties) = self.properties {
            result.push_str(&format!("{}properties:\n", indent));
            for (name, s) in properties {
                result.push_str(&format!("{}{}:\n", next_indent, name));
                result.push_str(&s.describe_schema(&double_indent));
            }
        }
        if let Some(ref pattern_properties) = self.pattern_properties {
            result.push_str(&format!("{}patternProperties:\n", indent));
            for (name, s) in pattern_properties {
                result.push_str(&format!("{}{}:\n", next_indent, name));
                result.push_str(&s.describe_schema(&double_indent));
            }
        }
        if let Some(ref dependencies) = self.dependencies {
            result.push_str(&format!("{}dependencies:\n", indent));
            for (name, dep) in dependencies {
                match dep {
                    SchemaOrStringArray::Schema(s) => {
                        result.push_str(&format!("{}{}:\n", next_indent, name));
                        result.push_str(&s.describe_schema(&double_indent));
                    }
                    SchemaOrStringArray::StringArray(arr) => {
                        result.push_str(&format!("{}{}:\n", next_indent, name));
                        for s in arr {
                            result.push_str(&format!("{}{}\n", double_indent, s));
                        }
                    }
                }
            }
        }
        if let Some(ref enumeration) = self.enumeration {
            result.push_str(&format!("{}enumeration:\n", indent));
            for value in enumeration {
                result.push_str(&format!("{}{}\n", next_indent, value));
            }
        }
        if let Some(ref type_value) = self.type_value {
            result.push_str(&format!("{}type: {}\n", indent, type_value.description()));
        }
        if let Some(ref all_of) = self.all_of {
            result.push_str(&format!("{}allOf:\n", indent));
            for s in all_of {
                result.push_str(&s.describe_schema(&next_indent));
                result.push_str(&format!("{}-\n", indent));
            }
        }
        if let Some(ref any_of) = self.any_of {
            result.push_str(&format!("{}anyOf:\n", indent));
            for s in any_of {
                result.push_str(&s.describe_schema(&next_indent));
                result.push_str(&format!("{}-\n", indent));
            }
        }
        if let Some(ref one_of) = self.one_of {
            result.push_str(&format!("{}oneOf:\n", indent));
            for s in one_of {
                result.push_str(&s.describe_schema(&next_indent));
                result.push_str(&format!("{}-\n", indent));
            }
        }
        if let Some(ref not) = self.not {
            result.push_str(&format!("{}not:\n", indent));
            result.push_str(&not.describe_schema(&next_indent));
        }
        if let Some(ref definitions) = self.definitions {
            result.push_str(&format!("{}definitions:\n", indent));
            for (name, s) in definitions {
                result.push_str(&format!("{}{}:\n", next_indent, name));
                result.push_str(&s.describe_schema(&double_indent));
            }
        }
        if let Some(ref title) = self.title {
            result.push_str(&format!("{}title: {}\n", indent, title));
        }
        if let Some(ref description) = self.description {
            result.push_str(&format!("{}description: {}\n", indent, description));
        }
        if let Some(ref default) = self.default {
            result.push_str(&format!("{}default:\n", indent));
            result.push_str(&format!("{}  {}\n", indent, default));
        }
        if let Some(ref format) = self.format {
            result.push_str(&format!("{}format: {}\n", indent, format));
        }
        if let Some(ref reference) = self.reference {
            result.push_str(&format!("{}$ref: {}\n", indent, reference));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_or_string_array_display() {
        let single = StringOrStringArray::String("string".to_string());
        assert_eq!(single.description(), "string");

        let array = StringOrStringArray::Array(vec!["one".to_string(), "two".to_string()]);
        assert_eq!(array.description(), "one, two");
    }

    #[test]
    fn test_schema_display() {
        let schema = Schema {
            title: Some("Test Schema".to_string()),
            type_value: Some(StringOrStringArray::String("object".to_string())),
            description: Some("A test schema".to_string()),
            ..Default::default()
        };
        let output = schema.to_string();
        assert!(output.contains("title: Test Schema"));
        assert!(output.contains("type: object"));
        assert!(output.contains("description: A test schema"));
    }

    #[test]
    fn test_schema_with_properties() {
        use std::collections::HashMap;

        let mut properties = HashMap::new();
        properties.insert("name".to_string(), Schema {
            type_value: Some(StringOrStringArray::String("string".to_string())),
            ..Default::default()
        });

        let schema = Schema {
            type_value: Some(StringOrStringArray::String("object".to_string())),
            properties: Some(properties),
            ..Default::default()
        };

        let output = schema.to_string();
        assert!(output.contains("properties:"));
        assert!(output.contains("name:"));
    }
}
