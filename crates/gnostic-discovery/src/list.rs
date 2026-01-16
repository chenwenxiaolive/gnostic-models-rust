//! Google APIs Discovery Service client.

use serde::{Deserialize, Serialize};

/// URL for the Google APIs Discovery Service.
pub const APIS_LIST_SERVICE_URL: &str = "https://www.googleapis.com/discovery/v1/apis";

/// Represents the list of APIs from the Discovery Service.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiList {
    /// Kind of the response.
    pub kind: String,
    /// Discovery version.
    #[serde(rename = "discoveryVersion")]
    pub discovery_version: String,
    /// List of APIs.
    pub items: Vec<Api>,
}

/// Represents a single API in the Discovery list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Api {
    /// Kind of the item.
    pub kind: String,
    /// API ID.
    pub id: String,
    /// API name.
    pub name: String,
    /// API version.
    pub version: String,
    /// API title.
    pub title: String,
    /// API description.
    pub description: String,
    /// URL to the Discovery REST document.
    #[serde(rename = "discoveryRestUrl")]
    pub discovery_rest_url: String,
    /// URL to the API documentation.
    #[serde(rename = "documentationLink", default)]
    pub documentation_link: String,
    /// Whether this is the preferred version.
    #[serde(default)]
    pub preferred: bool,
}

impl ApiList {
    /// Fetches the list of APIs from the Discovery Service.
    pub fn fetch() -> Result<Self, String> {
        let response = reqwest::blocking::get(APIS_LIST_SERVICE_URL)
            .map_err(|e| format!("Failed to fetch API list: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("HTTP error: {}", response.status()));
        }

        let list: ApiList = response
            .json()
            .map_err(|e| format!("Failed to parse API list: {}", e))?;

        Ok(list)
    }

    /// Parses the API list from JSON bytes.
    pub fn parse(bytes: &[u8]) -> Result<Self, String> {
        serde_json::from_slice(bytes).map_err(|e| format!("Failed to parse: {}", e))
    }

    /// Finds an API by name and version.
    pub fn api_with_name_and_version(&self, name: &str, version: &str) -> Option<&Api> {
        self.items
            .iter()
            .find(|api| api.name == name && api.version == version)
    }

    /// Finds the preferred version of an API by name.
    pub fn preferred_api(&self, name: &str) -> Option<&Api> {
        self.items
            .iter()
            .find(|api| api.name == name && api.preferred)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_api_list() {
        let json = r#"{
            "kind": "discovery#directoryList",
            "discoveryVersion": "v1",
            "items": [
                {
                    "kind": "discovery#directoryItem",
                    "id": "test:v1",
                    "name": "test",
                    "version": "v1",
                    "title": "Test API",
                    "description": "A test API",
                    "discoveryRestUrl": "https://example.com/test/v1/rest",
                    "preferred": true
                }
            ]
        }"#;

        let list = ApiList::parse(json.as_bytes()).unwrap();
        assert_eq!(list.kind, "discovery#directoryList");
        assert_eq!(list.items.len(), 1);
        assert_eq!(list.items[0].name, "test");
    }
}
