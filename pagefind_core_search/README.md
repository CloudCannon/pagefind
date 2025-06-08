# pagefind_core_search

Core search functionality for Pagefind - platform-agnostic search algorithms and data structures.

## Overview

This crate provides the core search logic extracted from the WebAssembly implementation, designed to be used by both the WASM interface (`pagefind_web`) and native implementations (`pagefind_native_search`).

## Structure

- `search/` - Main search index and algorithms
- `index/` - Index data structures and parsing
- `filter/` - Filter functionality for search results
- `sort/` - Sorting functionality for search results
- `utils/` - Utility functions and helpers

## Usage

```rust
use pagefind_core_search::{SearchIndex, SearchOptions, RankingWeights};

// Create a new search index
let mut index = SearchIndex::new();

// Set custom ranking weights
index.set_ranking_weights(RankingWeights {
    page_length: 0.75,
    term_frequency: 1.0,
    term_similarity: 1.0,
    term_saturation: 1.5,
});

// Load index chunks (implementation pending)
// index.load_chunk(&chunk_data)?;

// Perform a search (implementation pending)
let results = index.search("query", SearchOptions::default());
```

## Status

This is a skeleton implementation. The actual search logic needs to be extracted from `pagefind_web` and integrated here.