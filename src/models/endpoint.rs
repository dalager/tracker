use serde::{Deserialize, Serialize};
use std::fmt;

/// HTTP methods supported by the tracker CLI
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Head,
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpMethod::Get => write!(f, "GET"),
            HttpMethod::Post => write!(f, "POST"),
            HttpMethod::Put => write!(f, "PUT"),
            HttpMethod::Patch => write!(f, "PATCH"),
            HttpMethod::Delete => write!(f, "DELETE"),
            HttpMethod::Head => write!(f, "HEAD"),
        }
    }
}

impl HttpMethod {
    /// Returns the method as an uppercase string
    pub fn to_uppercase(&self) -> String {
        format!("{}", self)
    }

    /// Returns the method as a string slice
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Patch => "PATCH",
            HttpMethod::Delete => "DELETE",
            HttpMethod::Head => "HEAD",
        }
    }
}

/// Represents a configured API endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Endpoint {
    pub id: String,
    pub name: String,
    pub method: HttpMethod,
    pub url: String,
    pub description: Option<String>,
}

impl Endpoint {
    /// Create a new endpoint
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        method: HttpMethod,
        url: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            method,
            url: url.into(),
            description: None,
        }
    }

    /// Create a new endpoint with a description
    pub fn with_description(
        id: impl Into<String>,
        name: impl Into<String>,
        method: HttpMethod,
        url: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            method,
            url: url.into(),
            description: Some(description.into()),
        }
    }
}

/// Get a list of sample/hardcoded endpoints for demonstration
pub fn get_sample_endpoints() -> Vec<Endpoint> {
    vec![
        Endpoint::with_description(
            "1",
            "list-users",
            HttpMethod::Get,
            "/users",
            "Fetch all users",
        ),
        Endpoint::with_description(
            "2",
            "create-user",
            HttpMethod::Post,
            "/users",
            "Create a new user",
        ),
        Endpoint::with_description(
            "3",
            "get-user",
            HttpMethod::Get,
            "/users/{id}",
            "Get a specific user by ID",
        ),
        Endpoint::with_description(
            "4",
            "update-user",
            HttpMethod::Put,
            "/users/{id}",
            "Update a user",
        ),
        Endpoint::with_description(
            "5",
            "delete-user",
            HttpMethod::Delete,
            "/users/{id}",
            "Delete a user",
        ),
    ]
}