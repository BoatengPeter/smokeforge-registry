//! UI-independent domain types and validation.

use garde::Validate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LibraryStatus {
    #[default]
    Draft,
    Active,
    Archived,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LibraryItem {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub status: LibraryStatus,
    pub tags: Vec<String>,
    pub version: i64,
    pub sync_status: SyncStatus,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SyncStatus {
    #[default]
    Local,
    Pending,
    Synced,
    Conflict,
    Failed,
}

#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("the record is invalid: {0}")]
    Validation(String),
}

#[derive(Debug, Clone, Validate)]
pub struct LibraryDraft {
    #[garde(length(min = 1, max = 160))]
    pub title: String,
    #[garde(length(max = 5000))]
    pub description: String,
    #[garde(skip)]
    pub status: LibraryStatus,
    #[garde(length(max = 12))]
    pub tags: Vec<String>,
}

impl LibraryDraft {}

impl Default for LibraryDraft {
    fn default() -> Self {
        Self {
            title: String::new(),
            description: String::new(),
            status: LibraryStatus::Draft,
            tags: Vec::new(),
        }
    }
}

impl From<&LibraryItem> for LibraryDraft {
    fn from(item: &LibraryItem) -> Self {
        Self {
            title: item.title.clone(),
            description: item.description.clone(),
            status: item.status,
            tags: item.tags.clone(),
        }
    }
}

impl LibraryStatus {
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Self::Draft => "Draft",
            Self::Active => "Active",
            Self::Archived => "Archived",
        }
    }
}
