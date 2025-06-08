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

## Configuration

Pagefind Native Search supports configuration from multiple sources with the following precedence (highest to lowest):

1. **CLI arguments** - Command-line flags override all other settings
2. **Environment variables** - Variables prefixed with `PAGEFIND_`
3. **Configuration file** - `pagefind.toml`, `pagefind.yml`, `pagefind.yaml`, or `pagefind.json`
4. **Default values** - Built-in defaults

### Configuration File

Create a configuration file in your project root. The tool will automatically detect and load:
- `pagefind.toml` (TOML format)
- `pagefind.yml` or `pagefind.yaml` (YAML format)
- `pagefind.json` (JSON format)

You can also specify a custom config file path with `--config`:

```bash
pagefind-search --config ./config/search.toml search --query "term"
```

See `pagefind.example.toml` for a complete example configuration file.

### Environment Variables

All configuration options can be set via environment variables with the `PAGEFIND_` prefix:

```bash
export PAGEFIND_BUNDLE=./pagefind
export PAGEFIND_DEFAULT_LIMIT=50
export PAGEFIND_VERBOSE=true
pagefind-search search --query "term"
```

### Configuration Options

| Option | CLI Flag | Environment Variable | Description | Default |
|--------|----------|---------------------|-------------|---------|
| `bundle` | `--bundle` | `PAGEFIND_BUNDLE` | Path to Pagefind bundle directory | Required |
| `language` | `--language` | `PAGEFIND_LANGUAGE` | Force specific language | Auto-detect |
| `default_limit` | `--limit` | `PAGEFIND_DEFAULT_LIMIT` | Default result limit | 30 |
| `output_format` | `--output` | `PAGEFIND_OUTPUT_FORMAT` | Output format (json/text) | text |
| `verbose` | `--verbose` | `PAGEFIND_VERBOSE` | Enable verbose output | false |
| `quiet` | `--quiet` | `PAGEFIND_QUIET` | Only show errors | false |
| `config` | `--config` | - | Custom config file path | Auto-detect |

#### Search-Specific Options

| Option | Environment Variable | Description | Default |
|--------|---------------------|-------------|---------|
| `preload_chunks` | `PAGEFIND_PRELOAD_CHUNKS` | Enable chunk preloading | false |
| `cache_size_mb` | `PAGEFIND_CACHE_SIZE_MB` | Cache size in MB | 50 |
| `generate_excerpts` | `PAGEFIND_GENERATE_EXCERPTS` | Generate search excerpts | true |
| `excerpt_length` | `PAGEFIND_EXCERPT_LENGTH` | Max excerpt length | 300 |
| `excerpt_context` | `PAGEFIND_EXCERPT_CONTEXT` | Context words around matches | 15 |
| `load_fragments` | `PAGEFIND_LOAD_FRAGMENTS` | Enable fragment loading | true |
| `concurrent_fragments` | `PAGEFIND_CONCURRENT_FRAGMENTS` | Max concurrent loads | 5 |

#### Ranking Weights

Control how search results are scored:

| Option | CLI Flag | Environment Variable | Default |
|--------|----------|---------------------|---------|
| `ranking_term_similarity` | `--ranking-term-similarity` | `PAGEFIND_RANKING_TERM_SIMILARITY` | 1.0 |
| `ranking_page_length` | `--ranking-page-length` | `PAGEFIND_RANKING_PAGE_LENGTH` | 0.75 |
| `ranking_term_saturation` | `--ranking-term-saturation` | `PAGEFIND_RANKING_TERM_SATURATION` | 1.5 |
| `ranking_term_frequency` | `--ranking-term-frequency` | `PAGEFIND_RANKING_TERM_FREQUENCY` | 1.0 |

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

# Use custom config file
pagefind-search --config ./search-config.toml search --query "term"

# Override config with CLI args
pagefind-search search --query "term" --limit 10 --verbose
```

## Features

- Local file system access to Pagefind indexes
- Support for compressed (pagefind_dcd) file format
- Filter and sort support
- JSON and human-readable output formats
- Progress indicators for long operations
- Comprehensive configuration system with multiple sources
- Environment variable support with PAGEFIND_ prefix
- Configuration file support (TOML, YAML, JSON)
- Custom ranking weight configuration

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
- Configuration system with CLI, environment, and file support
- Custom ranking weight configuration

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