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

//! Error types for the compiler.

use crate::context::Context;
use std::fmt;
use thiserror::Error;

/// CompilerError represents compiler errors and their location in the document.
#[derive(Error, Debug, Clone)]
pub enum CompilerError {
    /// Error with location information (line and column).
    #[error("[{line},{column}] {path} {message}")]
    Located {
        line: usize,
        column: usize,
        path: String,
        message: String,
    },

    /// Error without location information.
    #[error("{path} {message}")]
    Unlocated { path: String, message: String },

    /// Simple error message without context.
    #[error("{0}")]
    Simple(String),

    /// IO error.
    #[error("IO error: {0}")]
    Io(String),

    /// YAML parsing error.
    #[error("YAML error: {0}")]
    Yaml(String),

    /// HTTP error.
    #[error("HTTP error: {0}")]
    Http(String),
}

impl CompilerError {
    /// Creates a new error from a context and message.
    pub fn new(context: &Context, message: impl Into<String>) -> Self {
        let message = message.into();
        match (context.line, context.column) {
            (Some(line), Some(column)) => CompilerError::Located {
                line,
                column,
                path: context.description(),
                message,
            },
            _ => CompilerError::Unlocated {
                path: context.description(),
                message,
            },
        }
    }

    /// Creates a new error from an optional context and message.
    pub fn new_opt(context: Option<&Context>, message: impl Into<String>) -> Self {
        match context {
            Some(ctx) => Self::new(ctx, message),
            None => CompilerError::Simple(message.into()),
        }
    }
}

/// ErrorGroup is a container for groups of errors.
#[derive(Debug, Clone)]
pub struct ErrorGroup {
    pub errors: Vec<CompilerError>,
}

impl ErrorGroup {
    /// Creates a new ErrorGroup from a vector of errors.
    pub fn new(errors: Vec<CompilerError>) -> Self {
        ErrorGroup { errors }
    }

    /// Returns a new ErrorGroup for a slice of errors or None if the slice is empty.
    pub fn from_errors(errors: Vec<CompilerError>) -> Option<Self> {
        if errors.is_empty() {
            None
        } else {
            Some(ErrorGroup { errors })
        }
    }

    /// Returns true if the group contains no errors.
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    /// Returns the number of errors in the group.
    pub fn len(&self) -> usize {
        self.errors.len()
    }

    /// Adds an error to the group.
    pub fn push(&mut self, error: CompilerError) {
        self.errors.push(error);
    }

    /// Extends the group with errors from another group.
    pub fn extend(&mut self, other: ErrorGroup) {
        self.errors.extend(other.errors);
    }

    /// Converts the group to a Result, returning Ok(()) if empty.
    pub fn into_result(self) -> std::result::Result<(), Self> {
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self)
        }
    }
}

impl fmt::Display for ErrorGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, err) in self.errors.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            write!(f, "{}", err)?;
        }
        Ok(())
    }
}

impl std::error::Error for ErrorGroup {}

impl Default for ErrorGroup {
    fn default() -> Self {
        ErrorGroup { errors: Vec::new() }
    }
}

impl From<CompilerError> for ErrorGroup {
    fn from(error: CompilerError) -> Self {
        ErrorGroup {
            errors: vec![error],
        }
    }
}

impl From<std::io::Error> for CompilerError {
    fn from(err: std::io::Error) -> Self {
        CompilerError::Io(err.to_string())
    }
}

impl From<yaml_rust2::ScanError> for CompilerError {
    fn from(err: yaml_rust2::ScanError) -> Self {
        CompilerError::Yaml(err.to_string())
    }
}

/// Result type alias for compiler operations.
pub type Result<T> = std::result::Result<T, CompilerError>;

/// MultiResult type alias for operations that may produce multiple errors.
pub type MultiResult<T> = std::result::Result<T, ErrorGroup>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Context;

    #[test]
    fn test_error_with_location() {
        let ctx = Context::new("test.field", Some(10), Some(5), None);
        let err = CompilerError::new(&ctx, "invalid value");
        assert_eq!(err.to_string(), "[10,5] test.field invalid value");
    }

    #[test]
    fn test_error_without_location() {
        let ctx = Context::new("test.field", None, None, None);
        let err = CompilerError::new(&ctx, "invalid value");
        assert_eq!(err.to_string(), "test.field invalid value");
    }

    #[test]
    fn test_error_group() {
        let ctx = Context::root("root");
        let mut group = ErrorGroup::default();
        group.push(CompilerError::new(&ctx, "error 1"));
        group.push(CompilerError::new(&ctx, "error 2"));
        assert_eq!(group.len(), 2);
        assert!(!group.is_empty());
    }

    #[test]
    fn test_error_group_from_errors() {
        let empty: Vec<CompilerError> = vec![];
        assert!(ErrorGroup::from_errors(empty).is_none());

        let errors = vec![CompilerError::Simple("test".to_string())];
        assert!(ErrorGroup::from_errors(errors).is_some());
    }
}
