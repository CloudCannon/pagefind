import { SearchService } from "./service.js";
import { parseFragment } from "./fragment.js";

/**
 * @typedef {import('../types/index').SearchConfig} SearchConfig
 * @typedef {import('../types/index').SearchResponse} SearchResponse
 * @typedef {import('../types/index').PagefindSearch} PagefindSearch
 * @typedef {import('../types/index').SearchOptions} SearchOptions
 * @typedef {import('../types/index').SearchResults} SearchResults
 * @typedef {import('../types/index').SearchResult} SearchResult
 * @typedef {import('../types/index').PreloadResponse} PreloadResponse
 * @typedef {import('../types/index').FiltersResponse} FiltersResponse
 * @typedef {import('../types/index').FragmentResponse} FragmentResponse
 * @typedef {import('../types/index').PageFragment} PageFragment
 */

/**
 * Create a new Pagefind search instance
 * @param {SearchConfig} config
 * @returns {Promise<SearchResponse>}
 */
export const createSearch = async (config) => {
    try {
        // Validate config
        if (!config.bundlePath) {
            return {
                errors: ["bundlePath is required"],
            };
        }

        // Create service instance
        const service = new SearchService(config);
        
        // Initialize the search
        const initResult = await service.init();
        if (initResult.error) {
            return {
                errors: [initResult.error],
            };
        }

        // Create the search API
        const searchApi = createSearchApi(service, config);
        
        return {
            errors: [],
            search: searchApi,
        };
    } catch (error) {
        return {
            errors: [error.message || "Failed to create search instance"],
        };
    }
};

/**
 * Create the search API object
 * @param {SearchService} service
 * @param {SearchConfig} config
 * @returns {PagefindSearch}
 */
const createSearchApi = (service, config) => {
    /**
     * @type {Map<string, PageFragment>}
     */
    const fragmentCache = new Map();

    return {
        /**
         * Perform a search
         * @param {string} query
         * @param {SearchOptions} [options]
         * @returns {Promise<SearchResults>}
         */
        search: async (query, options = {}) => {
            try {
                const result = await service.search(query, options);
                
                if (result.error) {
                    return {
                        errors: [result.error],
                        results: [],
                        unfilteredResultCount: 0,
                        filters: {},
                        totalFilters: {},
                    };
                }

                // Transform results to include data loading function
                const results = result.results.map(r => ({
                    page: r.page,
                    score: r.score,
                    wordCount: r.word_count,
                    matchedWordCount: r.word_locations,
                    data: async () => {
                        // Check cache first
                        if (fragmentCache.has(r.page)) {
                            return fragmentCache.get(r.page);
                        }

                        // Load fragment
                        const fragmentResult = await service.loadFragment(r.page);
                        if (fragmentResult.error) {
                            throw new Error(fragmentResult.error);
                        }

                        const fragment = parseFragment(fragmentResult.fragment, query);
                        fragmentCache.set(r.page, fragment);
                        return fragment;
                    }
                }));

                return {
                    errors: [],
                    results,
                    unfilteredResultCount: result.unfiltered_count,
                    filters: result.filters || {},
                    totalFilters: result.total_filters || {},
                };
            } catch (error) {
                return {
                    errors: [error.message || "Search failed"],
                    results: [],
                    unfilteredResultCount: 0,
                    filters: {},
                    totalFilters: {},
                };
            }
        },

        /**
         * Preload chunks for a query
         * @param {string} query
         * @returns {Promise<PreloadResponse>}
         */
        preload: async (query) => {
            try {
                const result = await service.preload(query);
                
                if (result.error) {
                    return {
                        errors: [result.error],
                    };
                }

                return {
                    errors: [],
                };
            } catch (error) {
                return {
                    errors: [error.message || "Preload failed"],
                };
            }
        },

        /**
         * Get all available filters
         * @returns {Promise<FiltersResponse>}
         */
        getFilters: async () => {
            try {
                const result = await service.getFilters();
                
                if (result.error) {
                    return {
                        errors: [result.error],
                        filters: {},
                    };
                }

                return {
                    errors: [],
                    filters: result.filters || {},
                };
            } catch (error) {
                return {
                    errors: [error.message || "Failed to get filters"],
                    filters: {},
                };
            }
        },

        /**
         * Load a fragment
         * @param {string} pageHash
         * @returns {Promise<FragmentResponse>}
         */
        loadFragment: async (pageHash) => {
            try {
                // Check cache first
                if (fragmentCache.has(pageHash)) {
                    return {
                        errors: [],
                        fragment: fragmentCache.get(pageHash),
                    };
                }

                const result = await service.loadFragment(pageHash);
                
                if (result.error) {
                    return {
                        errors: [result.error],
                    };
                }

                const fragment = parseFragment(result.fragment);
                fragmentCache.set(pageHash, fragment);

                return {
                    errors: [],
                    fragment,
                };
            } catch (error) {
                return {
                    errors: [error.message || "Failed to load fragment"],
                };
            }
        },

        /**
         * Destroy the search instance
         * @returns {Promise<void>}
         */
        destroy: async () => {
            fragmentCache.clear();
            await service.destroy();
        },
    };
};