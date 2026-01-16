//! Google API Discovery format parser.

use gnostic_compiler::{Context, CompilerError, ErrorGroup};
use gnostic_compiler::{map_value_for_key, string_for_scalar_node, is_mapping};
use std::sync::Arc;
use yaml_rust2::Yaml;

use crate::discovery::*;

/// Parser for converting YAML/JSON nodes to Discovery Protocol Buffer types.
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

        if let Some(v) = map_value_for_key(node, "kind") {
            if let Some(s) = string_for_scalar_node(v) {
                doc.kind = s;
            }
        }

        if let Some(v) = map_value_for_key(node, "discoveryVersion") {
            if let Some(s) = string_for_scalar_node(v) {
                doc.discovery_version = s;
            }
        }

        if let Some(v) = map_value_for_key(node, "id") {
            if let Some(s) = string_for_scalar_node(v) {
                doc.id = s;
            }
        }

        if let Some(v) = map_value_for_key(node, "name") {
            if let Some(s) = string_for_scalar_node(v) {
                doc.name = s;
            }
        }

        if let Some(v) = map_value_for_key(node, "version") {
            if let Some(s) = string_for_scalar_node(v) {
                doc.version = s;
            }
        }

        if let Some(v) = map_value_for_key(node, "revision") {
            if let Some(s) = string_for_scalar_node(v) {
                doc.revision = s;
            }
        }

        if let Some(v) = map_value_for_key(node, "title") {
            if let Some(s) = string_for_scalar_node(v) {
                doc.title = s;
            }
        }

        if let Some(v) = map_value_for_key(node, "description") {
            if let Some(s) = string_for_scalar_node(v) {
                doc.description = s;
            }
        }

        if let Some(v) = map_value_for_key(node, "documentationLink") {
            if let Some(s) = string_for_scalar_node(v) {
                doc.documentation_link = s;
            }
        }

        if let Some(v) = map_value_for_key(node, "protocol") {
            if let Some(s) = string_for_scalar_node(v) {
                doc.protocol = s;
            }
        }

        if let Some(v) = map_value_for_key(node, "baseUrl") {
            if let Some(s) = string_for_scalar_node(v) {
                doc.base_url = s;
            }
        }

        if let Some(v) = map_value_for_key(node, "basePath") {
            if let Some(s) = string_for_scalar_node(v) {
                doc.base_path = s;
            }
        }

        if let Some(v) = map_value_for_key(node, "rootUrl") {
            if let Some(s) = string_for_scalar_node(v) {
                doc.root_url = s;
            }
        }

        if let Some(v) = map_value_for_key(node, "servicePath") {
            if let Some(s) = string_for_scalar_node(v) {
                doc.service_path = s;
            }
        }

        if let Some(v) = map_value_for_key(node, "batchPath") {
            if let Some(s) = string_for_scalar_node(v) {
                doc.batch_path = s;
            }
        }

        if errors.is_empty() {
            Ok(doc)
        } else {
            Err(ErrorGroup::new(errors))
        }
    }
}
