//! Utility functions and helpers for search operations

use std::collections::HashMap;

/// Normalize a search query for consistent matching
pub fn normalize_query(query: &str) -> String {
    query
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// Split a query into individual search terms
pub fn tokenize_query(query: &str) -> Vec<String> {
    normalize_query(query)
        .split_whitespace()
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Calculate the similarity between two strings (0.0 to 1.0)
pub fn calculate_similarity(a: &str, b: &str) -> f32 {
    if a == b {
        return 1.0;
    }

    let a_lower = a.to_lowercase();
    let b_lower = b.to_lowercase();

    if a_lower == b_lower {
        return 0.95; // Case-insensitive match
    }

    // Simple prefix matching for now
    if a_lower.starts_with(&b_lower) || b_lower.starts_with(&a_lower) {
        let min_len = a_lower.len().min(b_lower.len()) as f32;
        let max_len = a_lower.len().max(b_lower.len()) as f32;
        return min_len / max_len;
    }

    0.0
}

/// Extract a snippet of text around a given position
pub fn extract_snippet(text: &str, position: usize, context_chars: usize) -> String {
    let start = position.saturating_sub(context_chars);
    let end = (position + context_chars).min(text.len());

    let mut snippet = String::new();

    // Find word boundaries
    let mut actual_start = start;
    if start > 0 {
        while actual_start > 0 && !text.chars().nth(actual_start - 1).unwrap_or(' ').is_whitespace() {
            actual_start -= 1;
        }
        if actual_start != 0 {
            snippet.push_str("...");
        }
    }

    let mut actual_end = end;
    if end < text.len() {
        while actual_end < text.len() && !text.chars().nth(actual_end).unwrap_or(' ').is_whitespace() {
            actual_end += 1;
        }
    }

    snippet.push_str(&text[actual_start..actual_end]);

    if actual_end < text.len() {
        snippet.push_str("...");
    }

    snippet
}

/// Merge multiple metadata maps, with later maps overriding earlier ones
pub fn merge_metadata(maps: Vec<HashMap<String, String>>) -> HashMap<String, String> {
    let mut result = HashMap::new();
    for map in maps {
        result.extend(map);
    }
    result
}

/// Calculate term frequency (TF) for ranking
pub fn calculate_term_frequency(term_count: usize, total_words: usize) -> f32 {
    if total_words == 0 {
        return 0.0;
    }
    (term_count as f32) / (total_words as f32)
}

/// Calculate inverse document frequency (IDF) for ranking
pub fn calculate_idf(docs_with_term: usize, total_docs: usize) -> f32 {
    if docs_with_term == 0 || total_docs == 0 {
        return 0.0;
    }
    ((total_docs as f32) / (docs_with_term as f32)).ln()
}

/// Decode a base64-encoded string
pub fn decode_base64(input: &str) -> Result<Vec<u8>, UtilError> {
    // TODO: Implement base64 decoding
    Ok(Vec::new())
}

/// Errors that can occur in utility functions
#[derive(Debug)]
pub enum UtilError {
    DecodingError(String),
    InvalidInput(String),
}

impl std::fmt::Display for UtilError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UtilError::DecodingError(msg) => write!(f, "Decoding error: {}", msg),
            UtilError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
        }
    }
}

impl std::error::Error for UtilError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_query() {
        assert_eq!(normalize_query("Hello World!"), "hello world");
        assert_eq!(normalize_query("  Multiple   Spaces  "), "multiple spaces");
        assert_eq!(normalize_query("Special@#$Characters"), "specialcharacters");
    }

    #[test]
    fn test_tokenize_query() {
        let tokens = tokenize_query("Hello World Test");
        assert_eq!(tokens, vec!["hello", "world", "test"]);
    }

    #[test]
    fn test_calculate_similarity() {
        assert_eq!(calculate_similarity("test", "test"), 1.0);
        assert_eq!(calculate_similarity("Test", "test"), 0.95);
        assert!(calculate_similarity("testing", "test") > 0.5);
        assert_eq!(calculate_similarity("completely", "different"), 0.0);
    }

    #[test]
    fn test_extract_snippet() {
        let text = "This is a test sentence for extracting snippets from text.";
        let snippet = extract_snippet(text, 15, 10);
        assert!(snippet.contains("test sentence"));
    }

    #[test]
    fn test_tf_idf_calculations() {
        assert_eq!(calculate_term_frequency(5, 100), 0.05);
        assert_eq!(calculate_term_frequency(0, 100), 0.0);
        
        let idf = calculate_idf(10, 1000);
        assert!(idf > 4.0 && idf < 5.0); // ln(100) ≈ 4.6
    }
}