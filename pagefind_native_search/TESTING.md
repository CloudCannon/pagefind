# Pagefind Native Search Testing Guide

This document provides an overview of the testing infrastructure for Pagefind Native Search.

## Test Structure

### 1. Rust Integration Tests

Location: `pagefind_native_search/tests/integration_test.rs`

These tests cover:
- Native search initialization
- Entry file loading and parsing
- File decompression (gzip with pagefind_dcd magic bytes)
- Language selection and fallback
- Search options configuration
- Fragment loading
- Chunk listing
- Configuration loading from files and environment variables

Run with:
```bash
cargo test
```

### 2. Unit Tests

Unit tests are included in the source modules:

- `src/file_loader/mod.rs` - Tests for file loading and decompression
- `src/config.rs` - Tests for configuration loading and merging

### 3. Node.js Wrapper Tests

Location: `wrappers/node-search/test/`

- `basic.test.js` - Basic instantiation tests
- `integration.test.js` - Comprehensive integration tests including:
  - Binary detection
  - Search functionality
  - Filter operations
  - Sorting
  - Error handling
  - TypeScript type definitions

Run with:
```bash
cd wrappers/node-search
npm test
```

## Test Coverage Areas

### Core Functionality
- ✅ Index loading and initialization
- ✅ Search operations
- ✅ Filter application
- ✅ Sorting
- ✅ Excerpt generation
- ✅ Fragment loading
- ✅ Multi-language support

### Configuration
- ✅ File-based configuration (TOML, YAML, JSON)
- ✅ Environment variable configuration
- ✅ CLI argument parsing
- ✅ Configuration precedence
- ✅ Default values

### Error Handling
- ✅ Missing bundle directory
- ✅ Invalid index files
- ✅ Corrupted data
- ✅ Binary not found (Node.js)
- ✅ Process failures

### Performance
- ✅ Chunk lazy loading
- ✅ Cache management
- ✅ Concurrent operations

## Running All Tests

### Prerequisites

1. Build the native search binary:
   ```bash
   cd pagefind_native_search
   cargo build --release
   ```

2. Install Node.js dependencies:
   ```bash
   cd wrappers/node-search
   npm install
   ```

### Run All Tests

```bash
# Run Rust tests
cd pagefind_native_search
cargo test

# Run Node.js tests
cd ../wrappers/node-search
npm test
```

### Test with Coverage

```bash
# Rust coverage (requires cargo-tarpaulin)
cargo install cargo-tarpaulin
cargo tarpaulin --out Html

# Node.js coverage
npm run test:coverage
```

## Creating Test Bundles

For integration testing, you'll need Pagefind bundles. Create test bundles:

```bash
# Create a simple test site
mkdir test-site
echo '<html><body><h1>Test</h1></body></html>' > test-site/index.html

# Generate Pagefind bundle
npx pagefind --source test-site --bundle-dir test-bundle
```

## Continuous Integration

Add to your CI pipeline:

```yaml
# Example GitHub Actions workflow
name: Test

on: [push, pull_request]

jobs:
  test-rust:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo test
        working-directory: pagefind_native_search

  test-node:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
        with:
          node-version: '18'
      - run: npm install
        working-directory: wrappers/node-search
      - run: npm test
        working-directory: wrappers/node-search
```

## Manual Testing

### CLI Testing

Test the CLI with various scenarios:

```bash
# Basic search
pagefind_native_search search "test" --bundle ./test-bundle

# With filters
pagefind_native_search search "test" \
  --bundle ./test-bundle \
  --filters '{"category": ["docs"]}'

# List filters
pagefind_native_search filters --bundle ./test-bundle

# Test configuration loading
PAGEFIND_VERBOSE=true pagefind_native_search search "test"
```

### Performance Testing

```bash
# Time a search operation
time pagefind_native_search search "common term" --bundle ./large-bundle

# Profile with perf (Linux)
perf record pagefind_native_search search "test" --bundle ./bundle
perf report
```

## Debugging Tests

### Rust Tests

```bash
# Run specific test
cargo test test_native_search_initialization

# Run with output
cargo test -- --nocapture

# Run with backtrace
RUST_BACKTRACE=1 cargo test
```

### Node.js Tests

```bash
# Run specific test file
node test/integration.test.js

# Debug with inspector
node --inspect test/integration.test.js
```

## Adding New Tests

### Rust Tests

Add to `tests/integration_test.rs`:

```rust
#[test]
fn test_new_feature() {
    let temp_dir = TempDir::new().unwrap();
    // Test implementation
}
```

### Node.js Tests

Add to `test/integration.test.js`:

```javascript
test('new feature test', async () => {
  const search = new PagefindNativeSearch();
  // Test implementation
});
```

## Known Issues

1. **Binary path detection** - The Node.js wrapper may need adjustments for different platforms
2. **Test bundle creation** - Tests currently use mock data; real CBOR encoding would be more accurate
3. **Performance benchmarks** - No automated performance regression tests yet

## Future Improvements

1. Add property-based testing for search operations
2. Implement fuzzing for file parsing
3. Add performance benchmarks
4. Create integration tests with real Pagefind bundles
5. Add cross-platform CI testing