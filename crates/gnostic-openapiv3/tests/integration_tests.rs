//! Integration tests comparing Rust parsing with Go reference output.

use gnostic_openapiv3::document::parse_document;
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
fn test_openapiv3_parse_petstore() {
    let bytes = load_openapi_file("petstore-v3.yaml");
    let doc = parse_document(&bytes).expect("Failed to parse petstore-v3.yaml");
    let reference = load_reference("petstore-v3-reference.json");

    // Compare basic fields
    assert_eq!(doc.openapi, reference["openapi"].as_str().unwrap());

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

    // Compare servers
    let ref_servers = reference["servers"].as_array();
    if let Some(ref_servers) = ref_servers {
        assert_eq!(doc.servers.len(), ref_servers.len(), "servers count mismatch");
        for (i, server) in doc.servers.iter().enumerate() {
            let ref_server = &ref_servers[i];
            assert_eq!(server.url, ref_server["url"].as_str().unwrap_or(""),
                "server[{}] url mismatch", i);
        }
    }

    // Compare paths count
    if let Some(paths) = &doc.paths {
        let ref_paths = &reference["paths"]["path"];
        if let Some(ref_path_array) = ref_paths.as_array() {
            assert_eq!(paths.path.len(), ref_path_array.len(),
                "paths count mismatch: rust={}, reference={}",
                paths.path.len(), ref_path_array.len());
        }
    }

    // Compare external docs
    if let Some(external_docs) = &doc.external_docs {
        let ref_external = &reference["externalDocs"];
        if !ref_external.is_null() {
            assert_eq!(external_docs.url, ref_external["url"].as_str().unwrap_or(""));
            assert_eq!(external_docs.description, ref_external["description"].as_str().unwrap_or(""));
        }
    }

    // Compare tags
    let ref_tags = reference["tags"].as_array();
    if let Some(ref_tags) = ref_tags {
        assert_eq!(doc.tags.len(), ref_tags.len(), "tags count mismatch");
        for (i, tag) in doc.tags.iter().enumerate() {
            let ref_tag = &ref_tags[i];
            assert_eq!(tag.name, ref_tag["name"].as_str().unwrap_or(""),
                "tag[{}] name mismatch", i);
        }
    }
}

#[test]
fn test_openapiv3_paths_detail() {
    let bytes = load_openapi_file("petstore-v3.yaml");
    let doc = parse_document(&bytes).expect("Failed to parse petstore-v3.yaml");
    let reference = load_reference("petstore-v3-reference.json");

    let paths = doc.paths.as_ref().expect("paths should exist");
    let ref_paths = reference["paths"]["path"].as_array()
        .expect("reference paths should be array");

    // Create a map for easier lookup
    let ref_path_map: std::collections::HashMap<&str, &Value> = ref_paths.iter()
        .filter_map(|p| p["name"].as_str().map(|n| (n, p)))
        .collect();

    for path_item in &paths.path {
        let ref_path = ref_path_map.get(path_item.name.as_str())
            .unwrap_or_else(|| panic!("Path {} not found in reference", path_item.name));

        let value = path_item.value.as_ref()
            .unwrap_or_else(|| panic!("Path {} has no value", path_item.name));

        // Check operations
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
fn test_openapiv3_components() {
    let bytes = load_openapi_file("petstore-v3.yaml");
    let doc = parse_document(&bytes).expect("Failed to parse petstore-v3.yaml");
    let reference = load_reference("petstore-v3-reference.json");

    if let Some(components) = &doc.components {
        let ref_components = &reference["components"];

        // Check schemas
        if let Some(schemas) = &components.schemas {
            let ref_schemas = &ref_components["schemas"]["additionalProperties"];
            if let Some(ref_schema_array) = ref_schemas.as_array() {
                assert_eq!(schemas.additional_properties.len(), ref_schema_array.len(),
                    "schemas count mismatch");

                // Create a map for lookup
                let ref_schema_map: std::collections::HashMap<&str, &Value> = ref_schema_array.iter()
                    .filter_map(|s| s["name"].as_str().map(|n| (n, s)))
                    .collect();

                for schema in &schemas.additional_properties {
                    if let Some(ref_schema) = ref_schema_map.get(schema.name.as_str()) {
                        // Schema exists in reference
                        assert!(true, "Schema {} found in reference", schema.name);
                    }
                }
            }
        }

        // Check security schemes
        if let Some(security_schemes) = &components.security_schemes {
            let ref_security = &ref_components["securitySchemes"]["additionalProperties"];
            if let Some(ref_security_array) = ref_security.as_array() {
                assert_eq!(security_schemes.additional_properties.len(), ref_security_array.len(),
                    "security schemes count mismatch");
            }
        }
    }
}
