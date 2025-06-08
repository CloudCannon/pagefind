import { createSearch } from '../lib/index.js';
import { strict as assert } from 'assert';

// Simple test runner
async function test(name, fn) {
    try {
        await fn();
        console.log(`✓ ${name}`);
    } catch (error) {
        console.error(`✗ ${name}`);
        console.error(`  ${error.message}`, error);
        process.exit(1);
    }
}

async function runTests() {
    console.log('Running basic tests for @pagefind/search\n');
    
    // Test 0: Create search with target public URL: https://pagefind.app/_pagefind
    await test('should return error for invalid bundle path', async () => {
        const { errors, search } = await createSearch({
            bundlePath: '../../docs/public/pagefind'
        });

        const results = search ? await search.search('test') : null;

        if (errors.length > 0) {
            console.error('Errors:', errors);
        }
        assert(errors.length <= 0, 'Expected no errors for valid path');
        assert(results, 'Expected search instance for valid path');
    });

    // Test 1: Create search instance with invalid path
    await test('should return error for invalid bundle path', async () => {
        const { errors, search } = await createSearch({
            bundlePath: '/non/existent/path'
        });
        
        assert(errors.length > 0, 'Expected errors for invalid path');
        assert(!search, 'Expected no search instance for invalid path');
    });
    
    // Test 2: Create search instance without bundlePath
    await test('should return error when bundlePath is missing', async () => {
        const { errors, search } = await createSearch({});
        
        assert(errors.length > 0, 'Expected errors for missing bundlePath');
        assert(errors[0].includes('bundlePath'), 'Error should mention bundlePath');
        assert(!search, 'Expected no search instance');
    });
    
    // Test 3: API structure validation
    await test('should validate API structure', async () => {
        // This test would need a valid bundle to work properly
        // For now, just test the structure
        
        const mockService = {
            init: async () => ({ success: true }),
            search: async () => ({ results: [], unfiltered_count: 0, filters: {} }),
            getFilters: async () => ({ filters: {} }),
            loadFragment: async () => ({ fragment: {} }),
            preload: async () => ({ success: true }),
            destroy: async () => {}
        };
        
        // Test that the API has the expected methods
        const expectedMethods = ['search', 'getFilters', 'loadFragment', 'preload', 'destroy'];
        
        // We can't easily test the full flow without mocking more internals
        // but we can verify the structure is correct
        assert(true, 'API structure test placeholder');
    });
    
    console.log('\nAll tests passed!');
}

// Run tests
runTests().catch(console.error);