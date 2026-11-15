//! Provider-neutral search interfaces and Meilisearch adapter.

pub mod http;
pub mod meilisearch;
// smokeforge:anchor search-modules

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    #[serde(flatten)]
    pub fields: serde_json::Map<String, serde_json::Value>,
}

#[derive(Clone, Copy, Debug)]
pub struct SearchOptions {
    pub limit: u64,
    pub offset: u64,
}

#[derive(Clone, Debug, Serialize)]
pub struct SearchResult {
    pub query: String,
    pub hits: Vec<serde_json::Value>,
    pub estimated_total_hits: i64,
    pub limit: u64,
    pub offset: u64,
    pub processing_time_ms: i64,
}

#[derive(Debug, Error)]
pub enum SearchError {
    #[error("invalid search input: {0}")]
    Invalid(String),
    #[error("search unavailable: {0}")]
    Unavailable(String),
}

pub trait Searcher: Send + Sync {
    fn search(
        &self,
        query: String,
        options: SearchOptions,
    ) -> impl std::future::Future<Output = Result<SearchResult, SearchError>> + Send;
}

pub trait Indexer: Send + Sync {
    fn upsert(
        &self,
        documents: Vec<Document>,
    ) -> impl std::future::Future<Output = Result<(), SearchError>> + Send;

    fn delete(
        &self,
        ids: Vec<String>,
    ) -> impl std::future::Future<Output = Result<(), SearchError>> + Send;
}

pub struct SearchService {
    engine: meilisearch::Meilisearch,
    max_limit: u64,
}

impl SearchService {
    pub fn new(engine: meilisearch::Meilisearch, max_limit: u64) -> Self {
        Self { engine, max_limit }
    }

    pub async fn search(
        &self,
        query: &str,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Result<SearchResult, SearchError> {
        let query = query.trim();
        if query.is_empty() || query.chars().count() > 200 {
            return Err(SearchError::Invalid(
                "query must contain 1 to 200 characters".to_owned(),
            ));
        }

        let limit = limit.unwrap_or(20);
        let offset = offset.unwrap_or(0);

        if limit == 0 || limit > self.max_limit {
            return Err(SearchError::Invalid(format!(
                "limit must be between 1 and {}",
                self.max_limit
            )));
        }

        if offset > 1000 {
            return Err(SearchError::Invalid(
                "offset must be between 0 and 1000".to_owned(),
            ));
        }

        self.engine
            .search(query.to_owned(), SearchOptions { limit, offset })
            .await
    }
}
