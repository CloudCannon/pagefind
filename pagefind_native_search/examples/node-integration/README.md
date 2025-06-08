# Node.js Integration Example

This example demonstrates how to integrate Pagefind Native Search into a Node.js application, including both a web server API and a CLI tool.

## Setup

1. Install dependencies:
   ```bash
   npm install
   ```

2. Ensure you have a Pagefind bundle available. You can either:
   - Use an existing bundle from your static site
   - Create a sample bundle (see instructions below)
   - Set the `PAGEFIND_BUNDLE` environment variable to point to your bundle

## Running the Examples

### Web Server API

Start the Express server:

```bash
npm start
# or
node server.js
```

The server will start on http://localhost:3000 and provide:
- A simple web UI for searching
- REST API endpoints for integration

#### API Endpoints

**Search Endpoint**
```bash
GET /api/search?q=<query>&limit=<number>&filters=<json>&sort=<json>

# Examples:
curl "http://localhost:3000/api/search?q=documentation"
curl "http://localhost:3000/api/search?q=api&limit=5"
curl "http://localhost:3000/api/search?q=guide&filters={\"category\":[\"tutorials\"]}"
```

**Filters Endpoint**
```bash
GET /api/filters

# Example:
curl "http://localhost:3000/api/filters"
```

**Suggestions Endpoint**
```bash
GET /api/suggestions?q=<partial-query>

# Example:
curl "http://localhost:3000/api/suggestions?q=java"
```

### CLI Tool

Run searches from the command line:

```bash
# Basic search
node search-cli.js "your search query"

# With options
node search-cli.js "documentation" --limit 5
node search-cli.js "tutorial" --filters '{"category":["guides"]}'
node search-cli.js "latest" --sort '{"by":"date","direction":"desc"}'

# JSON output for scripting
node search-cli.js "api" --json | jq '.results[].url'
```

## Creating a Sample Bundle

If you don't have a Pagefind bundle, you can create one:

1. Create a sample site:
   ```bash
   mkdir -p sample-site
   echo '<html><body><h1>Test Page</h1><p>This is test content.</p></body></html>' > sample-site/index.html
   ```

2. Run Pagefind to create the bundle:
   ```bash
   npx pagefind --source sample-site --bundle-dir sample-bundle
   ```

## Advanced Usage

### Custom Search Implementation

```javascript
const { PagefindNativeSearch } = require('@pagefind/node-search');

class SearchService {
  constructor(bundlePath) {
    this.search = new PagefindNativeSearch({ bundlePath });
    this.cache = new Map();
  }

  async searchWithCache(query, options) {
    const cacheKey = JSON.stringify({ query, options });
    
    if (this.cache.has(cacheKey)) {
      return this.cache.get(cacheKey);
    }
    
    const results = await this.search.search(query, options);
    this.cache.set(cacheKey, results);
    
    // Clear cache after 5 minutes
    setTimeout(() => this.cache.delete(cacheKey), 5 * 60 * 1000);
    
    return results;
  }
}
```

### Integration with Frontend Framework

Example React component:

```jsx
function SearchComponent() {
  const [query, setQuery] = useState('');
  const [results, setResults] = useState([]);
  const [loading, setLoading] = useState(false);

  const search = async () => {
    setLoading(true);
    try {
      const response = await fetch(`/api/search?q=${encodeURIComponent(query)}`);
      const data = await response.json();
      setResults(data.results);
    } catch (error) {
      console.error('Search failed:', error);
    }
    setLoading(false);
  };

  return (
    <div>
      <input 
        value={query} 
        onChange={(e) => setQuery(e.target.value)}
        onKeyPress={(e) => e.key === 'Enter' && search()}
      />
      <button onClick={search}>Search</button>
      
      {loading && <div>Searching...</div>}
      
      {results.map((result, i) => (
        <div key={i}>
          <h3>{result.title}</h3>
          <p>{result.excerpt}</p>
          <a href={result.url}>Read more</a>
        </div>
      ))}
    </div>
  );
}
```

### Environment Variables

Configure the application using environment variables:

```bash
# Set bundle path
export PAGEFIND_BUNDLE=/path/to/your/pagefind/bundle

# Set server port
export PORT=3001

# Run the server
npm start
```

### Docker Deployment

Create a `Dockerfile`:

```dockerfile
FROM node:18-alpine

WORKDIR /app

# Copy package files
COPY package*.json ./
RUN npm ci --only=production

# Copy application files
COPY . .

# Copy Pagefind bundle
COPY ./pagefind-bundle ./sample-bundle

EXPOSE 3000

CMD ["node", "server.js"]
```

Build and run:

```bash
docker build -t pagefind-search-api .
docker run -p 3000:3000 -e PAGEFIND_BUNDLE=/app/sample-bundle pagefind-search-api
```

## Performance Optimization

### 1. Connection Pooling

For high-traffic applications, implement connection pooling:

```javascript
const { Worker } = require('worker_threads');

class SearchPool {
  constructor(bundlePath, poolSize = 4) {
    this.workers = [];
    this.queue = [];
    
    for (let i = 0; i < poolSize; i++) {
      this.workers.push(this.createWorker(bundlePath));
    }
  }
  
  async search(query, options) {
    const worker = await this.getWorker();
    try {
      return await worker.search(query, options);
    } finally {
      this.releaseWorker(worker);
    }
  }
  
  // Implementation details...
}
```

### 2. Response Caching

Implement Redis caching for better performance:

```javascript
const redis = require('redis');
const client = redis.createClient();

app.get('/api/search', async (req, res) => {
  const cacheKey = `search:${JSON.stringify(req.query)}`;
  
  // Check cache
  const cached = await client.get(cacheKey);
  if (cached) {
    return res.json(JSON.parse(cached));
  }
  
  // Perform search
  const results = await search.search(req.query.q, options);
  
  // Cache results for 5 minutes
  await client.setex(cacheKey, 300, JSON.stringify(results));
  
  res.json(results);
});
```

### 3. Rate Limiting

Protect your API with rate limiting:

```javascript
const rateLimit = require('express-rate-limit');

const searchLimiter = rateLimit({
  windowMs: 1 * 60 * 1000, // 1 minute
  max: 100, // limit each IP to 100 requests per minute
  message: 'Too many search requests, please try again later.'
});

app.use('/api/search', searchLimiter);
```

## Troubleshooting

### Common Issues

1. **Binary not found**: Ensure the pagefind_native_search binary is available in your PATH or in the expected location.

2. **Bundle not found**: Verify the bundle path is correct and the directory contains the Pagefind index files.

3. **Memory issues**: For large indexes, increase Node.js memory limit:
   ```bash
   node --max-old-space-size=4096 server.js
   ```

4. **Slow searches**: Enable verbose logging to identify bottlenecks:
   ```bash
   export PAGEFIND_VERBOSE=true
   ```

### Debug Mode

Enable debug logging:

```javascript
const search = new PagefindNativeSearch({
  bundlePath: './sample-bundle',
  config: {
    verbose: true
  }
});
```

## Next Steps

- Integrate with your existing Node.js application
- Add authentication and authorization
- Implement advanced filtering UI
- Set up monitoring and analytics
- Deploy to production with proper scaling