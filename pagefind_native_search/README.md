# Pagefind Native Search

Native file system-based search implementation for Pagefind indexes. This crate provides the ability to search Pagefind indexes directly from Rust or through a CLI, without requiring a browser or WebAssembly environment.

## Overview

Pagefind Native Search allows you to:
- Search pre-built Pagefind indexes from the file system
- Use Pagefind search functionality in server-side applications
- Build CLI tools for searching static sites
- Integrate Pagefind search into native applications

## Features

- **Full compatibility** with Pagefind web indexes
- **Incremental chunk loading** for optimal performance
- **Filter support** with faceted search capabilities
- **Sorting** by custom fields
- **Excerpt generation** with highlighted matches
- **Multi-language support** with automatic language detection
- **Configuration** via files, environment variables, or CLI arguments

## Installation

### As a Rust Library

Add to your `Cargo.toml`:

```toml
[dependencies]
pagefind_native_search = "0.1.0"
```

### As a CLI Tool

```bash
cargo install pagefind_native_search
```

Or build from source:

```bash
git clone https://github.com/CloudCannon/pagefind.git
cd pagefind/pagefind_native_search
cargo build --release
```

## Usage

### CLI Usage

Basic search:
```bash
pagefind_native_search search "your query" --bundle ./pagefind
```

Search with filters:
```bash
pagefind_native_search search "documentation" \
  --bundle ./pagefind \
  --filters '{"category": ["docs", "guides"]}'
```

Search with sorting:
```bash
pagefind_native_search search "latest" \
  --bundle ./pagefind \
  --sort '{"by": "date", "direction": "desc"}'
```

List available filters:
```bash
pagefind_native_search filters --bundle ./pagefind
```

### Library Usage

```rust
use pagefind_native_search::{NativeSearch, SearchOptions};
use std::path::Path;

// Initialize the search index
let mut search = NativeSearch::new(Path::new("./pagefind"))?;
search.init(Some("en"))?;

// Perform a basic search
let results = search.search("rust programming", SearchOptions::default())?;

for result in results.results {
    println!("{}: {}", result.url, result.title);
}

// Search with filters
let mut options = SearchOptions::default();
options.filters.insert(
    "category".to_string(),
    vec!["documentation".to_string()]
);

let filtered_results = search.search("api", options)?;

// Get available filters
let filters = search.get_filters()?;
for (filter_name, values) in filters {
    println!("Filter: {}", filter_name);
    for (value, count) in values {
        println!("  {}: {}", value, count);
    }
}
```

## Configuration

Pagefind Native Search supports configuration through multiple sources with the following precedence:

1. **CLI arguments** (highest priority)
2. **Environment variables** (PAGEFIND_* prefix)
3. **Configuration files** (pagefind.toml, pagefind.yaml, pagefind.json)
4. **Default values** (lowest priority)

### Configuration File Example

`pagefind.toml`:
```toml
bundle = "./pagefind"
language = "en"
default_limit = 50
excerpt_length = 300
verbose = true

# Ranking weights
ranking_term_similarity = 1.0
ranking_page_length = 0.5
ranking_term_frequency = 2.0
```

### Environment Variables

```bash
export PAGEFIND_BUNDLE="./pagefind"
export PAGEFIND_LANGUAGE="en"
export PAGEFIND_DEFAULT_LIMIT="50"
export PAGEFIND_VERBOSE="true"
```

### Configuration Options

| Option | Description | Default |
|--------|-------------|---------|
| `bundle` | Path to Pagefind bundle directory | `./pagefind` |
| `language` | Force a specific language | Auto-detect |
| `default_limit` | Default number of results | 30 |
| `excerpt_length` | Maximum excerpt length | 300 |
| `excerpt_context` | Context words around matches | 15 |
| `preload_chunks` | Preload all chunks for performance | false |
| `cache_size_mb` | Cache size for loaded chunks | 50 |
| `output_format` | Output format (json/text) | text |
| `verbose` | Enable verbose logging | false |
| `quiet` | Only show errors | false |

## Performance Considerations

### Chunk Loading

Pagefind Native Search uses lazy loading of index chunks to minimize memory usage:

1. **Metadata** is loaded on initialization
2. **Index chunks** are loaded on-demand based on search terms
3. **Filter chunks** are loaded when filters are applied
4. **Fragments** are loaded when full content is requested

### Optimization Tips

1. **Enable chunk preloading** for frequently accessed indexes:
   ```toml
   preload_chunks = true
   ```

2. **Increase cache size** for better performance with large indexes:
   ```toml
   cache_size_mb = 100
   ```

3. **Use specific language** to avoid language detection overhead:
   ```toml
   language = "en"
   ```

4. **Limit concurrent fragment loads** to control memory usage:
   ```toml
   concurrent_fragments = 3
   ```

## Differences from Web-based Search

### Key Differences

1. **No WebAssembly** - Runs natively without browser requirements
2. **File system access** - Reads index files directly from disk
3. **Synchronous API** - No async/await required (though async support is planned)
4. **Memory management** - Direct control over chunk loading and caching
5. **CLI interface** - Can be used as a command-line tool

### Feature Parity

Native search maintains full compatibility with web-based search:
- Same ranking algorithms
- Identical filter behavior
- Compatible excerpt generation
- Matching search syntax (including exact phrase search)

### Limitations

- No real-time index updates (indexes must be pre-built)
- Larger binary size compared to WASM version
- Platform-specific binaries required

## Migration Guide

### From Web to Native Search

If you're currently using Pagefind in a browser and want to use native search:

1. **Ensure indexes are accessible** - Native search needs file system access to the pagefind bundle
2. **Update search initialization**:
   ```javascript
   // Web version
   const pagefind = await import("/pagefind/pagefind.js");
   await pagefind.init();
   
   // Native version (Node.js)
   const { PagefindNativeSearch } = require('@pagefind/node-search');
   const search = new PagefindNativeSearch({ bundlePath: './pagefind' });
   ```

3. **Adapt search calls**:
   ```javascript
   // Web version
   const results = await pagefind.search("query");
   
   // Native version
   const results = await search.search("query");
   ```

### API Compatibility

The native search API is designed to be similar to the web API:

| Web API | Native API | Notes |
|---------|------------|-------|
| `pagefind.init()` | `NativeSearch::new()` + `init()` | Two-step initialization |
| `pagefind.search()` | `search.search()` | Same parameters |
| `pagefind.filters()` | `search.get_filters()` | Returns all available filters |
| `pagefind.preload()` | `search.preload()` | Preload chunks for a query |

## Troubleshooting

### Common Issues

1. **"Bundle path does not exist"**
   - Ensure the pagefind directory exists and contains index files
   - Check the bundle path is correct relative to your working directory

2. **"Failed to decode metadata"**
   - Verify the pagefind bundle was built with a compatible version
   - Ensure files aren't corrupted

3. **"No language indexes found"**
   - Check pagefind-entry.json exists in the bundle directory
   - Verify the requested language has an index

4. **Performance issues**
   - Enable chunk preloading for frequently searched indexes
   - Increase cache size for large indexes
   - Consider using SSDs for better I/O performance

### Debug Mode

Enable verbose logging to troubleshoot issues:

```bash
pagefind_native_search search "query" --verbose
```

Or set the environment variable:
```bash
export PAGEFIND_VERBOSE=true
```

### Logging

Configure logging output:

```toml
# Log to file
logfile = "./pagefind-search.log"

# Control verbosity
verbose = true  # Detailed logs
quiet = false   # Only errors
```

## Examples

### Basic CLI Search Script

```bash
#!/bin/bash
# search.sh - Simple search wrapper

BUNDLE_PATH="${PAGEFIND_BUNDLE:-./pagefind}"
QUERY="$1"

if [ -z "$QUERY" ]; then
    echo "Usage: $0 <search query>"
    exit 1
fi

pagefind_native_search search "$QUERY" \
    --bundle "$BUNDLE_PATH" \
    --output-format json | jq '.results[] | {url, title}'
```

### Rust Integration Example

```rust
use pagefind_native_search::{NativeSearch, SearchOptions, NativeSearchConfig};
use std::path::PathBuf;

fn search_site(query: &str) -> anyhow::Result<()> {
    // Configure search
    let config = NativeSearchConfig::new("./pagefind")
        .with_language("en")
        .with_ranking_weights(RankingWeights {
            term_similarity: 1.0,
            page_length: 0.5,
            term_frequency: 2.0,
            term_saturation: 1.5,
        });

    // Initialize search
    let mut search = NativeSearch::with_config(config)?;
    search.init(None)?;

    // Search with options
    let mut options = SearchOptions::default();
    options.limit = Some(10);
    
    let results = search.search(query, options)?;
    
    // Process results
    for (i, result) in results.results.iter().enumerate() {
        println!("{}. {} (score: {:.2})", 
            i + 1, 
            result.title, 
            result.score
        );
        println!("   URL: {}", result.url);
        if let Some(excerpt) = &result.excerpt {
            println!("   {}", excerpt);
        }
        println!();
    }
    
    Ok(())
}
```

### Node.js Wrapper Example

```javascript
const { PagefindNativeSearch } = require('@pagefind/node-search');

async function searchWithFilters() {
    const search = new PagefindNativeSearch({
        bundlePath: './pagefind',
        config: { language: 'en' }
    });

    // Get available filters
    const filters = await search.getFilters();
    console.log('Available filters:', filters);

    // Search with filters
    const results = await search.search('documentation', {
        filters: {
            category: ['guides', 'reference'],
            author: ['John Doe']
        },
        sort: { by: 'date', direction: 'desc' },
        limit: 20
    });

    console.log(`Found ${results.results.length} results`);
    results.results.forEach(result => {
        console.log(`- ${result.title}: ${result.url}`);
    });
}

searchWithFilters().catch(console.error);
```

## Contributing

Contributions are welcome! Please see the main Pagefind repository for contribution guidelines.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/CloudCannon/pagefind.git
cd pagefind/pagefind_native_search

# Run tests
cargo test

# Run with verbose output
RUST_LOG=debug cargo run -- search "test" --bundle ../test_bundle
```

### Testing

Run the test suite:
```bash
# Unit tests
cargo test --lib

# Integration tests
cargo test --test '*'

# Run with coverage
cargo tarpaulin --out Html
```

## License

MIT License - see the main Pagefind repository for details.