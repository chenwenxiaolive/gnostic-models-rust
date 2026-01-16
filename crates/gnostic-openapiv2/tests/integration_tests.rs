//! Integration tests comparing Rust parsing with Go reference output.

use gnostic_openapiv2::document::parse_document;
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

/// Load and parse an OpenAPI file
fn load_openapi_file(filename: &str) -> Vec<u8> {
    let path = format!("{}/{}", TESTDATA_DIR, filename);
    fs::read(&path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", path, e))
}

#[test]
fn test_openapiv2_parse_petstore() {
    let bytes = load_openapi_file("petstore-v2.json");
    let doc = parse_document(&bytes).expect("Failed to parse petstore-v2.json");
    let reference = load_reference("petstore-v2-reference.json");

    // Compare basic fields
    assert_eq!(doc.swagger, reference["swagger"].as_str().unwrap());
    assert_eq!(doc.host, reference["host"].as_str().unwrap_or(""));
    assert_eq!(doc.base_path, reference["basePath"].as_str().unwrap_or(""));

    // Compare info
    let info = doc.info.as_ref().expect("info should exist");
    let ref_info = &reference["info"];
    assert_eq!(info.title, ref_info["title"].as_str().unwrap());
    assert_eq!(info.version, ref_info["version"].as_str().unwrap());
    assert_eq!(info.description, ref_info["description"].as_str().unwrap_or(""));
    assert_eq!(info.terms_of_service, ref_info["termsOfService"].as_str().unwrap_or(""));

    // Compare contact
    if let Some(contact) = &info.contact {
        let ref_contact = &ref_info["contact"];
        assert_eq!(contact.email, ref_contact["email"].as_str().unwrap_or(""));
    }

    // Compare license
    if let Some(license) = &info.license {
        let ref_license = &ref_info["license"];
        assert_eq!(license.name, ref_license["name"].as_str().unwrap_or(""));
        assert_eq!(license.url, ref_license["url"].as_str().unwrap_or(""));
    }

    // Compare schemes
    let ref_schemes = reference["schemes"].as_array();
    if let Some(ref_schemes) = ref_schemes {
        let ref_scheme_strs: Vec<&str> = ref_schemes.iter()
            .filter_map(|s| s.as_str())
            .collect();
        assert_eq!(doc.schemes.len(), ref_scheme_strs.len(), "schemes count mismatch");
        for (i, scheme) in doc.schemes.iter().enumerate() {
            assert_eq!(scheme, ref_scheme_strs[i], "scheme[{}] mismatch", i);
        }
    }

    // Compare consumes
    let ref_consumes = reference["consumes"].as_array();
    if let Some(ref_consumes) = ref_consumes {
        assert_eq!(doc.consumes.len(), ref_consumes.len(), "consumes count mismatch");
    }

    // Compare produces
    let ref_produces = reference["produces"].as_array();
    if let Some(ref_produces) = ref_produces {
        assert_eq!(doc.produces.len(), ref_produces.len(), "produces count mismatch");
    }
}

#[test]
fn test_openapiv2_paths() {
    let bytes = load_openapi_file("petstore-v2.json");
    let doc = parse_document(&bytes).expect("Failed to parse petstore-v2.json");
    let reference = load_reference("petstore-v2-reference.json");

    let paths = doc.paths.as_ref().expect("paths should exist");
    let ref_paths = reference["paths"]["path"].as_array()
        .expect("reference paths should be array");

    // Check path count
    assert_eq!(paths.path.len(), ref_paths.len(),
        "paths count mismatch: rust={}, reference={}",
        paths.path.len(), ref_paths.len());

    // Create a map for easier lookup
    let ref_path_map: std::collections::HashMap<&str, &Value> = ref_paths.iter()
        .filter_map(|p| p["name"].as_str().map(|n| (n, p)))
        .collect();

    for path_item in &paths.path {
        let ref_path = ref_path_map.get(path_item.name.as_str())
            .unwrap_or_else(|| panic!("Path {} not found in reference", path_item.name));

        let value = path_item.value.as_ref()
            .unwrap_or_else(|| panic!("Path {} has no value", path_item.name));

        let ref_value = &ref_path["value"];

        // Check GET
        if let Some(get) = &value.get {
            let ref_get = &ref_value["get"];
            if !ref_get.is_null() {
                assert_eq!(get.operation_id, ref_get["operationId"].as_str().unwrap_or(""),
                    "GET operationId mismatch for {}", path_item.name);
                assert_eq!(get.summary, ref_get["summary"].as_str().unwrap_or(""),
                    "GET summary mismatch for {}", path_item.name);
            }
        }

        // Check POST
        if let Some(post) = &value.post {
            let ref_post = &ref_value["post"];
            if !ref_post.is_null() {
                assert_eq!(post.operation_id, ref_post["operationId"].as_str().unwrap_or(""),
                    "POST operationId mismatch for {}", path_item.name);
            }
        }

        // Check PUT
        if let Some(put) = &value.put {
            let ref_put = &ref_value["put"];
            if !ref_put.is_null() {
                assert_eq!(put.operation_id, ref_put["operationId"].as_str().unwrap_or(""),
                    "PUT operationId mismatch for {}", path_item.name);
            }
        }

        // Check DELETE
        if let Some(delete) = &value.delete {
            let ref_delete = &ref_value["delete"];
            if !ref_delete.is_null() {
                assert_eq!(delete.operation_id, ref_delete["operationId"].as_str().unwrap_or(""),
                    "DELETE operationId mismatch for {}", path_item.name);
            }
        }
    }
}

#[test]
fn test_openapiv2_definitions() {
    let bytes = load_openapi_file("petstore-v2.json");
    let doc = parse_document(&bytes).expect("Failed to parse petstore-v2.json");
    let reference = load_reference("petstore-v2-reference.json");

    if let Some(definitions) = &doc.definitions {
        let ref_defs = &reference["definitions"]["additionalProperties"];
        if let Some(ref_def_array) = ref_defs.as_array() {
            assert_eq!(definitions.additional_properties.len(), ref_def_array.len(),
                "definitions count mismatch: rust={}, reference={}",
                definitions.additional_properties.len(), ref_def_array.len());

            // Create a map for lookup
            let ref_def_map: std::collections::HashMap<&str, &Value> = ref_def_array.iter()
                .filter_map(|d| d["name"].as_str().map(|n| (n, d)))
                .collect();

            for def in &definitions.additional_properties {
                assert!(ref_def_map.contains_key(def.name.as_str()),
                    "Definition {} not found in reference", def.name);

                if let Some(schema) = &def.value {
                    let ref_schema = &ref_def_map[def.name.as_str()]["value"];

                    // Check type if present
                    if let Some(type_item) = &schema.r#type {
                        let ref_type = &ref_schema["type"]["value"];
                        if let Some(ref_type_array) = ref_type.as_array() {
                            let ref_type_strs: Vec<&str> = ref_type_array.iter()
                                .filter_map(|t| t.as_str())
                                .collect();
                            assert_eq!(type_item.value.len(), ref_type_strs.len(),
                                "Type count mismatch for definition {}", def.name);
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn test_openapiv2_tags() {
    let bytes = load_openapi_file("petstore-v2.json");
    let doc = parse_document(&bytes).expect("Failed to parse petstore-v2.json");
    let reference = load_reference("petstore-v2-reference.json");

    let ref_tags = reference["tags"].as_array();
    if let Some(ref_tags) = ref_tags {
        assert_eq!(doc.tags.len(), ref_tags.len(),
            "tags count mismatch: rust={}, reference={}",
            doc.tags.len(), ref_tags.len());

        for (i, tag) in doc.tags.iter().enumerate() {
            let ref_tag = &ref_tags[i];
            assert_eq!(tag.name, ref_tag["name"].as_str().unwrap_or(""),
                "tag[{}] name mismatch", i);
            assert_eq!(tag.description, ref_tag["description"].as_str().unwrap_or(""),
                "tag[{}] description mismatch", i);
        }
    }
}

#[test]
fn test_openapiv2_external_docs() {
    let bytes = load_openapi_file("petstore-v2.json");
    let doc = parse_document(&bytes).expect("Failed to parse petstore-v2.json");
    let reference = load_reference("petstore-v2-reference.json");

    if let Some(external_docs) = &doc.external_docs {
        let ref_external = &reference["externalDocs"];
        if !ref_external.is_null() {
            assert_eq!(external_docs.url, ref_external["url"].as_str().unwrap_or(""));
            assert_eq!(external_docs.description, ref_external["description"].as_str().unwrap_or(""));
        }
    }
}
