use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ClipboardEntry {
    pub id: i64,
    pub content_type: String,
    pub text_content: Option<String>,
    pub file_path: Option<String>,
    pub metadata: Option<String>,
    pub created_at: i64,
    pub favorite: bool,
    pub source_app: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewClipboardEntry {
    pub content_type: String,
    pub text_content: Option<String>,
    pub file_path: Option<String>,
    pub metadata: Option<String>,
    pub source_app: Option<String>,
}

impl NewClipboardEntry {
    pub fn new_text(content: String) -> Self {
        Self {
            content_type: "text".to_string(),
            text_content: Some(content),
            file_path: None,
            metadata: None,
            source_app: None,
        }
    }

    pub fn new_text_with_source(content: String, source_app: Option<String>) -> Self {
        Self {
            content_type: "text".to_string(),
            text_content: Some(content),
            file_path: None,
            metadata: None,
            source_app,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardSearchParams {
    pub query: Option<String>,
    pub content_type: Option<String>,
    pub favorites_only: bool,
    pub limit: i64,
    pub offset: i64,
}

impl Default for ClipboardSearchParams {
    fn default() -> Self {
        Self {
            query: None,
            content_type: None,
            favorites_only: false,
            limit: 100,
            offset: 0,
        }
    }
}
