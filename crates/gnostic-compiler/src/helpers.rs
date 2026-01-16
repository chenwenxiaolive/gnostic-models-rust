// Copyright 2017 Google LLC. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Helper functions for YAML node manipulation.

use regex::Regex;
use yaml_rust2::Yaml;

/// Checks if a YAML node is a mapping (map/object).
pub fn is_mapping(node: &Yaml) -> bool {
    matches!(node, Yaml::Hash(_))
}

/// Checks if a YAML node is a sequence (array).
pub fn is_sequence(node: &Yaml) -> bool {
    matches!(node, Yaml::Array(_))
}

/// Checks if a YAML node is a scalar value.
pub fn is_scalar(node: &Yaml) -> bool {
    matches!(
        node,
        Yaml::String(_)
            | Yaml::Integer(_)
            | Yaml::Real(_)
            | Yaml::Boolean(_)
            | Yaml::Null
    )
}

/// Unpacks a YAML node if it's a mapping, returning a reference to the hash.
pub fn unpack_map(node: &Yaml) -> Option<&yaml_rust2::yaml::Hash> {
    match node {
        Yaml::Hash(h) => Some(h),
        _ => None,
    }
}

/// Returns sorted keys from a YAML mapping node.
pub fn sorted_keys_for_map(node: &Yaml) -> Vec<String> {
    let mut keys = Vec::new();
    if let Yaml::Hash(map) = node {
        for key in map.keys() {
            if let Yaml::String(s) = key {
                keys.push(s.clone());
            }
        }
    }
    keys.sort();
    keys
}

/// Checks if a YAML mapping contains a specific key.
pub fn map_has_key(node: &Yaml, key: &str) -> bool {
    if let Yaml::Hash(map) = node {
        map.contains_key(&Yaml::String(key.to_string()))
    } else {
        false
    }
}

/// Gets the value for a specific key from a YAML mapping.
pub fn map_value_for_key<'a>(node: &'a Yaml, key: &str) -> Option<&'a Yaml> {
    if let Yaml::Hash(map) = node {
        map.get(&Yaml::String(key.to_string()))
    } else {
        None
    }
}

/// Gets a sequence node if the node is a sequence.
pub fn sequence_node_for_node(node: &Yaml) -> Option<&Vec<Yaml>> {
    match node {
        Yaml::Array(arr) => Some(arr),
        _ => None,
    }
}

/// Gets a boolean value from a scalar node.
pub fn bool_for_scalar_node(node: &Yaml) -> Option<bool> {
    match node {
        Yaml::Boolean(b) => Some(*b),
        _ => None,
    }
}

/// Gets an integer value from a scalar node.
pub fn int_for_scalar_node(node: &Yaml) -> Option<i64> {
    match node {
        Yaml::Integer(i) => Some(*i),
        _ => None,
    }
}

/// Gets a float value from a scalar node.
pub fn float_for_scalar_node(node: &Yaml) -> Option<f64> {
    match node {
        Yaml::Real(s) => s.parse().ok(),
        Yaml::Integer(i) => Some(*i as f64),
        _ => None,
    }
}

/// Gets a string value from a scalar node.
pub fn string_for_scalar_node(node: &Yaml) -> Option<String> {
    match node {
        Yaml::String(s) => Some(s.clone()),
        Yaml::Integer(i) => Some(i.to_string()),
        Yaml::Real(r) => Some(r.clone()),
        Yaml::Boolean(b) => Some(b.to_string()),
        Yaml::Null => Some(String::new()),
        _ => None,
    }
}

/// Converts a sequence node to an array of strings.
pub fn string_array_for_sequence_node(node: &Yaml) -> Vec<String> {
    let mut strings = Vec::new();
    if let Yaml::Array(arr) = node {
        for item in arr {
            if let Some(s) = string_for_scalar_node(item) {
                strings.push(s);
            }
        }
    }
    strings
}

/// Identifies which keys from a list of required keys are not in a map.
pub fn missing_keys_in_map(node: &Yaml, required_keys: &[&str]) -> Vec<String> {
    let mut missing = Vec::new();
    for key in required_keys {
        if !map_has_key(node, key) {
            missing.push((*key).to_string());
        }
    }
    missing
}

/// Returns keys in a map that don't match a list of allowed keys and patterns.
pub fn invalid_keys_in_map(
    node: &Yaml,
    allowed_keys: &[&str],
    allowed_patterns: &[&Regex],
) -> Vec<String> {
    let mut invalid = Vec::new();
    if let Yaml::Hash(map) = node {
        for key in map.keys() {
            if let Yaml::String(key_str) = key {
                let mut found = false;
                // Check if key matches any allowed key
                for allowed in allowed_keys {
                    if key_str == *allowed {
                        found = true;
                        break;
                    }
                }
                // Check if key matches any allowed pattern
                if !found {
                    for pattern in allowed_patterns {
                        if pattern.is_match(key_str) {
                            found = true;
                            break;
                        }
                    }
                }
                if !found {
                    invalid.push(key_str.clone());
                }
            }
        }
    }
    invalid
}

/// Creates a new null YAML node.
pub fn new_null_node() -> Yaml {
    Yaml::Null
}

/// Creates a new mapping (hash) YAML node.
pub fn new_mapping_node() -> Yaml {
    Yaml::Hash(yaml_rust2::yaml::Hash::new())
}

/// Creates a new sequence (array) YAML node.
pub fn new_sequence_node() -> Yaml {
    Yaml::Array(Vec::new())
}

/// Creates a new string scalar node.
pub fn new_scalar_node_for_string(s: impl Into<String>) -> Yaml {
    Yaml::String(s.into())
}

/// Creates a new sequence node from a string array.
pub fn new_sequence_node_for_string_array(strings: &[String]) -> Yaml {
    Yaml::Array(strings.iter().map(|s| Yaml::String(s.clone())).collect())
}

/// Creates a new boolean scalar node.
pub fn new_scalar_node_for_bool(b: bool) -> Yaml {
    Yaml::Boolean(b)
}

/// Creates a new float scalar node.
pub fn new_scalar_node_for_float(f: f64) -> Yaml {
    Yaml::Real(f.to_string())
}

/// Creates a new integer scalar node.
pub fn new_scalar_node_for_int(i: i64) -> Yaml {
    Yaml::Integer(i)
}

/// Returns "property" or "properties" based on count.
pub fn plural_properties(count: usize) -> &'static str {
    if count == 1 {
        "property"
    } else {
        "properties"
    }
}

/// Checks if a string array contains a specific value.
pub fn string_array_contains_value(array: &[String], value: &str) -> bool {
    array.iter().any(|item| item == value)
}

/// Checks if a string array contains all of the specified values.
pub fn string_array_contains_values(array: &[String], values: &[&str]) -> bool {
    values
        .iter()
        .all(|value| string_array_contains_value(array, value))
}

/// Returns a human-readable representation of a YAML node.
pub fn display(node: &Yaml) -> String {
    match node {
        Yaml::String(s) => format!("{} (string)", s),
        Yaml::Integer(i) => format!("{} (integer)", i),
        Yaml::Real(r) => format!("{} (float)", r),
        Yaml::Boolean(b) => format!("{} (boolean)", b),
        Yaml::Null => "null".to_string(),
        Yaml::Array(_) => "[array]".to_string(),
        Yaml::Hash(_) => "{object}".to_string(),
        _ => format!("{:?}", node),
    }
}

/// Marshals a YAML node to bytes.
pub fn marshal(node: &Yaml) -> Vec<u8> {
    let mut out_str = String::new();
    {
        let mut emitter = yaml_rust2::YamlEmitter::new(&mut out_str);
        let _ = emitter.dump(node);
    }
    out_str.into_bytes()
}

/// Iterates over key-value pairs in a YAML mapping.
pub fn iter_map<F>(node: &Yaml, mut f: F)
where
    F: FnMut(&str, &Yaml),
{
    if let Yaml::Hash(map) = node {
        for (key, value) in map {
            if let Yaml::String(key_str) = key {
                f(key_str, value);
            }
        }
    }
}

/// Iterates over items in a YAML sequence.
pub fn iter_sequence<F>(node: &Yaml, mut f: F)
where
    F: FnMut(usize, &Yaml),
{
    if let Yaml::Array(arr) = node {
        for (i, item) in arr.iter().enumerate() {
            f(i, item);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use yaml_rust2::YamlLoader;

    fn parse_yaml(s: &str) -> Yaml {
        YamlLoader::load_from_str(s).unwrap().remove(0)
    }

    #[test]
    fn test_is_mapping() {
        let yaml = parse_yaml("key: value");
        assert!(is_mapping(&yaml));

        let yaml = parse_yaml("- item");
        assert!(!is_mapping(&yaml));
    }

    #[test]
    fn test_is_sequence() {
        let yaml = parse_yaml("- item\n- item2");
        assert!(is_sequence(&yaml));

        let yaml = parse_yaml("key: value");
        assert!(!is_sequence(&yaml));
    }

    #[test]
    fn test_map_value_for_key() {
        let yaml = parse_yaml("name: test\nvalue: 123");
        let name = map_value_for_key(&yaml, "name");
        assert!(name.is_some());
        assert_eq!(string_for_scalar_node(name.unwrap()), Some("test".to_string()));

        let missing = map_value_for_key(&yaml, "missing");
        assert!(missing.is_none());
    }

    #[test]
    fn test_string_for_scalar_node() {
        let yaml = parse_yaml("test");
        assert_eq!(string_for_scalar_node(&yaml), Some("test".to_string()));

        let yaml = parse_yaml("123");
        assert_eq!(string_for_scalar_node(&yaml), Some("123".to_string()));
    }

    #[test]
    fn test_bool_for_scalar_node() {
        let yaml = parse_yaml("true");
        assert_eq!(bool_for_scalar_node(&yaml), Some(true));

        let yaml = parse_yaml("false");
        assert_eq!(bool_for_scalar_node(&yaml), Some(false));
    }

    #[test]
    fn test_int_for_scalar_node() {
        let yaml = parse_yaml("42");
        assert_eq!(int_for_scalar_node(&yaml), Some(42));
    }

    #[test]
    fn test_float_for_scalar_node() {
        let yaml = parse_yaml("3.14");
        let result = float_for_scalar_node(&yaml);
        assert!(result.is_some());
        assert!((result.unwrap() - 3.14).abs() < 0.001);
    }

    #[test]
    fn test_string_array_for_sequence_node() {
        let yaml = parse_yaml("- a\n- b\n- c");
        let arr = string_array_for_sequence_node(&yaml);
        assert_eq!(arr, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_sorted_keys_for_map() {
        let yaml = parse_yaml("z: 1\na: 2\nm: 3");
        let keys = sorted_keys_for_map(&yaml);
        assert_eq!(keys, vec!["a", "m", "z"]);
    }

    #[test]
    fn test_missing_keys_in_map() {
        let yaml = parse_yaml("a: 1\nb: 2");
        let missing = missing_keys_in_map(&yaml, &["a", "b", "c"]);
        assert_eq!(missing, vec!["c"]);
    }

    #[test]
    fn test_invalid_keys_in_map() {
        let yaml = parse_yaml("valid: 1\nx-ext: 2\ninvalid: 3");
        let pattern = Regex::new(r"^x-").unwrap();
        let invalid = invalid_keys_in_map(&yaml, &["valid"], &[&pattern]);
        assert_eq!(invalid, vec!["invalid"]);
    }
}
