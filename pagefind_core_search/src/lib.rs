//! Core search functionality for Pagefind
//! 
//! This crate provides platform-agnostic search algorithms and data structures
//! for searching Pagefind indexes. It is designed to be used by both the
//! WebAssembly interface (pagefind_web) and native implementations.

pub mod search;
pub mod index;
pub mod filter;
pub mod sort;
pub mod utils;

// Re-export commonly used types at the crate root
pub use search::{SearchIndex, SearchOptions, SearchResult};
pub use index::{IndexChunk, Page, PageWord};
pub use filter::{FilterChunk, FilterSet};
pub use sort::{SortOptions, SortOrder};

/// Ranking weights for search result scoring
#[derive(Debug, Clone, Copy)]
pub struct RankingWeights {
    pub page_length: f32,
    pub term_frequency: f32,
    pub term_similarity: f32,
    pub term_saturation: f32,
}

impl Default for RankingWeights {
    fn default() -> Self {
        Self {
            page_length: 0.75,
            term_frequency: 1.0,
            term_similarity: 1.0,
            term_saturation: 1.5,
        }
    }
}

/// Main entry point for initializing a search index
pub fn init_search_index() -> SearchIndex {
    SearchIndex::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_ranking_weights() {
        let weights = RankingWeights::default();
        assert_eq!(weights.page_length, 0.75);
        assert_eq!(weights.term_frequency, 1.0);
        assert_eq!(weights.term_similarity, 1.0);
        assert_eq!(weights.term_saturation, 1.5);
    }
}