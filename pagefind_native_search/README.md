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
use pagefind_native_search::{NativeSearch, RankingWeights};
use pagefind_core_search::SearchOptions;

// Create a new native search instance
let mut search = NativeSearch::new("/path/to/pagefind/bundle")?;
search.init()?;

// Set language (optional)
search.set_language("en".to_string());

// Set custom ranking weights (optional)
search.set_ranking_weights(RankingWeights::default());

// Perform a search
let results = search.search("query", Some(SearchOptions::default()))?;
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

## Status

This is a skeleton implementation. The following features need to be implemented:
- Actual file decompression logic
- Integration with pagefind_core_search for search operations
- Fragment loading for full page content
- Excerpt generation
- Metadata extraction