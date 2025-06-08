const express = require('express');
const cors = require('cors');
const { PagefindNativeSearch } = require('@pagefind/node-search');
const path = require('path');

const app = express();
const PORT = process.env.PORT || 3000;

// Enable CORS for frontend integration
app.use(cors());
app.use(express.json());

// Initialize Pagefind search
// Update this path to point to your actual Pagefind bundle
const BUNDLE_PATH = process.env.PAGEFIND_BUNDLE || path.join(__dirname, 'sample-bundle');

const search = new PagefindNativeSearch({
  bundlePath: BUNDLE_PATH,
  config: {
    language: 'en'
  }
});

// Search endpoint
app.get('/api/search', async (req, res) => {
  try {
    const { 
      q = '', 
      filters, 
      sort, 
      limit = 20,
      page = 1 
    } = req.query;

    // Parse filters if provided as JSON string
    let filterOptions = {};
    if (filters) {
      try {
        filterOptions = JSON.parse(filters);
      } catch (e) {
        return res.status(400).json({ 
          error: 'Invalid filters format. Must be valid JSON.' 
        });
      }
    }

    // Parse sort if provided
    let sortOptions = null;
    if (sort) {
      try {
        sortOptions = JSON.parse(sort);
      } catch (e) {
        return res.status(400).json({ 
          error: 'Invalid sort format. Must be valid JSON.' 
        });
      }
    }

    // Perform search
    const results = await search.search(q, {
      filters: Object.keys(filterOptions).length > 0 ? filterOptions : undefined,
      sort: sortOptions,
      limit: parseInt(limit)
    });

    // Add pagination info
    const totalPages = Math.ceil((results.totalResults || results.results.length) / limit);
    
    res.json({
      ...results,
      pagination: {
        page: parseInt(page),
        limit: parseInt(limit),
        totalPages,
        totalResults: results.totalResults || results.results.length
      }
    });

  } catch (error) {
    console.error('Search error:', error);
    res.status(500).json({ 
      error: 'Search failed', 
      message: error.message 
    });
  }
});

// Get available filters
app.get('/api/filters', async (req, res) => {
  try {
    const filters = await search.getFilters();
    res.json(filters);
  } catch (error) {
    console.error('Filter error:', error);
    res.status(500).json({ 
      error: 'Failed to get filters', 
      message: error.message 
    });
  }
});

// Search suggestions endpoint (searches and returns just titles)
app.get('/api/suggestions', async (req, res) => {
  try {
    const { q = '' } = req.query;
    
    if (!q || q.length < 2) {
      return res.json({ suggestions: [] });
    }

    const results = await search.search(q, { limit: 5 });
    
    const suggestions = results.results.map(r => ({
      title: r.title,
      url: r.url
    }));

    res.json({ suggestions });
  } catch (error) {
    console.error('Suggestions error:', error);
    res.json({ suggestions: [] });
  }
});

// Health check
app.get('/api/health', (req, res) => {
  res.json({ 
    status: 'ok', 
    bundlePath: BUNDLE_PATH,
    timestamp: new Date().toISOString()
  });
});

// Serve a simple search UI
app.get('/', (req, res) => {
  res.send(`
    <!DOCTYPE html>
    <html>
    <head>
      <title>Pagefind Search API</title>
      <style>
        body {
          font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
          max-width: 800px;
          margin: 0 auto;
          padding: 2rem;
          background: #f5f5f5;
        }
        .container {
          background: white;
          padding: 2rem;
          border-radius: 8px;
          box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }
        h1 { color: #333; }
        .search-box {
          display: flex;
          gap: 1rem;
          margin-bottom: 2rem;
        }
        input[type="text"] {
          flex: 1;
          padding: 0.75rem;
          font-size: 16px;
          border: 2px solid #ddd;
          border-radius: 4px;
        }
        button {
          padding: 0.75rem 1.5rem;
          background: #007bff;
          color: white;
          border: none;
          border-radius: 4px;
          cursor: pointer;
          font-size: 16px;
        }
        button:hover {
          background: #0056b3;
        }
        .results {
          margin-top: 2rem;
        }
        .result {
          margin-bottom: 1.5rem;
          padding-bottom: 1.5rem;
          border-bottom: 1px solid #eee;
        }
        .result:last-child {
          border-bottom: none;
        }
        .result h3 {
          margin: 0 0 0.5rem 0;
          color: #0066cc;
        }
        .result .url {
          color: #666;
          font-size: 14px;
          margin-bottom: 0.5rem;
        }
        .result .excerpt {
          color: #333;
          line-height: 1.5;
        }
        .filters {
          margin-bottom: 1rem;
          padding: 1rem;
          background: #f8f9fa;
          border-radius: 4px;
        }
        .loading {
          text-align: center;
          color: #666;
          padding: 2rem;
        }
        .error {
          color: #dc3545;
          padding: 1rem;
          background: #f8d7da;
          border-radius: 4px;
          margin-top: 1rem;
        }
      </style>
    </head>
    <body>
      <div class="container">
        <h1>Pagefind Search Demo</h1>
        
        <div class="search-box">
          <input 
            type="text" 
            id="searchInput" 
            placeholder="Enter your search query..."
            autofocus
          >
          <button onclick="performSearch()">Search</button>
        </div>
        
        <div id="filters" class="filters" style="display: none;"></div>
        <div id="results" class="results"></div>
      </div>

      <script>
        const searchInput = document.getElementById('searchInput');
        const resultsDiv = document.getElementById('results');
        const filtersDiv = document.getElementById('filters');

        // Search on Enter key
        searchInput.addEventListener('keypress', (e) => {
          if (e.key === 'Enter') {
            performSearch();
          }
        });

        // Load filters on page load
        loadFilters();

        async function loadFilters() {
          try {
            const response = await fetch('/api/filters');
            const filters = await response.json();
            
            if (Object.keys(filters).length > 0) {
              filtersDiv.style.display = 'block';
              filtersDiv.innerHTML = '<strong>Available filters:</strong><br>';
              
              for (const [category, values] of Object.entries(filters)) {
                const valueList = Object.entries(values)
                  .map(([name, count]) => \`\${name} (\${count})\`)
                  .join(', ');
                filtersDiv.innerHTML += \`<div>\${category}: \${valueList}</div>\`;
              }
            }
          } catch (error) {
            console.error('Failed to load filters:', error);
          }
        }

        async function performSearch() {
          const query = searchInput.value.trim();
          
          resultsDiv.innerHTML = '<div class="loading">Searching...</div>';
          
          try {
            const response = await fetch(\`/api/search?q=\${encodeURIComponent(query)}\`);
            const data = await response.json();
            
            if (!response.ok) {
              throw new Error(data.message || 'Search failed');
            }
            
            displayResults(data);
          } catch (error) {
            resultsDiv.innerHTML = \`<div class="error">Error: \${error.message}</div>\`;
          }
        }

        function displayResults(data) {
          if (data.results.length === 0) {
            resultsDiv.innerHTML = '<p>No results found.</p>';
            return;
          }
          
          const html = data.results.map(result => \`
            <div class="result">
              <h3>\${result.title || 'Untitled'}</h3>
              <div class="url">\${result.url}</div>
              \${result.excerpt ? \`<div class="excerpt">\${result.excerpt}</div>\` : ''}
            </div>
          \`).join('');
          
          resultsDiv.innerHTML = \`
            <p>Found \${data.results.length} results\${data.totalResults ? \` out of \${data.totalResults}\` : ''}:</p>
            \${html}
          \`;
        }
      </script>
    </body>
    </html>
  `);
});

// Start server
app.listen(PORT, () => {
  console.log(`Pagefind search server running on http://localhost:${PORT}`);
  console.log(`API endpoints:`);
  console.log(`  - GET /api/search?q=query`);
  console.log(`  - GET /api/filters`);
  console.log(`  - GET /api/suggestions?q=query`);
  console.log(`  - GET /api/health`);
  console.log(`\nBundle path: ${BUNDLE_PATH}`);
});