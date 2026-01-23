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

//! File and HTTP reading with caching support.

use crate::error::{CompilerError, Result};
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use serde_yaml::Value as Yaml;
use std::collections::HashMap;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use url::Url;

/// Global file cache (thread-safe).
static FILE_CACHE: Lazy<RwLock<HashMap<String, Vec<u8>>>> = Lazy::new(|| RwLock::new(HashMap::new()));

/// Global parsed YAML cache (thread-safe).
static INFO_CACHE: Lazy<RwLock<HashMap<String, Yaml>>> = Lazy::new(|| RwLock::new(HashMap::new()));

/// File cache enabled flag.
static FILE_CACHE_ENABLED: AtomicBool = AtomicBool::new(true);

/// Info cache enabled flag.
static INFO_CACHE_ENABLED: AtomicBool = AtomicBool::new(true);

/// Verbose reader flag.
static VERBOSE_READER: AtomicBool = AtomicBool::new(false);

/// Enables file caching.
pub fn enable_file_cache() {
    FILE_CACHE_ENABLED.store(true, Ordering::SeqCst);
}

/// Disables file caching.
pub fn disable_file_cache() {
    FILE_CACHE_ENABLED.store(false, Ordering::SeqCst);
}

/// Enables parsed info caching.
pub fn enable_info_cache() {
    INFO_CACHE_ENABLED.store(true, Ordering::SeqCst);
}

/// Disables parsed info caching.
pub fn disable_info_cache() {
    INFO_CACHE_ENABLED.store(false, Ordering::SeqCst);
}

/// Sets verbose reader mode.
pub fn set_verbose_reader(verbose: bool) {
    VERBOSE_READER.store(verbose, Ordering::SeqCst);
}

/// Removes an entry from the file cache.
pub fn remove_from_file_cache(fileurl: &str) {
    if FILE_CACHE_ENABLED.load(Ordering::SeqCst) {
        FILE_CACHE.write().remove(fileurl);
    }
}

/// Removes an entry from the info cache.
pub fn remove_from_info_cache(filename: &str) {
    if INFO_CACHE_ENABLED.load(Ordering::SeqCst) {
        INFO_CACHE.write().remove(filename);
    }
}

/// Clears the file cache.
pub fn clear_file_cache() {
    FILE_CACHE.write().clear();
}

/// Clears the info cache.
pub fn clear_info_cache() {
    INFO_CACHE.write().clear();
}

/// Clears all caches.
pub fn clear_caches() {
    clear_file_cache();
    clear_info_cache();
}

/// Fetches a URL asynchronously (public API for use by other crates).
pub async fn fetch_url(url_str: &str) -> Result<Vec<u8>> {
    fetch_url_async(url_str).await
}

/// Fetches a file from a URL using hyper.
pub fn fetch_file(fileurl: &str) -> Result<Vec<u8>> {
    let cache_enabled = FILE_CACHE_ENABLED.load(Ordering::SeqCst);
    let verbose = VERBOSE_READER.load(Ordering::SeqCst);

    // Check cache first
    if cache_enabled {
        if let Some(bytes) = FILE_CACHE.read().get(fileurl) {
            if verbose {
                log::info!("Cache hit {}", fileurl);
            }
            return Ok(bytes.clone());
        }
        if verbose {
            log::info!("Fetching {}", fileurl);
        }
    }

    // Use tokio runtime for async HTTP request
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|e| CompilerError::Http(format!("Failed to create runtime: {}", e)))?;

    let bytes = runtime.block_on(async {
        fetch_url_async(fileurl).await
    })?;

    // Store in cache
    if cache_enabled {
        FILE_CACHE.write().insert(fileurl.to_string(), bytes.clone());
    }

    Ok(bytes)
}

/// Async function to fetch URL using hyper (HTTP only).
async fn fetch_http(url_str: &str, uri: http::Uri, host: String) -> Result<Vec<u8>> {
    use hyper::{Body, Client, Request};
    use hyper::client::HttpConnector;

    // Create HTTP client
    let client: Client<HttpConnector, Body> = Client::new();

    let req = Request::builder()
        .uri(uri)
        .header("Host", host)
        .header("User-Agent", "gnostic-compiler/0.1.0")
        .body(Body::empty())
        .map_err(|e| CompilerError::Http(format!("Failed to build request: {}", e)))?;

    let response = client.request(req).await
        .map_err(|e| CompilerError::Http(format!("Failed to fetch {}: {}", url_str, e)))?;

    if !response.status().is_success() {
        return Err(CompilerError::Http(format!(
            "Error downloading {}: {}",
            url_str,
            response.status()
        )));
    }

    let body_bytes = hyper::body::to_bytes(response.into_body()).await
        .map_err(|e| CompilerError::Http(format!("Failed to read response body: {}", e)))?;

    Ok(body_bytes.to_vec())
}

/// Async function to fetch URL using hyper.
async fn fetch_url_async(url_str: &str) -> Result<Vec<u8>> {
    use http::Uri;

    let uri: Uri = url_str.parse()
        .map_err(|e| CompilerError::Http(format!("Invalid URL {}: {}", url_str, e)))?;

    let host = uri.host()
        .ok_or_else(|| CompilerError::Http(format!("No host in URL: {}", url_str)))?
        .to_string();

    let scheme = uri.scheme_str().unwrap_or("http");

    if scheme == "https" {
        // HTTPS not supported without additional dependencies
        return Err(CompilerError::Http(format!(
            "HTTPS URLs are not supported. Please download the file locally first: {}",
            url_str
        )));
    }

    fetch_http(url_str, uri, host).await
}

/// Reads bytes from a file (local or URL).
pub fn read_bytes_for_file(filename: &str) -> Result<Vec<u8>> {
    // Check if it's a URL
    if let Ok(url) = Url::parse(filename) {
        if url.scheme() == "http" || url.scheme() == "https" {
            return fetch_file(filename);
        }
    }

    // Local file
    std::fs::read(filename).map_err(|e| CompilerError::Io(format!("Failed to read {}: {}", filename, e)))
}

/// Parses bytes as YAML.
pub fn read_info_from_bytes(filename: &str, bytes: &[u8]) -> Result<Yaml> {
    let cache_enabled = INFO_CACHE_ENABLED.load(Ordering::SeqCst);
    let verbose = VERBOSE_READER.load(Ordering::SeqCst);

    // Check cache first
    if cache_enabled && !filename.is_empty() {
        if let Some(info) = INFO_CACHE.read().get(filename) {
            if verbose {
                log::info!("Cache hit info for file {}", filename);
            }
            return Ok(info.clone());
        }
        if verbose {
            log::info!("Reading info for file {}", filename);
        }
    }

    // Parse YAML
    let content = std::str::from_utf8(bytes)
        .map_err(|e| CompilerError::Yaml(format!("Invalid UTF-8: {}", e)))?;

    let yaml: Yaml = serde_yaml::from_str(content)?;

    // Store in cache
    if cache_enabled && !filename.is_empty() {
        INFO_CACHE.write().insert(filename.to_string(), yaml.clone());
    }

    Ok(yaml)
}

/// Reads a file and returns the parsed YAML.
pub fn read_info_for_file(filename: &str) -> Result<Yaml> {
    let bytes = read_bytes_for_file(filename)?;
    read_info_from_bytes(filename, &bytes)
}

/// Reads a file and returns the fragment needed to resolve a $ref.
pub fn read_info_for_ref(basefile: &str, reference: &str) -> Result<Yaml> {
    let cache_enabled = INFO_CACHE_ENABLED.load(Ordering::SeqCst);
    let verbose = VERBOSE_READER.load(Ordering::SeqCst);

    // Check cache first
    if cache_enabled {
        if let Some(info) = INFO_CACHE.read().get(reference) {
            if verbose {
                log::info!("Cache hit for ref {}#{}", basefile, reference);
            }
            return Ok(info.clone());
        }
        if verbose {
            log::info!("Reading info for ref {}#{}", basefile, reference);
        }
    }

    // Split reference into file and path parts
    let parts: Vec<&str> = reference.splitn(2, '#').collect();
    let filename = if !parts[0].is_empty() {
        // Check if it's a URL
        if Url::parse(parts[0]).is_ok() {
            parts[0].to_string()
        } else {
            // Local file - resolve relative to base
            let basedir = Path::new(basefile)
                .parent()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default();
            if basedir.is_empty() {
                parts[0].to_string()
            } else {
                format!("{}/{}", basedir, parts[0])
            }
        }
    } else {
        basefile.to_string()
    };

    // Read and parse the file
    let bytes = read_bytes_for_file(&filename)?;
    let mut info = read_info_from_bytes(&filename, &bytes)?;

    // Handle document node (serde_yaml returns single value, not array)
    // Navigate to the referenced path
    if parts.len() > 1 && !parts[1].is_empty() {
        let path: Vec<&str> = parts[1].split('/').collect();
        for (i, key) in path.iter().enumerate() {
            if i > 0 && !key.is_empty() {
                // Skip empty keys (from leading /)
                if let Yaml::Mapping(ref map) = info {
                    if let Some(value) = map.get(&Yaml::String((*key).to_string())) {
                        info = value.clone();
                    } else {
                        if cache_enabled {
                            INFO_CACHE.write().insert(reference.to_string(), Yaml::Null);
                        }
                        return Err(CompilerError::Simple(format!(
                            "could not resolve {}",
                            reference
                        )));
                    }
                } else {
                    return Err(CompilerError::Simple(format!(
                        "could not resolve {}",
                        reference
                    )));
                }
            }
        }
    }

    // Store in cache
    if cache_enabled {
        INFO_CACHE.write().insert(reference.to_string(), info.clone());
    }

    Ok(info)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_operations() {
        clear_caches();
        enable_file_cache();
        enable_info_cache();

        // These are just smoke tests for the cache operations
        assert!(FILE_CACHE_ENABLED.load(Ordering::SeqCst));
        assert!(INFO_CACHE_ENABLED.load(Ordering::SeqCst));

        disable_file_cache();
        disable_info_cache();

        assert!(!FILE_CACHE_ENABLED.load(Ordering::SeqCst));
        assert!(!INFO_CACHE_ENABLED.load(Ordering::SeqCst));

        // Re-enable for other tests
        enable_file_cache();
        enable_info_cache();
    }

    #[test]
    fn test_read_info_from_bytes() {
        let yaml_content = b"name: test\nvalue: 123";
        let result = read_info_from_bytes("test.yaml", yaml_content);
        assert!(result.is_ok());

        let yaml = result.unwrap();
        assert!(matches!(yaml, Yaml::Mapping(_)));
    }
}
