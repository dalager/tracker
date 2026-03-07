use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Http(String),
    Network(String),
    Parse(String),
    Validation(String),
    Io(std::io::Error),
    Reqwest(reqwest::Error),
    SerdeJson(serde_json::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Http(msg) => write!(f, "HTTP Error: {}", msg),
            Self::Network(msg) => write!(f, "Network Error: {}", msg),
            Self::Parse(msg) => write!(f, "Parse Error: {}", msg),
            Self::Validation(msg) => write!(f, "Validation Error: {}", msg),
            Self::Io(e) => write!(f, "IO Error: {}", e),
            Self::Reqwest(e) => write!(f, "Request Error: {}", e),
            Self::SerdeJson(e) => write!(f, "JSON Error: {}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::Reqwest(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::SerdeJson(e)
    }
}