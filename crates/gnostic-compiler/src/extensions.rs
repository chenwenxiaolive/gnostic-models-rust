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

//! Extension handler support for vendor extensions.

use crate::context::Context;
use crate::error::{CompilerError, Result};
use std::io::Write;
use std::process::{Command, Stdio};
use std::sync::Arc;
use yaml_rust2::Yaml;

/// ExtensionHandler describes a binary that is called by the compiler to handle specification extensions.
#[derive(Debug, Clone)]
pub struct ExtensionHandler {
    /// Name of the extension handler binary.
    pub name: String,
}

impl ExtensionHandler {
    /// Creates a new ExtensionHandler.
    pub fn new(name: impl Into<String>) -> Self {
        ExtensionHandler { name: name.into() }
    }

    /// Handles an extension by calling the external binary.
    pub fn handle(&self, node: &Yaml, extension_name: &str) -> Result<Option<Vec<u8>>> {
        if self.name.is_empty() {
            return Ok(None);
        }

        // Serialize the YAML node
        let mut yaml_str = String::new();
        {
            let mut emitter = yaml_rust2::YamlEmitter::new(&mut yaml_str);
            let _ = emitter.dump(node);
        }

        // Build request (simplified - in real implementation this would use protobuf)
        // For now, we'll pass YAML directly and expect YAML back
        let request = format!(
            "version: \"0.1.0\"\nextension_name: \"{}\"\nyaml: |\n{}",
            extension_name,
            yaml_str
                .lines()
                .map(|l| format!("  {}", l))
                .collect::<Vec<_>>()
                .join("\n")
        );

        // Call the external handler
        let mut child = Command::new(&self.name)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| {
                CompilerError::Io(format!("Failed to spawn extension handler {}: {}", self.name, e))
            })?;

        // Write request to stdin
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(request.as_bytes()).map_err(|e| {
                CompilerError::Io(format!("Failed to write to extension handler: {}", e))
            })?;
        }

        // Wait for output
        let output = child.wait_with_output().map_err(|e| {
            CompilerError::Io(format!("Failed to get extension handler output: {}", e))
        })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(CompilerError::Simple(format!(
                "Extension handler {} failed: {}",
                self.name, stderr
            )));
        }

        if output.stdout.is_empty() {
            return Ok(None);
        }

        Ok(Some(output.stdout))
    }
}

/// Calls extension handlers for a given extension.
pub fn call_extension(
    context: &Context,
    node: &Yaml,
    extension_name: &str,
) -> Result<(bool, Option<Vec<u8>>)> {
    let handlers = match &context.extension_handlers {
        Some(h) => h,
        None => return Ok((false, None)),
    };

    for handler in handlers.iter() {
        match handler.handle(node, extension_name)? {
            Some(response) => return Ok((true, Some(response))),
            None => continue,
        }
    }

    Ok((false, None))
}

/// Creates extension handlers from a context.
pub fn get_extension_handlers(context: &Context) -> Option<Arc<Vec<ExtensionHandler>>> {
    context.extension_handlers.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extension_handler_new() {
        let handler = ExtensionHandler::new("test-handler");
        assert_eq!(handler.name, "test-handler");
    }

    #[test]
    fn test_extension_handler_empty_name() {
        let handler = ExtensionHandler::new("");
        let yaml = Yaml::Null;
        let result = handler.handle(&yaml, "x-test");
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_call_extension_no_handlers() {
        let context = Context::root("test");
        let yaml = Yaml::Null;
        let result = call_extension(&context, &yaml, "x-test");
        assert!(result.is_ok());
        let (handled, _) = result.unwrap();
        assert!(!handled);
    }
}
