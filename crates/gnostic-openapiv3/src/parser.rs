//! OpenAPI v3 YAML to Protocol Buffer parser.

use gnostic_compiler::{Context, CompilerError, ErrorGroup};
use gnostic_compiler::{map_value_for_key, string_for_scalar_node, bool_for_scalar_node,
                       string_array_for_sequence_node, is_mapping, iter_map};
use serde_yaml::Value as Yaml;
use std::sync::Arc;

use crate::openapi_v3::*;

/// Parser for converting YAML nodes to OpenAPI v3 Protocol Buffer types.
pub struct Parser;

impl Parser {
    /// Parses a Document from a YAML node.
    pub fn parse_document(node: &Yaml, context: &Arc<Context>) -> Result<Document, ErrorGroup> {
        let mut errors = Vec::new();
        let mut doc = Document::default();

        if !is_mapping(node) {
            errors.push(CompilerError::new(context, format!("expected mapping, got {:?}", node)));
            return Err(ErrorGroup::new(errors));
        }

        // Parse openapi version
        if let Some(v) = map_value_for_key(node, "openapi") {
            if let Some(s) = string_for_scalar_node(v) {
                doc.openapi = s;
            }
        }

        // Parse info
        if let Some(v) = map_value_for_key(node, "info") {
            let child_ctx = Arc::new(context.child("info"));
            match Self::parse_info(v, &child_ctx) {
                Ok(info) => doc.info = Some(info),
                Err(e) => errors.extend(e.errors),
            }
        }

        // Parse servers
        if let Some(v) = map_value_for_key(node, "servers") {
            if let Yaml::Sequence(arr) = v {
                for (i, item) in arr.iter().enumerate() {
                    let child_ctx = Arc::new(context.child(format!("servers[{}]", i)));
                    match Self::parse_server(item, &child_ctx) {
                        Ok(server) => doc.servers.push(server),
                        Err(e) => errors.extend(e.errors),
                    }
                }
            }
        }

        // Parse paths
        if let Some(v) = map_value_for_key(node, "paths") {
            let child_ctx = Arc::new(context.child("paths"));
            match Self::parse_paths(v, &child_ctx) {
                Ok(paths) => doc.paths = Some(paths),
                Err(e) => errors.extend(e.errors),
            }
        }

        // Parse components
        if let Some(v) = map_value_for_key(node, "components") {
            let child_ctx = Arc::new(context.child("components"));
            match Self::parse_components(v, &child_ctx) {
                Ok(components) => doc.components = Some(components),
                Err(e) => errors.extend(e.errors),
            }
        }

        // Parse tags
        if let Some(v) = map_value_for_key(node, "tags") {
            if let Yaml::Sequence(arr) = v {
                for (i, item) in arr.iter().enumerate() {
                    let child_ctx = Arc::new(context.child(format!("tags[{}]", i)));
                    match Self::parse_tag(item, &child_ctx) {
                        Ok(tag) => doc.tags.push(tag),
                        Err(e) => errors.extend(e.errors),
                    }
                }
            }
        }

        // Parse externalDocs
        if let Some(v) = map_value_for_key(node, "externalDocs") {
            let child_ctx = Arc::new(context.child("externalDocs"));
            match Self::parse_external_docs(v, &child_ctx) {
                Ok(external_docs) => doc.external_docs = Some(external_docs),
                Err(e) => errors.extend(e.errors),
            }
        }

        if errors.is_empty() {
            Ok(doc)
        } else {
            Err(ErrorGroup::new(errors))
        }
    }

    /// Parses Info from a YAML node.
    pub fn parse_info(node: &Yaml, context: &Arc<Context>) -> Result<Info, ErrorGroup> {
        let mut errors = Vec::new();
        let mut info = Info::default();

        if let Some(v) = map_value_for_key(node, "title") {
            if let Some(s) = string_for_scalar_node(v) {
                info.title = s;
            }
        }

        if let Some(v) = map_value_for_key(node, "description") {
            if let Some(s) = string_for_scalar_node(v) {
                info.description = s;
            }
        }

        if let Some(v) = map_value_for_key(node, "termsOfService") {
            if let Some(s) = string_for_scalar_node(v) {
                info.terms_of_service = s;
            }
        }

        if let Some(v) = map_value_for_key(node, "contact") {
            let child_ctx = Arc::new(context.child("contact"));
            match Self::parse_contact(v, &child_ctx) {
                Ok(contact) => info.contact = Some(contact),
                Err(e) => errors.extend(e.errors),
            }
        }

        if let Some(v) = map_value_for_key(node, "license") {
            let child_ctx = Arc::new(context.child("license"));
            match Self::parse_license(v, &child_ctx) {
                Ok(license) => info.license = Some(license),
                Err(e) => errors.extend(e.errors),
            }
        }

        if let Some(v) = map_value_for_key(node, "version") {
            if let Some(s) = string_for_scalar_node(v) {
                info.version = s;
            }
        }

        if errors.is_empty() {
            Ok(info)
        } else {
            Err(ErrorGroup::new(errors))
        }
    }

    /// Parses Contact from a YAML node.
    pub fn parse_contact(node: &Yaml, _context: &Arc<Context>) -> Result<Contact, ErrorGroup> {
        let mut contact = Contact::default();

        if let Some(v) = map_value_for_key(node, "name") {
            if let Some(s) = string_for_scalar_node(v) {
                contact.name = s;
            }
        }

        if let Some(v) = map_value_for_key(node, "url") {
            if let Some(s) = string_for_scalar_node(v) {
                contact.url = s;
            }
        }

        if let Some(v) = map_value_for_key(node, "email") {
            if let Some(s) = string_for_scalar_node(v) {
                contact.email = s;
            }
        }

        Ok(contact)
    }

    /// Parses License from a YAML node.
    pub fn parse_license(node: &Yaml, _context: &Arc<Context>) -> Result<License, ErrorGroup> {
        let mut license = License::default();

        if let Some(v) = map_value_for_key(node, "name") {
            if let Some(s) = string_for_scalar_node(v) {
                license.name = s;
            }
        }

        if let Some(v) = map_value_for_key(node, "url") {
            if let Some(s) = string_for_scalar_node(v) {
                license.url = s;
            }
        }

        Ok(license)
    }

    /// Parses Server from a YAML node.
    pub fn parse_server(node: &Yaml, _context: &Arc<Context>) -> Result<Server, ErrorGroup> {
        let mut server = Server::default();

        if let Some(v) = map_value_for_key(node, "url") {
            if let Some(s) = string_for_scalar_node(v) {
                server.url = s;
            }
        }

        if let Some(v) = map_value_for_key(node, "description") {
            if let Some(s) = string_for_scalar_node(v) {
                server.description = s;
            }
        }

        Ok(server)
    }

    /// Parses Paths from a YAML node.
    pub fn parse_paths(node: &Yaml, context: &Arc<Context>) -> Result<Paths, ErrorGroup> {
        let mut errors = Vec::new();
        let mut paths = Paths::default();

        iter_map(node, |path, value| {
            let child_ctx = Arc::new(context.child(path.to_string()));
            match Self::parse_path_item(value, &child_ctx) {
                Ok(path_item) => {
                    paths.path.push(NamedPathItem {
                        name: path.to_string(),
                        value: Some(path_item),
                    });
                }
                Err(e) => errors.extend(e.errors),
            }
        });

        if errors.is_empty() {
            Ok(paths)
        } else {
            Err(ErrorGroup::new(errors))
        }
    }

    /// Parses PathItem from a YAML node.
    pub fn parse_path_item(node: &Yaml, context: &Arc<Context>) -> Result<PathItem, ErrorGroup> {
        let mut errors = Vec::new();
        let mut path_item = PathItem::default();

        if let Some(v) = map_value_for_key(node, "$ref") {
            if let Some(s) = string_for_scalar_node(v) {
                path_item.r#ref = s;
            }
        }

        if let Some(v) = map_value_for_key(node, "summary") {
            if let Some(s) = string_for_scalar_node(v) {
                path_item.summary = s;
            }
        }

        if let Some(v) = map_value_for_key(node, "description") {
            if let Some(s) = string_for_scalar_node(v) {
                path_item.description = s;
            }
        }

        // Parse HTTP methods
        for method in &["get", "put", "post", "delete", "options", "head", "patch", "trace"] {
            if let Some(v) = map_value_for_key(node, method) {
                let child_ctx = Arc::new(context.child(*method));
                match Self::parse_operation(v, &child_ctx) {
                    Ok(op) => {
                        match *method {
                            "get" => path_item.get = Some(op),
                            "put" => path_item.put = Some(op),
                            "post" => path_item.post = Some(op),
                            "delete" => path_item.delete = Some(op),
                            "options" => path_item.options = Some(op),
                            "head" => path_item.head = Some(op),
                            "patch" => path_item.patch = Some(op),
                            "trace" => path_item.trace = Some(op),
                            _ => {}
                        }
                    }
                    Err(e) => errors.extend(e.errors),
                }
            }
        }

        if errors.is_empty() {
            Ok(path_item)
        } else {
            Err(ErrorGroup::new(errors))
        }
    }

    /// Parses Operation from a YAML node.
    pub fn parse_operation(node: &Yaml, context: &Arc<Context>) -> Result<Operation, ErrorGroup> {
        let mut errors = Vec::new();
        let mut operation = Operation::default();

        if let Some(v) = map_value_for_key(node, "tags") {
            operation.tags = string_array_for_sequence_node(v);
        }

        if let Some(v) = map_value_for_key(node, "summary") {
            if let Some(s) = string_for_scalar_node(v) {
                operation.summary = s;
            }
        }

        if let Some(v) = map_value_for_key(node, "description") {
            if let Some(s) = string_for_scalar_node(v) {
                operation.description = s;
            }
        }

        if let Some(v) = map_value_for_key(node, "operationId") {
            if let Some(s) = string_for_scalar_node(v) {
                operation.operation_id = s;
            }
        }

        if let Some(v) = map_value_for_key(node, "deprecated") {
            if let Some(b) = bool_for_scalar_node(v) {
                operation.deprecated = b;
            }
        }

        // Parse responses
        if let Some(v) = map_value_for_key(node, "responses") {
            let child_ctx = Arc::new(context.child("responses"));
            match Self::parse_responses(v, &child_ctx) {
                Ok(responses) => operation.responses = Some(responses),
                Err(e) => errors.extend(e.errors),
            }
        }

        if errors.is_empty() {
            Ok(operation)
        } else {
            Err(ErrorGroup::new(errors))
        }
    }

    /// Parses Responses from a YAML node.
    pub fn parse_responses(node: &Yaml, context: &Arc<Context>) -> Result<Responses, ErrorGroup> {
        let mut errors = Vec::new();
        let mut responses = Responses::default();

        iter_map(node, |code, value| {
            let child_ctx = Arc::new(context.child(code.to_string()));
            match Self::parse_response_or_reference(value, &child_ctx) {
                Ok(response) => {
                    responses.response_or_reference.push(NamedResponseOrReference {
                        name: code.to_string(),
                        value: Some(response),
                    });
                }
                Err(e) => errors.extend(e.errors),
            }
        });

        if errors.is_empty() {
            Ok(responses)
        } else {
            Err(ErrorGroup::new(errors))
        }
    }

    /// Parses ResponseOrReference from a YAML node.
    pub fn parse_response_or_reference(node: &Yaml, context: &Arc<Context>) -> Result<ResponseOrReference, ErrorGroup> {
        // Check if it's a reference
        if let Some(v) = map_value_for_key(node, "$ref") {
            if let Some(s) = string_for_scalar_node(v) {
                return Ok(ResponseOrReference {
                    oneof: Some(response_or_reference::Oneof::Reference(Reference {
                        r#ref: s,
                        ..Default::default()
                    })),
                });
            }
        }

        // Parse as response
        Self::parse_response(node, context).map(|r| ResponseOrReference {
            oneof: Some(response_or_reference::Oneof::Response(r)),
        })
    }

    /// Parses Response from a YAML node.
    pub fn parse_response(node: &Yaml, _context: &Arc<Context>) -> Result<Response, ErrorGroup> {
        let mut response = Response::default();

        if let Some(v) = map_value_for_key(node, "description") {
            if let Some(s) = string_for_scalar_node(v) {
                response.description = s;
            }
        }

        Ok(response)
    }

    /// Parses Components from a YAML node.
    pub fn parse_components(node: &Yaml, context: &Arc<Context>) -> Result<Components, ErrorGroup> {
        let mut errors = Vec::new();
        let mut components = Components::default();

        // Parse schemas
        if let Some(v) = map_value_for_key(node, "schemas") {
            let child_ctx = Arc::new(context.child("schemas"));
            match Self::parse_schemas_or_references(v, &child_ctx) {
                Ok(schemas) => components.schemas = Some(schemas),
                Err(e) => errors.extend(e.errors),
            }
        }

        if errors.is_empty() {
            Ok(components)
        } else {
            Err(ErrorGroup::new(errors))
        }
    }

    /// Parses SchemasOrReferences from a YAML node.
    pub fn parse_schemas_or_references(node: &Yaml, context: &Arc<Context>) -> Result<SchemasOrReferences, ErrorGroup> {
        let mut errors = Vec::new();
        let mut schemas = SchemasOrReferences::default();

        iter_map(node, |name, value| {
            let child_ctx = Arc::new(context.child(name.to_string()));
            match Self::parse_schema_or_reference(value, &child_ctx) {
                Ok(schema) => {
                    schemas.additional_properties.push(NamedSchemaOrReference {
                        name: name.to_string(),
                        value: Some(schema),
                    });
                }
                Err(e) => errors.extend(e.errors),
            }
        });

        if errors.is_empty() {
            Ok(schemas)
        } else {
            Err(ErrorGroup::new(errors))
        }
    }

    /// Parses SchemaOrReference from a YAML node.
    pub fn parse_schema_or_reference(node: &Yaml, context: &Arc<Context>) -> Result<SchemaOrReference, ErrorGroup> {
        // Check if it's a reference
        if let Some(v) = map_value_for_key(node, "$ref") {
            if let Some(s) = string_for_scalar_node(v) {
                return Ok(SchemaOrReference {
                    oneof: Some(schema_or_reference::Oneof::Reference(Reference {
                        r#ref: s,
                        ..Default::default()
                    })),
                });
            }
        }

        // Parse as schema
        Self::parse_schema(node, context).map(|s| SchemaOrReference {
            oneof: Some(schema_or_reference::Oneof::Schema(Box::new(s))),
        })
    }

    /// Parses Schema from a YAML node.
    pub fn parse_schema(node: &Yaml, context: &Arc<Context>) -> Result<Schema, ErrorGroup> {
        let mut errors = Vec::new();
        let mut schema = Schema::default();

        if let Some(v) = map_value_for_key(node, "type") {
            if let Some(s) = string_for_scalar_node(v) {
                schema.r#type = s;
            }
        }

        if let Some(v) = map_value_for_key(node, "format") {
            if let Some(s) = string_for_scalar_node(v) {
                schema.format = s;
            }
        }

        if let Some(v) = map_value_for_key(node, "title") {
            if let Some(s) = string_for_scalar_node(v) {
                schema.title = s;
            }
        }

        if let Some(v) = map_value_for_key(node, "description") {
            if let Some(s) = string_for_scalar_node(v) {
                schema.description = s;
            }
        }

        if let Some(v) = map_value_for_key(node, "nullable") {
            if let Some(b) = bool_for_scalar_node(v) {
                schema.nullable = b;
            }
        }

        if let Some(v) = map_value_for_key(node, "readOnly") {
            if let Some(b) = bool_for_scalar_node(v) {
                schema.read_only = b;
            }
        }

        if let Some(v) = map_value_for_key(node, "writeOnly") {
            if let Some(b) = bool_for_scalar_node(v) {
                schema.write_only = b;
            }
        }

        if let Some(v) = map_value_for_key(node, "deprecated") {
            if let Some(b) = bool_for_scalar_node(v) {
                schema.deprecated = b;
            }
        }

        // Parse properties
        if let Some(v) = map_value_for_key(node, "properties") {
            let child_ctx = Arc::new(context.child("properties"));
            match Self::parse_properties(v, &child_ctx) {
                Ok(props) => schema.properties = Some(props),
                Err(e) => errors.extend(e.errors),
            }
        }

        // Parse required
        if let Some(v) = map_value_for_key(node, "required") {
            schema.required = string_array_for_sequence_node(v);
        }

        // Parse items (for arrays)
        if let Some(v) = map_value_for_key(node, "items") {
            let child_ctx = Arc::new(context.child("items"));
            match Self::parse_schema_or_reference(v, &child_ctx) {
                Ok(items) => {
                    schema.items = Some(ItemsItem {
                        schema_or_reference: vec![items],
                    });
                }
                Err(e) => errors.extend(e.errors),
            }
        }

        if errors.is_empty() {
            Ok(schema)
        } else {
            Err(ErrorGroup::new(errors))
        }
    }

    /// Parses Properties from a YAML node.
    pub fn parse_properties(node: &Yaml, context: &Arc<Context>) -> Result<Properties, ErrorGroup> {
        let mut errors = Vec::new();
        let mut properties = Properties::default();

        iter_map(node, |name, value| {
            let child_ctx = Arc::new(context.child(name.to_string()));
            match Self::parse_schema_or_reference(value, &child_ctx) {
                Ok(schema) => {
                    properties.additional_properties.push(NamedSchemaOrReference {
                        name: name.to_string(),
                        value: Some(schema),
                    });
                }
                Err(e) => errors.extend(e.errors),
            }
        });

        if errors.is_empty() {
            Ok(properties)
        } else {
            Err(ErrorGroup::new(errors))
        }
    }

    /// Parses Tag from a YAML node.
    pub fn parse_tag(node: &Yaml, _context: &Arc<Context>) -> Result<Tag, ErrorGroup> {
        let mut tag = Tag::default();

        if let Some(v) = map_value_for_key(node, "name") {
            if let Some(s) = string_for_scalar_node(v) {
                tag.name = s;
            }
        }

        if let Some(v) = map_value_for_key(node, "description") {
            if let Some(s) = string_for_scalar_node(v) {
                tag.description = s;
            }
        }

        Ok(tag)
    }

    /// Parses ExternalDocs from a YAML node.
    pub fn parse_external_docs(node: &Yaml, _context: &Arc<Context>) -> Result<ExternalDocs, ErrorGroup> {
        let mut external_docs = ExternalDocs::default();

        if let Some(v) = map_value_for_key(node, "description") {
            if let Some(s) = string_for_scalar_node(v) {
                external_docs.description = s;
            }
        }

        if let Some(v) = map_value_for_key(node, "url") {
            if let Some(s) = string_for_scalar_node(v) {
                external_docs.url = s;
            }
        }

        Ok(external_docs)
    }
}
