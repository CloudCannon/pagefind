//! Index data structures and parsing

use std::collections::HashMap;

/// Represents a chunk of the search index
#[derive(Debug)]
pub struct IndexChunk {
    pub hash: String,
    pub pages: Vec<Page>,
    pub words: HashMap<String, Vec<PageWord>>,
}

impl IndexChunk {
    /// Parse an index chunk from raw bytes
    pub fn from_bytes(data: &[u8]) -> Result<Self, IndexError> {
        // TODO: Implement parsing logic
        Ok(Self {
            hash: String::new(),
            pages: Vec::new(),
            words: HashMap::new(),
        })
    }
}

/// Represents a page in the index
#[derive(Debug, Clone)]
pub struct Page {
    pub id: String,
    pub url: String,
    pub title: String,
    pub content: String,
    pub word_count: usize,
    pub meta: HashMap<String, String>,
    pub filters: HashMap<String, Vec<String>>,
}

impl Page {
    /// Create a new page
    pub fn new(id: String, url: String) -> Self {
        Self {
            id,
            url,
            title: String::new(),
            content: String::new(),
            word_count: 0,
            meta: HashMap::new(),
            filters: HashMap::new(),
        }
    }
}

/// Represents a word occurrence in a page
#[derive(Debug, Clone)]
pub struct PageWord {
    pub page_id: String,
    pub positions: Vec<usize>,
    pub weight: f32,
}

impl PageWord {
    /// Create a new page word reference
    pub fn new(page_id: String) -> Self {
        Self {
            page_id,
            positions: Vec::new(),
            weight: 1.0,
        }
    }

    /// Add a position where this word occurs
    pub fn add_position(&mut self, position: usize) {
        self.positions.push(position);
    }
}

/// Metadata about the entire index
#[derive(Debug)]
pub struct IndexMetadata {
    pub version: String,
    pub languages: Vec<String>,
    pub total_pages: usize,
    pub total_words: usize,
    pub chunks: Vec<ChunkMetadata>,
}

/// Metadata about a single index chunk
#[derive(Debug)]
pub struct ChunkMetadata {
    pub hash: String,
    pub page_count: usize,
    pub word_count: usize,
}

/// Errors that can occur during index operations
#[derive(Debug)]
pub enum IndexError {
    ParseError(String),
    InvalidFormat(String),
    CorruptedData(String),
}

impl std::fmt::Display for IndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IndexError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            IndexError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
            IndexError::CorruptedData(msg) => write!(f, "Corrupted data: {}", msg),
        }
    }
}

impl std::error::Error for IndexError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_creation() {
        let page = Page::new("test-id".to_string(), "/test/url".to_string());
        assert_eq!(page.id, "test-id");
        assert_eq!(page.url, "/test/url");
        assert_eq!(page.word_count, 0);
    }

    #[test]
    fn test_page_word_positions() {
        let mut word = PageWord::new("page-1".to_string());
        word.add_position(10);
        word.add_position(25);
        assert_eq!(word.positions.len(), 2);
        assert_eq!(word.positions[0], 10);
        assert_eq!(word.positions[1], 25);
    }
}