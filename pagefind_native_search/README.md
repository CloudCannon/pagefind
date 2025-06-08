# pagefind_native_search

Native search capabilities for Pagefind - file system based search implementation.

## Overview

This crate provides native search functionality for Pagefind indexes, allowing searches to be performed directly from Rust without requiring a browser or WebAssembly environment. It includes both a library interface and a CLI tool.

## Structure

- `file_loader/` - Native file loading and decompression logic
- `config/` - Configuration structures and parsing
- `cli/` - CLI-specific utilities and helpers

## Library Usage

```rust
use pagefind_native_search::{NativeSearch, SearchOptions};
use pagefind_core_search::RankingWeights;
use std::collections::HashMap;

// Create a new native search instance
let mut search = NativeSearch::new("/path/to/pagefind/bundle")?;

// Initialize with optional language preference
search.init(Some("en"))?;

// Set custom ranking weights (optional)
search.set_ranking_weights(RankingWeights::default());

// Perform a search
let options = SearchOptions {
    filters: HashMap::new(),
    sort: None,
};
let results = search.search("query", options)?;

// Process results
for result in results.results {
    println!("Page: {} (score: {})", result.page, result.page_score);
    
    // Load fragment for more details
    let fragment = search.load_fragment(&result.page)?;
    println!("URL: {}", fragment.url);
}
```

## CLI Usage

```bash
# Search a Pagefind index
pagefind-search search --bundle /path/to/bundle --query "search term"

# Search with filters
pagefind-search search --bundle /path/to/bundle --query "search term" \
  --filters '{"category": ["tech"], "author": ["alice"]}'

# List available filters
pagefind-search filters --bundle /path/to/bundle

# Output as JSON
pagefind-search search --bundle /path/to/bundle --query "search term" --output json
```

## Features

- Local file system access to Pagefind indexes
- Support for compressed (pagefind_dcd) file format
- Filter and sort support
- JSON and human-readable output formats
- Progress indicators for long operations

## Implementation Status

✅ **Implemented:**
- File loading and gzip decompression with `pagefind_dcd` magic byte detection
- Entry file (`pagefind-entry.json`) parsing
- Metadata loading and CBOR decoding
- Index chunk loading with lazy loading based on search terms
- Filter chunk loading and filtering support
- Fragment loading (JSON format)
- Search functionality (term search and exact phrase search)
- Filter support with bitset operations
- Sorting support
- CLI interface with search and filter commands
- JSON and text output formats

🚧 **TODO:**
- Excerpt generation from fragments
- Highlighting support in search results
- Sub-result calculation for headings/anchors
- Performance optimizations for large indexes
- More comprehensive error handling
- Additional language stemming support

## File Format Support

The implementation supports all Pagefind file formats:

- `pagefind-entry.json`: Entry point with language information
- `pagefind.{hash}.pf_meta`: Metadata files (CBOR encoded, optionally gzipped)
- `index/{hash}.pf_index`: Index chunks (CBOR encoded, optionally gzipped)
- `filter/{hash}.pf_filter`: Filter chunks (CBOR encoded, optionally gzipped)
- `fragment/{hash}.pf_fragment`: Page fragments (JSON, optionally gzipped)

Files are automatically decompressed if they are gzipped and contain the `pagefind_dcd` magic bytes.