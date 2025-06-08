use std::collections::BTreeMap;

pub mod filter;
pub mod index;
pub mod metadata;
pub mod search;
pub mod sort;
pub mod utils;

pub use search::{PageSearchResult, RankingWeights, SearchIndex};

/// Represents a word occurrence on a page
#[derive(Debug, Clone)]
pub struct PageWord {
    pub page: u32,
    pub locs: Vec<(u8, u32)>, // (weight, location)
}

/// Represents a chunk of the search index
#[derive(Debug, Clone)]
pub struct IndexChunk {
    pub from: String,
    pub to: String,
    pub hash: String,
}

/// Represents a page in the search index
#[derive(Debug, Clone)]
pub struct Page {
    pub hash: String,
    pub word_count: u32,
}

/// Main search index structure containing all search data
#[derive(Debug)]
pub struct CoreSearchIndex {
    pub generator_version: Option<String>,
    pub pages: Vec<Page>,
    pub average_page_length: f32,
    pub chunks: Vec<IndexChunk>,
    pub filter_chunks: BTreeMap<String, String>,
    pub words: BTreeMap<String, Vec<PageWord>>,
    pub filters: BTreeMap<String, BTreeMap<String, Vec<u32>>>,
    pub sorts: BTreeMap<String, Vec<u32>>,
    pub ranking_weights: RankingWeights,
}

impl CoreSearchIndex {
    pub fn new() -> Self {
        Self {
            generator_version: None,
            pages: Vec::new(),
            average_page_length: 0.0,
            chunks: Vec::new(),
            filter_chunks: BTreeMap::new(),
            words: BTreeMap::new(),
            filters: BTreeMap::new(),
            sorts: BTreeMap::new(),
            ranking_weights: RankingWeights::default(),
        }
    }
}

impl Default for CoreSearchIndex {
    fn default() -> Self {
        Self::new()
    }
}