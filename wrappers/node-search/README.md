# @pagefind/search

Native search functionality for Pagefind indexes in Node.js.

This package provides a Node.js API for searching Pagefind indexes directly from your server or build process, without requiring a browser environment.

## Installation

```bash
npm install @pagefind/search
```

## Usage

### Basic Search

```javascript
import { createSearch } from '@pagefind/search';

// Initialize a search instance
const { search } = await createSearch({
    bundlePath: './public/pagefind'
});

// Perform a search
const results = await search.search('documentation');

// Process results
for (const result of results.results) {
    console.log(`Found: ${result.page} (score: ${result.score})`);
    
    // Load full fragment data
    const data = await result.data();
    console.log(`URL: ${data.url}`);
    console.log(`Title: ${data.meta.title}`);
    console.log(`Excerpt: ${data.excerpt()}`);
}
```

### Advanced Search with Filters

```javascript
const results = await search.search('api', {
    filters: {
        category: ['technical', 'reference'],
        language: ['en']
    },
    sort: {
        by: 'date',
        direction: 'desc'
    },
    limit: 10
});
```

### Working with Filters

```javascript
// Get all available filters
const { filters } = await search.getFilters();

console.log('Available filters:');
for (const [filterName, values] of Object.entries(filters)) {
    console.log(`${filterName}:`);
    for (const [value, count] of Object.entries(values)) {
        console.log(`  - ${value} (${count} pages)`);
    }
}
```

### Preloading for Performance

```javascript
// Preload chunks for common search terms
await search.preload('documentation');
await search.preload('api');

// Subsequent searches for these terms will be faster
const results = await search.search('documentation api');
```

### Loading Fragments Directly

```javascript
// If you have a page hash from previous results
const { fragment } = await search.loadFragment('abc123def456');

console.log(`URL: ${fragment.url}`);
console.log(`Content: ${fragment.excerpt(300)}`);
```

## API Reference

### `createSearch(config)`

Creates a new search instance.

**Parameters:**
- `config.bundlePath` (string, required): Path to the Pagefind bundle directory
- `config.language` (string, optional): Force a specific language (ISO 639-1 code)
- `config.rankingWeights` (object, optional): Custom ranking weights
- `config.verbose` (boolean, optional): Enable verbose logging

**Returns:** Promise<SearchResponse>
- `errors`: Array of error messages
- `search`: PagefindSearch instance

### `search.search(query, options)`

Performs a search.

**Parameters:**
- `query` (string): The search query
- `options.filters` (object, optional): Filters to apply
- `options.sort` (object, optional): Sort configuration
- `options.limit` (number, optional): Maximum results to return

**Returns:** Promise<SearchResults>

### `search.getFilters()`

Gets all available filters in the index.

**Returns:** Promise<FiltersResponse>

### `search.preload(query)`

Preloads chunks for a query to improve subsequent search performance.

**Parameters:**
- `query` (string): The query to preload

**Returns:** Promise<PreloadResponse>

### `search.loadFragment(pageHash)`

Loads a fragment by its hash.

**Parameters:**
- `pageHash` (string): The page hash

**Returns:** Promise<FragmentResponse>

### `search.destroy()`

Cleans up resources used by the search instance.

**Returns:** Promise<void>

## Fragment API

Each fragment returned by `result.data()` or `loadFragment()` includes:

- `url`: The page URL
- `content`: Full text content
- `wordCount`: Total word count
- `filters`: Page filters
- `meta`: Page metadata
- `anchors`: Array of headings/anchors
- `excerpt(length)`: Function to generate an excerpt
- `subResults`: Sub-results for matching headings (when available)

## Error Handling

All methods return objects with an `errors` array. Always check for errors:

```javascript
const results = await search.search('query');
if (results.errors.length > 0) {
    console.error('Search failed:', results.errors);
    return;
}
```

## Performance Considerations

1. **Reuse search instances**: Create one search instance and reuse it for multiple searches
2. **Use preload**: Preload common search terms during initialization
3. **Cache fragments**: The library caches loaded fragments automatically
4. **Limit results**: Use the `limit` option for large result sets

## Requirements

- Node.js 14 or higher
- A Pagefind bundle created with the main Pagefind tool

## License

MIT