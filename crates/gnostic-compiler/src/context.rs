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

//! Context management for document traversal.

use crate::extensions::ExtensionHandler;
use std::sync::Arc;
use serde_yaml::Value as Yaml;

/// Context contains state of the compiler as it traverses a document.
#[derive(Debug, Clone)]
pub struct Context {
    /// Parent context in the traversal hierarchy.
    pub parent: Option<Arc<Context>>,
    /// Name of the current element being processed.
    pub name: String,
    /// Line number in the source document (if available).
    pub line: Option<usize>,
    /// Column number in the source document (if available).
    pub column: Option<usize>,
    /// Extension handlers for processing vendor extensions.
    pub extension_handlers: Option<Arc<Vec<ExtensionHandler>>>,
}

impl Context {
    /// Creates a new Context with extension handlers.
    pub fn new_with_extensions(
        name: impl Into<String>,
        line: Option<usize>,
        column: Option<usize>,
        parent: Option<Arc<Context>>,
        extension_handlers: Option<Arc<Vec<ExtensionHandler>>>,
    ) -> Self {
        Context {
            parent,
            name: name.into(),
            line,
            column,
            extension_handlers,
        }
    }

    /// Creates a new Context, inheriting extension handlers from the parent.
    pub fn new(
        name: impl Into<String>,
        line: Option<usize>,
        column: Option<usize>,
        parent: Option<Arc<Context>>,
    ) -> Self {
        let extension_handlers = parent.as_ref().and_then(|p| p.extension_handlers.clone());
        Context {
            parent,
            name: name.into(),
            line,
            column,
            extension_handlers,
        }
    }

    /// Creates a new root Context.
    pub fn root(name: impl Into<String>) -> Self {
        Context {
            parent: None,
            name: name.into(),
            line: None,
            column: None,
            extension_handlers: None,
        }
    }

    /// Creates a child Context with the given name.
    pub fn child(self: &Arc<Self>, name: impl Into<String>) -> Self {
        Context::new(name, None, None, Some(Arc::clone(self)))
    }

    /// Creates a child Context with position information.
    pub fn child_with_position(
        self: &Arc<Self>,
        name: impl Into<String>,
        line: usize,
        column: usize,
    ) -> Self {
        Context::new(name, Some(line), Some(column), Some(Arc::clone(self)))
    }

    /// Returns a text description of the compiler state (path from root).
    pub fn description(&self) -> String {
        match &self.parent {
            Some(parent) => format!("{}.{}", parent.description(), self.name),
            None => self.name.clone(),
        }
    }

    /// Returns the location description with line and column if available.
    pub fn location_description(&self) -> String {
        match (self.line, self.column) {
            (Some(line), Some(column)) => {
                format!("[{},{}] {}", line, column, self.description())
            }
            _ => self.description(),
        }
    }
}

/// Extracts line and column from a serde_yaml node if available.
/// Note: serde_yaml doesn't directly provide line/column info in the same way as Go's yaml.v3,
/// so this function is a placeholder for future enhancement.
pub fn position_from_yaml(_node: &Yaml) -> (Option<usize>, Option<usize>) {
    // serde_yaml doesn't provide position information by default
    // This could be enhanced with a custom parser or different YAML library
    (None, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_description() {
        let root = Arc::new(Context::root("root"));
        assert_eq!(root.description(), "root");

        let child = Arc::new(root.child("child"));
        assert_eq!(child.description(), "root.child");

        let grandchild = child.child("grandchild");
        assert_eq!(grandchild.description(), "root.child.grandchild");
    }

    #[test]
    fn test_location_description() {
        let ctx = Context::new("test", Some(10), Some(5), None);
        assert_eq!(ctx.location_description(), "[10,5] test");

        let ctx_no_pos = Context::new("test", None, None, None);
        assert_eq!(ctx_no_pos.location_description(), "test");
    }
}
