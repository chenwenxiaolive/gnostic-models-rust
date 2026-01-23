//! OpenAPI v2 (Swagger) YAML to Protocol Buffer parser.

use gnostic_compiler::{Context, CompilerError, ErrorGroup};
use gnostic_compiler::{map_value_for_key, string_for_scalar_node, bool_for_scalar_node,
                       string_array_for_sequence_node,
                       is_mapping, is_sequence, iter_map, iter_sequence};
use std::sync::Arc;
use serde_yaml::Value as Yaml;

use crate::openapi_v2::*;

/// Parser for converting YAML nodes to OpenAPI v2 Protocol Buffer types.
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

        // Parse swagger version
        if let Some(v) = map_value_for_key(node, "swagger") {
            if let Some(s) = string_for_scalar_node(v) {
                doc.swagger = s;
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

        // Parse host
        if let Some(v) = map_value_for_key(node, "host") {
            if let Some(s) = string_for_scalar_node(v) {
                doc.host = s;
            }
        }

        // Parse basePath
        if let Some(v) = map_value_for_key(node, "basePath") {
            if let Some(s) = string_for_scalar_node(v) {
                doc.base_path = s;
            }
        }

        // Parse schemes
        if let Some(v) = map_value_for_key(node, "schemes") {
            doc.schemes = string_array_for_sequence_node(v);
        }

        // Parse consumes
        if let Some(v) = map_value_for_key(node, "consumes") {
            doc.consumes = string_array_for_sequence_node(v);
        }

        // Parse produces
        if let Some(v) = map_value_for_key(node, "produces") {
            doc.produces = string_array_for_sequence_node(v);
        }

        // Parse paths
        if let Some(v) = map_value_for_key(node, "paths") {
            let child_ctx = Arc::new(context.child("paths"));
            match Self::parse_paths(v, &child_ctx) {
                Ok(paths) => doc.paths = Some(paths),
                Err(e) => errors.extend(e.errors),
            }
        }

        // Parse definitions
        if let Some(v) = map_value_for_key(node, "definitions") {
            let child_ctx = Arc::new(context.child("definitions"));
            match Self::parse_definitions(v, &child_ctx) {
                Ok(defs) => doc.definitions = Some(defs),
                Err(e) => errors.extend(e.errors),
            }
        }

        // Parse tags
        if let Some(v) = map_value_for_key(node, "tags") {
            let child_ctx = Arc::new(context.child("tags"));
            match Self::parse_tags(v, &child_ctx) {
                Ok(tags) => doc.tags = tags,
                Err(e) => errors.extend(e.errors),
            }
        }

        // Parse externalDocs
        if let Some(v) = map_value_for_key(node, "externalDocs") {
            let child_ctx = Arc::new(context.child("externalDocs"));
            match Self::parse_external_docs(v, &child_ctx) {
                Ok(ext) => doc.external_docs = Some(ext),
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

        if let Some(v) = map_value_for_key(node, "version") {
            if let Some(s) = string_for_scalar_node(v) {
                info.version = s;
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

        // Parse HTTP methods
        for method in &["get", "put", "post", "delete", "options", "head", "patch"] {
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
    pub fn parse_operation(node: &Yaml, _context: &Arc<Context>) -> Result<Operation, ErrorGroup> {
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

        if let Some(v) = map_value_for_key(node, "consumes") {
            operation.consumes = string_array_for_sequence_node(v);
        }

        if let Some(v) = map_value_for_key(node, "produces") {
            operation.produces = string_array_for_sequence_node(v);
        }

        if let Some(v) = map_value_for_key(node, "deprecated") {
            if let Some(b) = bool_for_scalar_node(v) {
                operation.deprecated = b;
            }
        }

        Ok(operation)
    }

    /// Parses Definitions from a YAML node.
    pub fn parse_definitions(node: &Yaml, context: &Arc<Context>) -> Result<Definitions, ErrorGroup> {
        let mut errors = Vec::new();
        let mut definitions = Definitions::default();

        iter_map(node, |name, value| {
            let child_ctx = Arc::new(context.child(name.to_string()));
            match Self::parse_schema(value, &child_ctx) {
                Ok(schema) => {
                    definitions.additional_properties.push(NamedSchema {
                        name: name.to_string(),
                        value: Some(schema),
                    });
                }
                Err(e) => errors.extend(e.errors),
            }
        });

        if errors.is_empty() {
            Ok(definitions)
        } else {
            Err(ErrorGroup::new(errors))
        }
    }

    /// Parses Schema from a YAML node.
    pub fn parse_schema(node: &Yaml, _context: &Arc<Context>) -> Result<Schema, ErrorGroup> {
        let mut schema = Schema::default();

        if let Some(v) = map_value_for_key(node, "$ref") {
            if let Some(s) = string_for_scalar_node(v) {
                schema.r#ref = s;
            }
        }

        if let Some(v) = map_value_for_key(node, "type") {
            if let Some(s) = string_for_scalar_node(v) {
                schema.r#type = Some(TypeItem { value: vec![s] });
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

        if let Some(v) = map_value_for_key(node, "required") {
            schema.required = string_array_for_sequence_node(v);
        }

        Ok(schema)
    }

    /// Parses tags array from a YAML node.
    pub fn parse_tags(node: &Yaml, context: &Arc<Context>) -> Result<Vec<Tag>, ErrorGroup> {
        let mut errors = Vec::new();
        let mut tags = Vec::new();

        if !is_sequence(node) {
            errors.push(CompilerError::new(context, "tags must be an array".to_string()));
            return Err(ErrorGroup::new(errors));
        }

        iter_sequence(node, |i, item| {
            let child_ctx = Arc::new(context.child(i.to_string()));
            match Self::parse_tag(item, &child_ctx) {
                Ok(tag) => tags.push(tag),
                Err(e) => errors.extend(e.errors),
            }
        });

        if errors.is_empty() {
            Ok(tags)
        } else {
            Err(ErrorGroup::new(errors))
        }
    }

    /// Parses a single Tag from a YAML node.
    pub fn parse_tag(node: &Yaml, context: &Arc<Context>) -> Result<Tag, ErrorGroup> {
        let mut errors = Vec::new();
        let mut tag = Tag::default();

        if !is_mapping(node) {
            errors.push(CompilerError::new(context, "tag must be an object".to_string()));
            return Err(ErrorGroup::new(errors));
        }

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

        if let Some(v) = map_value_for_key(node, "externalDocs") {
            let child_ctx = Arc::new(context.child("externalDocs"));
            match Self::parse_external_docs(v, &child_ctx) {
                Ok(ext) => tag.external_docs = Some(ext),
                Err(e) => errors.extend(e.errors),
            }
        }

        if errors.is_empty() {
            Ok(tag)
        } else {
            Err(ErrorGroup::new(errors))
        }
    }

    /// Parses ExternalDocs from a YAML node.
    pub fn parse_external_docs(node: &Yaml, context: &Arc<Context>) -> Result<ExternalDocs, ErrorGroup> {
        let mut errors = Vec::new();
        let mut external_docs = ExternalDocs::default();

        if !is_mapping(node) {
            errors.push(CompilerError::new(context, "externalDocs must be an object".to_string()));
            return Err(ErrorGroup::new(errors));
        }

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

        if errors.is_empty() {
            Ok(external_docs)
        } else {
            Err(ErrorGroup::new(errors))
        }
    }
}
