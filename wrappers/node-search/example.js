import { createSearch } from './lib/index.js';

async function main() {
    console.log('Pagefind Search Example\n');
    
    // Initialize search with a Pagefind bundle
    console.log('Initializing search...');
    const { errors, search } = await createSearch({
        bundlePath: './test-bundle', // Replace with your actual bundle path
        language: 'en',
        verbose: true
    });
    
    if (errors.length > 0) {
        console.error('Failed to initialize search:', errors);
        process.exit(1);
    }
    
    console.log('Search initialized successfully!\n');
    
    // Example 1: Basic search
    console.log('Example 1: Basic Search');
    console.log('------------------------');
    const basicResults = await search.search('documentation');
    
    console.log(`Found ${basicResults.results.length} results`);
    console.log(`Total unfiltered: ${basicResults.unfilteredResultCount}\n`);
    
    // Show first 3 results
    for (let i = 0; i < Math.min(3, basicResults.results.length); i++) {
        const result = basicResults.results[i];
        console.log(`Result ${i + 1}:`);
        console.log(`  Page: ${result.page}`);
        console.log(`  Score: ${result.score.toFixed(2)}`);
        console.log(`  Word count: ${result.wordCount}`);
        
        // Load fragment data
        const data = await result.data();
        console.log(`  URL: ${data.url}`);
        console.log(`  Title: ${data.meta.title || 'No title'}`);
        console.log(`  Excerpt: ${data.excerpt(150)}`);
        console.log();
    }
    
    // Example 2: Search with filters
    console.log('\nExample 2: Filtered Search');
    console.log('---------------------------');
    const filteredResults = await search.search('api', {
        filters: {
            category: ['technical'],
            language: ['en']
        },
        limit: 5
    });
    
    console.log(`Found ${filteredResults.results.length} filtered results`);
    console.log('Active filters:', filteredResults.filters);
    
    // Example 3: Get all available filters
    console.log('\nExample 3: Available Filters');
    console.log('-----------------------------');
    const { filters } = await search.getFilters();
    
    for (const [filterName, values] of Object.entries(filters)) {
        console.log(`\n${filterName}:`);
        const sortedValues = Object.entries(values)
            .sort((a, b) => b[1] - a[1])
            .slice(0, 5);
        
        for (const [value, count] of sortedValues) {
            console.log(`  - ${value}: ${count} pages`);
        }
        
        if (Object.keys(values).length > 5) {
            console.log(`  ... and ${Object.keys(values).length - 5} more`);
        }
    }
    
    // Example 4: Exact phrase search
    console.log('\n\nExample 4: Exact Phrase Search');
    console.log('--------------------------------');
    const exactResults = await search.search('"getting started"');
    console.log(`Found ${exactResults.results.length} exact matches`);
    
    // Example 5: Sorted search
    console.log('\nExample 5: Sorted Search');
    console.log('-------------------------');
    const sortedResults = await search.search('tutorial', {
        sort: {
            by: 'date',
            direction: 'desc'
        },
        limit: 10
    });
    
    console.log(`Found ${sortedResults.results.length} results (sorted by date, newest first)`);
    
    // Example 6: Preloading for performance
    console.log('\n\nExample 6: Preloading');
    console.log('----------------------');
    console.log('Preloading common search terms...');
    
    const preloadTerms = ['documentation', 'api', 'tutorial', 'guide'];
    for (const term of preloadTerms) {
        await search.preload(term);
        console.log(`  ✓ Preloaded: ${term}`);
    }
    
    console.log('\nSubsequent searches for these terms will be faster!');
    
    // Clean up
    await search.destroy();
    console.log('\nSearch instance destroyed.');
}

// Run the example
main().catch(console.error);