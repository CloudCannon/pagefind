# Configuration Examples

This directory contains example configuration files for Pagefind Native Search in different formats:

- `pagefind.toml` - TOML format (recommended)
- `pagefind.yaml` - YAML format
- `pagefind.json` - JSON format

## Usage

Place one of these files in your project root or specify the path using the `--config` flag:

```bash
# Use config file in current directory
pagefind_native_search search "query"

# Specify config file path
pagefind_native_search search "query" --config ./config/pagefind.toml
```

## Configuration Precedence

Configuration is loaded in the following order (highest to lowest priority):

1. **CLI arguments** - Command line flags override all other settings
2. **Environment variables** - Variables prefixed with `PAGEFIND_`
3. **Configuration file** - Settings from the config file
4. **Default values** - Built-in defaults

## Environment Variables

Any configuration option can be set via environment variables by prefixing with `PAGEFIND_` and converting to uppercase:

```bash
export PAGEFIND_BUNDLE="./my-bundle"
export PAGEFIND_DEFAULT_LIMIT="50"
export PAGEFIND_VERBOSE="true"
export PAGEFIND_RANKING_TERM_SIMILARITY="2.0"
```

## Common Configurations

### Development Configuration

```toml
# pagefind.dev.toml
bundle = "./pagefind"
verbose = true
output_format = "json"
default_limit = 10
```

### Production Configuration

```toml
# pagefind.prod.toml
bundle = "/var/www/pagefind"
quiet = true
logfile = "/var/log/pagefind-search.log"
preload_chunks = true
cache_size_mb = 100
default_limit = 50
```

### Performance-Optimized Configuration

```toml
# pagefind.performance.toml
bundle = "./pagefind"
preload_chunks = true          # Load all chunks at startup
cache_size_mb = 200            # Large cache for better performance
concurrent_fragments = 10       # More concurrent loads
generate_excerpts = false       # Skip excerpt generation for speed
load_fragments = false          # Don't load full fragments
```

### Custom Ranking Configuration

```toml
# pagefind.ranking.toml
bundle = "./pagefind"

# Boost exact term matches
ranking_term_similarity = 2.0

# Reduce influence of page length
ranking_page_length = 0.5

# Standard term frequency weight
ranking_term_frequency = 1.0

# Increase term saturation weight
ranking_term_saturation = 1.5
```

## Configuration Options Reference

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `bundle` | string | `./pagefind` | Path to Pagefind bundle directory |
| `language` | string | auto-detect | Force a specific language |
| `default_limit` | number | 30 | Default number of results |
| `preload_chunks` | boolean | false | Load all chunks at startup |
| `cache_size_mb` | number | 50 | Cache size in megabytes |
| `output_format` | string | `text` | Output format (json/text) |
| `verbose` | boolean | false | Enable verbose logging |
| `quiet` | boolean | false | Only show errors |
| `logfile` | string | none | Path to log file |
| `generate_excerpts` | boolean | true | Generate search excerpts |
| `excerpt_length` | number | 300 | Maximum excerpt length |
| `excerpt_context` | number | 15 | Context words around matches |
| `load_fragments` | boolean | true | Load full page fragments |
| `concurrent_fragments` | number | 5 | Max concurrent fragment loads |
| `ranking_term_similarity` | float | 1.0 | Term similarity weight |
| `ranking_page_length` | float | 1.0 | Page length weight |
| `ranking_term_saturation` | float | 1.0 | Term saturation weight |
| `ranking_term_frequency` | float | 1.0 | Term frequency weight |

## Tips

1. **Use TOML for readability** - TOML format is recommended for configuration files due to its clarity and support for comments.

2. **Environment-specific configs** - Create separate config files for different environments (dev, staging, prod).

3. **Version control** - Include example configs in version control but exclude environment-specific values.

4. **Validation** - The tool will validate configuration on startup and report any errors.

5. **Performance tuning** - Start with defaults and adjust based on your specific needs and index size.