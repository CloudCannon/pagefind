#!/usr/bin/env node

const { PagefindNativeSearch } = require('@pagefind/node-search');
const path = require('path');

// Parse command line arguments
const args = process.argv.slice(2);
if (args.length === 0 || args.includes('--help') || args.includes('-h')) {
  console.log(`
Pagefind Search CLI

Usage:
  node search-cli.js <query> [options]

Options:
  --bundle <path>     Path to Pagefind bundle (default: ./sample-bundle)
  --limit <number>    Maximum results to return (default: 10)
  --filters <json>    JSON string of filters to apply
  --sort <json>       JSON string of sort configuration
  --json              Output results as JSON
  --help, -h          Show this help message

Examples:
  node search-cli.js "search query"
  node search-cli.js "api docs" --limit 5
  node search-cli.js "tutorial" --filters '{"category":["guides"]}'
  node search-cli.js "latest" --sort '{"by":"date","direction":"desc"}'
  node search-cli.js "query" --json | jq '.results[].url'
`);
  process.exit(0);
}

// Extract query and options
const query = args[0];
const options = {
  bundle: './sample-bundle',
  limit: 10,
  filters: null,
  sort: null,
  json: false
};

// Parse command line options
for (let i = 1; i < args.length; i++) {
  switch (args[i]) {
    case '--bundle':
      options.bundle = args[++i];
      break;
    case '--limit':
      options.limit = parseInt(args[++i]);
      break;
    case '--filters':
      try {
        options.filters = JSON.parse(args[++i]);
      } catch (e) {
        console.error('Error: Invalid JSON for filters');
        process.exit(1);
      }
      break;
    case '--sort':
      try {
        options.sort = JSON.parse(args[++i]);
      } catch (e) {
        console.error('Error: Invalid JSON for sort');
        process.exit(1);
      }
      break;
    case '--json':
      options.json = true;
      break;
  }
}

// Perform search
async function search() {
  try {
    // Initialize search
    const search = new PagefindNativeSearch({
      bundlePath: options.bundle
    });

    // Build search options
    const searchOptions = {};
    if (options.filters) searchOptions.filters = options.filters;
    if (options.sort) searchOptions.sort = options.sort;
    if (options.limit) searchOptions.limit = options.limit;

    // Perform search
    console.error(`Searching for: "${query}"`);
    const results = await search.search(query, searchOptions);

    // Output results
    if (options.json) {
      console.log(JSON.stringify(results, null, 2));
    } else {
      displayResults(results);
    }

  } catch (error) {
    console.error('Search error:', error.message);
    process.exit(1);
  }
}

function displayResults(results) {
  if (results.results.length === 0) {
    console.log('No results found.');
    return;
  }

  console.log(`\nFound ${results.results.length} results:\n`);

  results.results.forEach((result, index) => {
    console.log(`${index + 1}. ${result.title || 'Untitled'}`);
    console.log(`   URL: ${result.url}`);
    
    if (result.excerpt) {
      console.log(`   ${result.excerpt}`);
    }
    
    if (result.meta && Object.keys(result.meta).length > 0) {
      console.log(`   Meta:`, result.meta);
    }
    
    console.log();
  });

  // Show filter information if available
  if (results.filters && Object.keys(results.filters).length > 0) {
    console.log('Available filters for these results:');
    for (const [filterName, values] of Object.entries(results.filters)) {
      const valueList = Object.entries(values)
        .map(([name, count]) => `${name} (${count})`)
        .join(', ');
      console.log(`  ${filterName}: ${valueList}`);
    }
  }
}

// Run search
search();