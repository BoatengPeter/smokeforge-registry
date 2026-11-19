//! UI-independent domain types.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LibraryItem {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub status: String,
    pub tags: Vec<String>,
    pub version: i64,
    pub created_at: String,
    pub updated_at: String,
}
