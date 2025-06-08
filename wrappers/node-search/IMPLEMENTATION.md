# @pagefind/search Implementation Details

This document describes the implementation of the Node.js wrapper for Pagefind's native search functionality.

## Architecture

The package uses a subprocess-based approach to communicate with the `pagefind-search` CLI binary. This design was chosen for:

1. **Simplicity**: No need for complex native bindings or N-API
2. **Portability**: Works across all platforms without compilation
3. **Isolation**: The search process runs independently
4. **Compatibility**: Follows similar patterns to the existing Pagefind wrapper

## File Structure

```
wrappers/node-search/
├── lib/
│   ├── index.js          # Main entry point and API
│   ├── service.js        # Subprocess management
│   ├── fragment.js       # Fragment parsing and excerpt generation
│   └── resolveBinary.js  # Binary path resolution
├── types/
│   └── index.d.ts        # TypeScript definitions
├── test/
│   └── basic.test.js     # Basic unit tests
├── example.js            # Usage examples
├── package.json          # Package configuration
├── tsconfig.json         # TypeScript configuration
├── README.md             # User documentation
└── IMPLEMENTATION.md     # This file
```

## Key Components

### 1. Main API (`lib/index.js`)

- `createSearch()`: Factory function that creates a search instance
- Implements caching for fragments to improve performance
- Provides a clean Promise-based API
- Handles error propagation gracefully

### 2. Service Layer (`lib/service.js`)

- Manages subprocess communication with the CLI binary
- Executes commands and parses JSON responses
- Handles binary path resolution
- Direct file access for fragment loading (optimization)

### 3. Fragment Parser (`lib/fragment.js`)

- Parses raw fragment data into rich objects
- Generates excerpts with search term highlighting
- Creates sub-results for matching headings
- Handles both exact phrase and term-based searches

### 4. Binary Resolution (`lib/resolveBinary.js`)

- Locates the pagefind-search binary
- Supports development and production environments
- Falls back to PATH if binary not found locally

## API Design Decisions

1. **Async/Promise-based**: All operations return Promises for consistency
2. **Error handling**: Errors are returned in arrays, not thrown
3. **Lazy loading**: Fragments are loaded on-demand via `result.data()`
4. **Caching**: Fragments are cached to avoid redundant loads
5. **TypeScript support**: Full type definitions for better DX

## Integration with Native Search

The package communicates with the `pagefind-search` CLI tool which provides:

- Search operations with the native Rust implementation
- Filter support with bitset operations
- Sorting capabilities
- Direct file system access to Pagefind bundles

## Performance Optimizations

1. **Fragment caching**: Loaded fragments are cached in memory
2. **Direct file access**: Fragments are read directly instead of through CLI
3. **Preloading**: Chunks can be preloaded for common queries
4. **Subprocess reuse**: The CLI process could be kept alive for multiple operations (future enhancement)

## Future Enhancements

1. **Persistent subprocess**: Keep the CLI process running for better performance
2. **Streaming results**: Support for streaming large result sets
3. **Binary bundling**: Include platform-specific binaries in npm package
4. **WebSocket mode**: Alternative communication method for better performance
5. **Worker thread support**: Run searches in worker threads

## Testing

Basic tests are included to verify:
- Error handling for invalid configurations
- API structure validation
- Basic functionality

Integration tests would require a valid Pagefind bundle.

## Publishing

The package is ready to be published to npm under the `@pagefind` scope:

```bash
npm publish --access public
```

Platform-specific binary packages would need to be created separately.