# Pagefind Node Search

Node.js wrapper for Pagefind Native Search, providing server-side search capabilities for Pagefind indexes.

## Installation

```bash
npm install @pagefind/node-search
```

## Quick Start

```javascript
const { PagefindNativeSearch } = require('@pagefind/node-search');

// Initialize search with a bundle path
const search = new PagefindNativeSearch({
  bundlePath: './pagefind'
});

// Perform a search
const results = await search.search('your search query');
console.log(results);
```

## API Reference

### Constructor

```javascript
new PagefindNativeSearch(options)
```

Options:
- `bundlePath` (string): Path to the Pagefind bundle directory. Default: `'./pagefind'`
- `config` (object): Additional configuration options
  - `language` (string): Force a specific language

### Methods

#### search(query, options)

Performs a search on the index.

```javascript
const results = await search.search('search query', {
  filters: {
    category: ['blog', 'docs'],
    author: ['John Doe']
  },
  sort: {
    by: 'date',
    direction: 'desc'
  },
  limit: 20
});
```

Parameters:
- `query` (string): The search query
- `options` (object, optional):
  - `filters` (object): Key-value pairs of filters to apply
  - `sort` (object): Sorting configuration
    - `by` (string): Field to sort by
    - `direction` ('asc' | 'desc'): Sort direction
  - `limit` (number): Maximum number of results

Returns: Promise<SearchResults>

```typescript
interface SearchResults {
  results: Array<{
    url: string;
    title: string;
    excerpt?: string;
    score?: number;
    meta?: Record<string, string>;
    filters?: Record<string, string[]>;
  }>;
  totalResults?: number;
  filters?: Record<string, Record<string, number>>;
}
```

#### getFilters()

Retrieves all available filters from the index.

```javascript
const filters = await search.getFilters();
console.log(filters);
// Output:
// {
//   category: { blog: 10, docs: 25, tutorials: 8 },
//   author: { 'John Doe': 15, 'Jane Smith': 18 }
// }
```

Returns: Promise<Record<string, Record<string, number>>>

## Examples

### Basic Search

```javascript
const { PagefindNativeSearch } = require('@pagefind/node-search');

async function basicSearch() {
  const search = new PagefindNativeSearch({
    bundlePath: './public/pagefind'
  });

  try {
    const results = await search.search('javascript tutorial');
    
    console.log(`Found ${results.results.length} results:`);
    results.results.forEach((result, i) => {
      console.log(`${i + 1}. ${result.title}`);
      console.log(`   URL: ${result.url}`);
      if (result.excerpt) {
        console.log(`   ${result.excerpt}`);
      }
    });
  } catch (error) {
    console.error('Search failed:', error);
  }
}

basicSearch();
```

### Filtered Search

```javascript
async function filteredSearch() {
  const search = new PagefindNativeSearch({
    bundlePath: './pagefind'
  });

  // First, get available filters
  const availableFilters = await search.getFilters();
  console.log('Available filters:', availableFilters);

  // Search with filters
  const results = await search.search('api', {
    filters: {
      category: ['documentation', 'reference'],
      language: ['javascript', 'typescript']
    }
  });

  console.log(`Found ${results.results.length} filtered results`);
}
```

### Sorted Results

```javascript
async function sortedSearch() {
  const search = new PagefindNativeSearch({
    bundlePath: './pagefind'
  });

  // Get latest posts
  const results = await search.search('', {
    sort: {
      by: 'date',
      direction: 'desc'
    },
    limit: 10
  });

  console.log('Latest 10 posts:');
  results.results.forEach(result => {
    console.log(`- ${result.title} (${result.meta?.date})`);
  });
}
```

### Express.js Integration

```javascript
const express = require('express');
const { PagefindNativeSearch } = require('@pagefind/node-search');

const app = express();
const search = new PagefindNativeSearch({
  bundlePath: './public/pagefind'
});

app.get('/api/search', async (req, res) => {
  try {
    const { q, filters, sort, limit } = req.query;
    
    const options = {};
    if (filters) options.filters = JSON.parse(filters);
    if (sort) options.sort = JSON.parse(sort);
    if (limit) options.limit = parseInt(limit);
    
    const results = await search.search(q || '', options);
    res.json(results);
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.get('/api/filters', async (req, res) => {
  try {
    const filters = await search.getFilters();
    res.json(filters);
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.listen(3000, () => {
  console.log('Search API running on http://localhost:3000');
});
```

## TypeScript Support

This package includes TypeScript definitions. Use with TypeScript:

```typescript
import { PagefindNativeSearch, SearchOptions, SearchResults } from '@pagefind/node-search';

const search = new PagefindNativeSearch({
  bundlePath: './pagefind'
});

const options: SearchOptions = {
  filters: {
    category: ['blog']
  },
  limit: 10
};

const results: SearchResults = await search.search('query', options);
```

## Error Handling

The wrapper will throw errors in the following cases:

- Binary not found: `Could not find pagefind_native_search binary`
- Invalid bundle path: `Bundle path does not exist`
- Search process errors: `Search process exited with code X`
- Invalid JSON responses: `Failed to parse search results`

Always wrap calls in try-catch blocks:

```javascript
try {
  const results = await search.search('query');
} catch (error) {
  console.error('Search error:', error.message);
  // Handle error appropriately
}
```

## Performance Tips

1. **Reuse instances**: Create one `PagefindNativeSearch` instance and reuse it
2. **Bundle location**: Place the bundle on fast storage (SSD) for better performance
3. **Limit results**: Use the `limit` option to reduce processing time
4. **Filter early**: Apply filters to reduce the search space

## Binary Management

The package expects the `pagefind_native_search` binary to be available. It looks in:

1. `./bin/` directory within the package
2. `../../target/release/` (for development)
3. System PATH

The binary must match your platform (Windows, macOS, Linux) and architecture.

## Troubleshooting

### Binary not found

If you get a "Could not find pagefind_native_search binary" error:

1. Ensure the package was installed correctly
2. Check that the binary exists in the expected location
3. Verify the binary has execute permissions
4. Try installing the native binary separately

### Search errors

Enable verbose logging by setting environment variables:

```bash
export PAGEFIND_VERBOSE=true
node your-app.js
```

### Performance issues

For large indexes:
- Consider implementing result pagination
- Use filters to narrow search scope
- Monitor memory usage of the native process

## License

MIT - See LICENSE file for details