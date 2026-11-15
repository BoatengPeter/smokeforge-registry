use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::{Document, SearchError, SearchOptions, SearchResult};

pub struct Meilisearch {
    base_url: String,
    index: String,
    api_key: String,
    client: Client,
}

impl Meilisearch {
    pub fn new(config: &crate::config::Config) -> Result<Self, SearchError> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(
                config.search_timeout_seconds,
            ))
            .build()
            .map_err(|error| SearchError::Unavailable(error.to_string()))?;

        Ok(Self {
            base_url: config.search_url.trim_end_matches('/').to_owned(),
            index: config.search_index.clone(),
            api_key: config.search_api_key.clone(),
            client,
        })
    }

    pub async fn search(
        &self,
        query: String,
        options: SearchOptions,
    ) -> Result<SearchResult, SearchError> {
        let payload = SearchRequest {
            q: query.clone(),
            limit: options.limit,
            offset: options.offset,
        };
        let url = format!("{}/indexes/{}/search", self.base_url, self.index);
        let response = self
            .client
            .post(url)
            .bearer_auth(&self.api_key)
            .json(&payload)
            .send()
            .await
            .map_err(|error| SearchError::Unavailable(error.to_string()))?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(SearchResult {
                query,
                hits: Vec::new(),
                estimated_total_hits: 0,
                limit: options.limit,
                offset: options.offset,
                processing_time_ms: 0,
            });
        }

        if !response.status().is_success() {
            return Err(SearchError::Unavailable(format!(
                "status {}",
                response.status()
            )));
        }

        let body: SearchResponse = response
            .json()
            .await
            .map_err(|error| SearchError::Unavailable(error.to_string()))?;

        Ok(SearchResult {
            query,
            hits: body.hits,
            estimated_total_hits: body.estimated_total_hits,
            limit: body.limit,
            offset: body.offset,
            processing_time_ms: body.processing_time_ms,
        })
    }

    pub async fn upsert(&self, documents: Vec<Document>) -> Result<(), SearchError> {
        let url = format!(
            "{}/indexes/{}/documents?primaryKey=id",
            self.base_url, self.index
        );
        let response = self
            .client
            .post(url)
            .bearer_auth(&self.api_key)
            .json(&documents)
            .send()
            .await
            .map_err(|error| SearchError::Unavailable(error.to_string()))?;

        if !response.status().is_success() {
            return Err(SearchError::Unavailable(format!(
                "status {}",
                response.status()
            )));
        }

        Ok(())
    }

    pub async fn delete(&self, ids: Vec<String>) -> Result<(), SearchError> {
        let url = format!(
            "{}/indexes/{}/documents/delete-batch",
            self.base_url, self.index
        );
        let response = self
            .client
            .post(url)
            .bearer_auth(&self.api_key)
            .json(&ids)
            .send()
            .await
            .map_err(|error| SearchError::Unavailable(error.to_string()))?;

        if !response.status().is_success() {
            return Err(SearchError::Unavailable(format!(
                "status {}",
                response.status()
            )));
        }

        Ok(())
    }
}

#[derive(Debug, Serialize)]
struct SearchRequest {
    q: String,
    limit: u64,
    offset: u64,
}

#[derive(Debug, Deserialize)]
struct SearchResponse {
    #[serde(default)]
    hits: Vec<serde_json::Value>,
    #[serde(rename = "estimatedTotalHits", default)]
    estimated_total_hits: i64,
    #[serde(default)]
    limit: u64,
    #[serde(default)]
    offset: u64,
    #[serde(rename = "processingTimeMs", default)]
    processing_time_ms: i64,
}
