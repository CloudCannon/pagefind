//! Filter functionality for search results

use std::collections::{HashMap, HashSet};

/// Represents a set of filters to apply to search results
#[derive(Debug, Clone, Default)]
pub struct FilterSet {
    /// Map of filter name to allowed values
    pub filters: HashMap<String, HashSet<String>>,
    /// Whether to use AND or OR logic between different filter types
    pub mode: FilterMode,
}

impl FilterSet {
    /// Create a new empty filter set
    pub fn new() -> Self {
        Self {
            filters: HashMap::new(),
            mode: FilterMode::And,
        }
    }

    /// Add a filter with allowed values
    pub fn add_filter(&mut self, name: String, values: Vec<String>) {
        let value_set: HashSet<String> = values.into_iter().collect();
        self.filters.insert(name, value_set);
    }

    /// Check if a page matches the filter set
    pub fn matches(&self, page_filters: &HashMap<String, Vec<String>>) -> bool {
        if self.filters.is_empty() {
            return true;
        }

        match self.mode {
            FilterMode::And => self.matches_all(page_filters),
            FilterMode::Or => self.matches_any(page_filters),
        }
    }

    fn matches_all(&self, page_filters: &HashMap<String, Vec<String>>) -> bool {
        for (filter_name, allowed_values) in &self.filters {
            if let Some(page_values) = page_filters.get(filter_name) {
                let has_match = page_values.iter()
                    .any(|v| allowed_values.contains(v));
                if !has_match {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    fn matches_any(&self, page_filters: &HashMap<String, Vec<String>>) -> bool {
        for (filter_name, allowed_values) in &self.filters {
            if let Some(page_values) = page_filters.get(filter_name) {
                let has_match = page_values.iter()
                    .any(|v| allowed_values.contains(v));
                if has_match {
                    return true;
                }
            }
        }
        false
    }
}

/// Mode for combining multiple filters
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterMode {
    /// All filters must match (default)
    And,
    /// At least one filter must match
    Or,
}

impl Default for FilterMode {
    fn default() -> Self {
        FilterMode::And
    }
}

/// Represents a chunk of filter data
#[derive(Debug)]
pub struct FilterChunk {
    pub hash: String,
    pub filters: HashMap<String, FilterData>,
}

impl FilterChunk {
    /// Parse a filter chunk from raw bytes
    pub fn from_bytes(data: &[u8]) -> Result<Self, FilterError> {
        // TODO: Implement parsing logic
        Ok(Self {
            hash: String::new(),
            filters: HashMap::new(),
        })
    }
}

/// Data about a specific filter
#[derive(Debug)]
pub struct FilterData {
    pub name: String,
    pub values: HashMap<String, usize>, // value -> count
}

/// Errors that can occur during filter operations
#[derive(Debug)]
pub enum FilterError {
    ParseError(String),
    InvalidFilter(String),
}

impl std::fmt::Display for FilterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FilterError::ParseError(msg) => write!(f, "Filter parse error: {}", msg),
            FilterError::InvalidFilter(msg) => write!(f, "Invalid filter: {}", msg),
        }
    }
}

impl std::error::Error for FilterError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_set_creation() {
        let filter_set = FilterSet::new();
        assert_eq!(filter_set.filters.len(), 0);
        assert_eq!(filter_set.mode, FilterMode::And);
    }

    #[test]
    fn test_filter_matching_and_mode() {
        let mut filter_set = FilterSet::new();
        filter_set.add_filter("category".to_string(), vec!["tech".to_string(), "news".to_string()]);
        filter_set.add_filter("author".to_string(), vec!["alice".to_string()]);

        let mut page_filters = HashMap::new();
        page_filters.insert("category".to_string(), vec!["tech".to_string()]);
        page_filters.insert("author".to_string(), vec!["alice".to_string()]);

        // Should match with AND mode
        assert!(filter_set.matches(&page_filters));

        // Should not match if missing a filter
        page_filters.remove("author");
        assert!(!filter_set.matches(&page_filters));

        // Should match with OR mode
        filter_set.mode = FilterMode::Or;
        assert!(filter_set.matches(&page_filters));
    }
}