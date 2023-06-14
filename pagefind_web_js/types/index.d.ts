export {};

declare global {
    type PagefindIndexOptions = {
        basePath?: string,
        baseUrl?: string,
        primary?: boolean,
        indexWeight?: number,
        mergeFilter?: Object,
        language?: string,
    };

    type PagefindSearchOptions = {
        preload?: boolean,
        verbose?: boolean,
        filters?: Object,
        sort?: Object,
    }

    type PagefindSearchResults = {
        results: PagefindSearchResult[],
        unfilteredResultCount: number,
        filters: Object,
        totalFilters: Object,
        timings: {
            preload: number,
            search: number,
            total: number
        }
    }

    type PagefindSearchResult = {
        id: number,
        score: number,
        words: number[],
        excerpt_range: number[],
        data: () => Promise<any>
    }

    type PagefindSearchFragment = {
        url: string,
        content: string,
        excerpt: string,
        word_count: number,
        locations: number[],
        filters: Record<string, string[]>
        meta: Record<string, string>,
        anchors: PagefindSearchAnchor[],
    }

    type PagefindSearchAnchor = {
        element: string,
        id: string,
        text?: string,
        location: number,
    }
}
