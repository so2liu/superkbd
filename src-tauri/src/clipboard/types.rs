use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClipboardContent {
    pub content_type: ContentType,
    pub text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ContentType {
    Text,
    Image,
    File,
    Html,
}

impl ClipboardContent {
    pub fn new_text(text: String) -> Self {
        Self {
            content_type: ContentType::Text,
            text: Some(text),
        }
    }

    pub fn is_empty(&self) -> bool {
        match &self.text {
            Some(t) => t.trim().is_empty(),
            None => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard_content_creation() {
        let content = ClipboardContent::new_text("Hello".to_string());
        assert_eq!(content.content_type, ContentType::Text);
        assert_eq!(content.text, Some("Hello".to_string()));
        assert!(!content.is_empty());
    }

    #[test]
    fn test_empty_detection() {
        let empty = ClipboardContent::new_text("  ".to_string());
        assert!(empty.is_empty());

        let not_empty = ClipboardContent::new_text("text".to_string());
        assert!(!not_empty.is_empty());
    }
}
