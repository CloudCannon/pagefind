# Basic CLI Usage Example

This example demonstrates basic usage of the Pagefind Native Search CLI.

## Prerequisites

1. Build the pagefind_native_search binary:
   ```bash
   cd ../..
   cargo build --release
   ```

2. Create a sample Pagefind bundle or use an existing one.

## Usage Examples

### Basic Search

```bash
# Search for a simple term
pagefind_native_search search "documentation" --bundle ./sample-bundle

# Search for a phrase
pagefind_native_search search "getting started guide" --bundle ./sample-bundle

# Exact phrase search
pagefind_native_search search '"exact phrase match"' --bundle ./sample-bundle
```

### Search with Filters

```bash
# Filter by category
pagefind_native_search search "api" \
  --bundle ./sample-bundle \
  --filters '{"category": ["documentation"]}'

# Multiple filters
pagefind_native_search search "tutorial" \
  --bundle ./sample-bundle \
  --filters '{"category": ["guides"], "language": ["javascript"]}'
```

### Search with Sorting

```bash
# Sort by date (newest first)
pagefind_native_search search "latest" \
  --bundle ./sample-bundle \
  --sort '{"by": "date", "direction": "desc"}'

# Sort by title alphabetically
pagefind_native_search search "" \
  --bundle ./sample-bundle \
  --sort '{"by": "title", "direction": "asc"}' \
  --limit 20
```

### List Available Filters

```bash
# Show all available filters and their counts
pagefind_native_search filters --bundle ./sample-bundle
```

### Output Formats

```bash
# JSON output (for scripting)
pagefind_native_search search "query" \
  --bundle ./sample-bundle \
  --output-format json

# Pretty JSON with jq
pagefind_native_search search "query" \
  --bundle ./sample-bundle \
  --output-format json | jq '.'

# Extract just URLs
pagefind_native_search search "query" \
  --bundle ./sample-bundle \
  --output-format json | jq -r '.results[].url'
```

## Shell Script Example

Create a `search.sh` script:

```bash
#!/bin/bash

# Simple search wrapper with defaults
BUNDLE="${PAGEFIND_BUNDLE:-./sample-bundle}"
LIMIT="${PAGEFIND_LIMIT:-10}"

search() {
    local query="$1"
    shift
    
    pagefind_native_search search "$query" \
        --bundle "$BUNDLE" \
        --default-limit "$LIMIT" \
        --output-format json \
        "$@" | jq -r '.results[] | "\(.title)\n  \(.url)\n  \(.excerpt // "No excerpt")\n"'
}

# Usage
search "your query"
search "filtered query" --filters '{"category": ["blog"]}'
```

## Configuration File Example

Create a `pagefind.toml`:

```toml
bundle = "./sample-bundle"
default_limit = 20
output_format = "json"
excerpt_length = 200
verbose = false

# Custom ranking weights
ranking_term_similarity = 1.5
ranking_page_length = 0.5
```

Then run without specifying bundle:

```bash
pagefind_native_search search "query"
```

## Environment Variables

```bash
# Set default bundle path
export PAGEFIND_BUNDLE="./my-site/pagefind"

# Enable verbose logging
export PAGEFIND_VERBOSE=true

# Set default language
export PAGEFIND_LANGUAGE=en

# Run search
pagefind_native_search search "query"
```

## Advanced Usage

### Batch Processing

Process multiple queries from a file:

```bash
#!/bin/bash

# queries.txt contains one query per line
while IFS= read -r query; do
    echo "Searching for: $query"
    pagefind_native_search search "$query" \
        --bundle ./sample-bundle \
        --output-format json \
        --default-limit 5 | jq -r '.results[].url'
    echo "---"
done < queries.txt
```

### Integration with fzf

Interactive search with fuzzy finder:

```bash
#!/bin/bash

# Interactive search with preview
search_interactive() {
    pagefind_native_search search "" \
        --bundle ./sample-bundle \
        --output-format json \
        --default-limit 100 | \
    jq -r '.results[] | "\(.title)|\(.url)"' | \
    fzf --delimiter '|' \
        --preview 'echo "URL: {2}"' \
        --preview-window up:3:wrap | \
    cut -d'|' -f2
}

# Open selected result
url=$(search_interactive)
if [ -n "$url" ]; then
    open "$url"  # or xdg-open on Linux
fi
```

## Performance Testing

```bash
#!/bin/bash

# Measure search performance
time_search() {
    local query="$1"
    local iterations="${2:-10}"
    
    echo "Timing search for: '$query' ($iterations iterations)"
    
    time for i in $(seq 1 $iterations); do
        pagefind_native_search search "$query" \
            --bundle ./sample-bundle \
            --output-format json > /dev/null
    done
}

time_search "common term"
time_search "specific phrase"
time_search "" # All results