use std::sync::Arc;

use axum::{Extension, Json, Router, extract::Query, routing::get};
use serde::{Deserialize, Serialize};

use super::{SearchError, SearchService};
use crate::error::AppError;

pub fn router(service: Arc<SearchService>) -> Router {
    Router::new()
        .route("/api/v1/search", get(search))
        .layer(Extension(service))
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct SearchResponse {
    pub hits: Vec<serde_json::Value>,
    pub estimated_total_hits: i64,
    pub limit: u64,
    pub offset: u64,
    pub processing_time_ms: i64,
}

async fn search(
    Extension(service): Extension<Arc<SearchService>>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<SearchResponse>, AppError> {
    let result = service
        .search(
            query.q.as_deref().unwrap_or_default(),
            query.limit,
            query.offset,
        )
        .await
        .map_err(map_error)?;

    Ok(Json(SearchResponse {
        hits: result.hits,
        estimated_total_hits: result.estimated_total_hits,
        limit: result.limit,
        offset: result.offset,
        processing_time_ms: result.processing_time_ms,
    }))
}

fn map_error(error: SearchError) -> AppError {
    match error {
        SearchError::Invalid(detail) => AppError::new(
            axum::http::StatusCode::UNPROCESSABLE_ENTITY,
            "validation_failed",
            "Validation failed",
            detail,
        ),
        SearchError::Unavailable(_) => AppError::new(
            axum::http::StatusCode::SERVICE_UNAVAILABLE,
            "search_unavailable",
            "Search unavailable",
            "The search service is unavailable.",
        ),
    }
}
