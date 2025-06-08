/**
 * Initialize a Pagefind search instance with a bundle directory
 */
export function createSearch(config: SearchConfig): Promise<SearchResponse>;

/**
 * Configuration for creating a search instance
 */
export interface SearchConfig {
    /**
     * Path to the Pagefind bundle directory
     * @example "./public/pagefind"
     */
    bundlePath: string;
    
    /**
     * Force a specific language. If not specified, will use the language with the most pages.
     * Expects an ISO 639-1 code.
     * @example "en"
     */
    language?: string;
    
    /**
     * Custom ranking weights for search results
     */
    rankingWeights?: RankingWeights;
    
    /**
     * Verbose logging
     */
    verbose?: boolean;
}

/**
 * Response from creating a search instance
 */
export interface SearchResponse {
    errors: string[];
    search?: PagefindSearch;
}

/**
 * A Pagefind search instance
 */
export interface PagefindSearch {
    /**
     * Perform a search query
     */
    search: (query: string, options?: SearchOptions) => Promise<SearchResults>;
    
    /**
     * Preload chunks for a query to improve search performance
     */
    preload: (query: string) => Promise<PreloadResponse>;
    
    /**
     * Get all available filters in the index
     */
    getFilters: () => Promise<FiltersResponse>;
    
    /**
     * Load a fragment for a search result
     */
    loadFragment: (pageHash: string) => Promise<FragmentResponse>;
    
    /**
     * Destroy the search instance and clean up resources
     */
    destroy: () => Promise<void>;
}

/**
 * Options for performing a search
 */
export interface SearchOptions {
    /**
     * Filters to apply to the search
     * @example { "category": ["tech", "news"], "author": ["alice"] }
     */
    filters?: Record<string, string[]>;
    
    /**
     * Sort results by a specific key
     */
    sort?: {
        /**
         * The sort key to use
         */
        by: string;
        
        /**
         * Sort direction
         */
        direction: "asc" | "desc";
    };
    
    /**
     * Maximum number of results to return
     */
    limit?: number;
}

/**
 * Search results
 */
export interface SearchResults {
    errors: string[];
    results: SearchResult[];
    unfilteredResultCount: number;
    filters: Record<string, Record<string, number>>;
    totalFilters: Record<string, Record<string, number>>;
}

/**
 * Individual search result
 */
export interface SearchResult {
    /**
     * Page hash identifier
     */
    page: string;
    
    /**
     * Relevance score
     */
    score: number;
    
    /**
     * Word count of the page
     */
    wordCount: number;
    
    /**
     * Number of matched word locations
     */
    matchedWordCount: number;
    
    /**
     * Load the full fragment for this result
     */
    data: () => Promise<PageFragment>;
}

/**
 * Response from preloading chunks
 */
export interface PreloadResponse {
    errors: string[];
}

/**
 * Response from getting filters
 */
export interface FiltersResponse {
    errors: string[];
    filters: Record<string, Record<string, number>>;
}

/**
 * Response from loading a fragment
 */
export interface FragmentResponse {
    errors: string[];
    fragment?: PageFragment;
}

/**
 * Page fragment data
 */
export interface PageFragment {
    /**
     * URL of the page
     */
    url: string;
    
    /**
     * Full content of the page
     */
    content: string;
    
    /**
     * Word count
     */
    wordCount: number;
    
    /**
     * Page filters
     */
    filters: Record<string, string[]>;
    
    /**
     * Page metadata
     */
    meta: Record<string, string>;
    
    /**
     * Anchors/headings in the page
     */
    anchors: PageAnchor[];
    
    /**
     * Generate an excerpt with optional highlighting
     */
    excerpt: (length?: number) => string;
    
    /**
     * Sub-results for this page (headings that match the search)
     */
    subResults?: SubResult[];
}

/**
 * Page anchor/heading
 */
export interface PageAnchor {
    element: string;
    id?: string;
    text?: string;
    location: number;
}

/**
 * Sub-result within a page
 */
export interface SubResult {
    title: string;
    url: string;
    excerpt: string;
    locations: number[];
}

/**
 * Custom ranking weights
 */
export interface RankingWeights {
    pageLength?: number;
    pageLocation?: number;
    pageBoost?: number;
    termSaturation?: number;
    termFrequency?: number;
    termSimilarity?: number;
}