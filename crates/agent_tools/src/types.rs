use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Reqwest(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Serde(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Reqwest(err) => write!(f, "Reqwest error: {}", err),
            Error::Serde(err) => write!(f, "Serde error: {}", err),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub content: String,
    pub description: Option<String>,
}

#[async_trait]
pub trait SearchEngine: Send + Sync  {
    async fn search(&self, query: &str, save_html_page: bool) -> Result<Vec<SearchResult>, Error>;

    fn name(&self) -> String;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryDescription {
    #[serde(rename = "queryNum")]
    pub query_num: u32,
    pub query: String,
    pub description: String,
}