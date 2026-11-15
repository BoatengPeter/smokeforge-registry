//! Signed cursor and offset pagination helpers.

mod cursor;
mod offset;
#[cfg(test)]
mod tests;

pub use cursor::Codec;
pub use offset::{parse_cursor, parse_offset};

use serde::Serialize;
use thiserror::Error;

/// Input bounds shared by cursor and offset pagination.
#[derive(Clone, Copy, Debug)]
pub struct Limits {
    pub default_limit: u64,
    pub max_limit: u64,
    pub max_offset: u64,
}

/// One offset-paginated page.
#[derive(Clone, Copy, Debug)]
pub struct Offset {
    pub page: u64,
    pub page_size: u64,
}

impl Offset {
    pub fn offset_value(&self) -> u64 {
        (self.page - 1) * self.page_size
    }
}

/// One cursor-paginated request.
#[derive(Clone, Debug)]
pub struct CursorRequest {
    pub limit: u64,
    pub raw: Option<String>,
}

/// Pagination metadata included in a list response.
#[derive(Clone, Debug, Serialize)]
pub struct Metadata {
    pub mode: String,
    pub limit: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_pages: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    pub has_more: bool,
}

/// One page of items and its metadata.
#[derive(Clone, Debug, Serialize)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub pagination: Metadata,
}

impl<T> Page<T> {
    pub fn new(items: Vec<T>, pagination: Metadata) -> Self {
        Self { items, pagination }
    }
}

impl Offset {
    pub fn metadata(&self, total_items: i64) -> Metadata {
        let total_pages = if self.page_size == 0 {
            0
        } else {
            (total_items.max(0) as u64).div_ceil(self.page_size)
        };

        Metadata {
            mode: "offset".to_owned(),
            limit: self.page_size,
            offset: Some(self.offset_value()),
            page: Some(self.page),
            page_size: Some(self.page_size),
            total_items: Some(total_items),
            total_pages: Some(total_pages),
            next_cursor: None,
            has_more: self.offset_value() + self.page_size < total_items.max(0) as u64,
        }
    }
}

/// Builds metadata for one cursor page.
pub fn cursor_metadata(limit: u64, next_cursor: Option<String>) -> Metadata {
    let has_more = next_cursor.is_some();

    Metadata {
        mode: "cursor".to_owned(),
        limit,
        offset: None,
        page: None,
        page_size: None,
        total_items: None,
        total_pages: None,
        next_cursor,
        has_more,
    }
}

/// Pagination validation failure.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum PaginationError {
    #[error("invalid pagination input: {0}")]
    Invalid(String),
    #[error("invalid cursor")]
    InvalidCursor,
}

impl Limits {
    pub fn validate(&self) -> Result<(), PaginationError> {
        if self.default_limit == 0 || self.max_limit == 0 {
            return Err(PaginationError::Invalid(
                "limits must be positive".to_owned(),
            ));
        }

        if self.default_limit > self.max_limit {
            return Err(PaginationError::Invalid(
                "default limit exceeds max limit".to_owned(),
            ));
        }

        Ok(())
    }
}
