/**
 * Parse a raw fragment into a PageFragment object with additional methods
 * @param {Object} rawFragment - The raw fragment data from the file
 * @param {string} [searchQuery] - Optional search query for excerpt generation
 * @returns {import('../types/index').PageFragment}
 */
export function parseFragment(rawFragment, searchQuery) {
    const fragment = {
        url: rawFragment.url,
        content: rawFragment.content,
        wordCount: rawFragment.word_count,
        filters: rawFragment.filters || {},
        meta: rawFragment.meta || {},
        anchors: rawFragment.anchors || [],
        
        /**
         * Generate an excerpt from the content
         * @param {number} [length=200] - Maximum length of the excerpt
         * @returns {string}
         */
        excerpt: function(length = 200) {
            if (!this.content) return '';
            
            // If we have a search query, try to find relevant sections
            if (searchQuery) {
                const excerpt = generateSearchExcerpt(this.content, searchQuery, length);
                if (excerpt) return excerpt;
            }
            
            // Otherwise, return the beginning of the content
            return generateDefaultExcerpt(this.content, length);
        }
    };
    
    // Add sub-results if we have a search query
    if (searchQuery && rawFragment.anchors && rawFragment.anchors.length > 0) {
        fragment.subResults = generateSubResults(rawFragment, searchQuery);
    }
    
    return fragment;
}

/**
 * Generate an excerpt that includes the search terms
 * @param {string} content - The full content
 * @param {string} query - The search query
 * @param {number} maxLength - Maximum length of the excerpt
 * @returns {string|null}
 */
function generateSearchExcerpt(content, query, maxLength) {
    // Extract search terms from the query
    const terms = extractSearchTerms(query);
    if (terms.length === 0) return null;
    
    // Find the first occurrence of any search term
    let bestMatch = null;
    let bestIndex = content.length;
    
    for (const term of terms) {
        const index = content.toLowerCase().indexOf(term.toLowerCase());
        if (index !== -1 && index < bestIndex) {
            bestIndex = index;
            bestMatch = term;
        }
    }
    
    if (bestMatch === null) return null;
    
    // Calculate excerpt boundaries
    const halfLength = Math.floor(maxLength / 2);
    let start = Math.max(0, bestIndex - halfLength);
    let end = Math.min(content.length, bestIndex + bestMatch.length + halfLength);
    
    // Adjust to word boundaries
    if (start > 0) {
        const spaceIndex = content.lastIndexOf(' ', start);
        if (spaceIndex > start - 20) {
            start = spaceIndex + 1;
        }
    }
    
    if (end < content.length) {
        const spaceIndex = content.indexOf(' ', end);
        if (spaceIndex !== -1 && spaceIndex < end + 20) {
            end = spaceIndex;
        }
    }
    
    // Build the excerpt
    let excerpt = content.substring(start, end);
    
    // Add ellipsis if needed
    if (start > 0) excerpt = '...' + excerpt;
    if (end < content.length) excerpt = excerpt + '...';
    
    return excerpt;
}

/**
 * Generate a default excerpt from the beginning of the content
 * @param {string} content - The full content
 * @param {number} maxLength - Maximum length of the excerpt
 * @returns {string}
 */
function generateDefaultExcerpt(content, maxLength) {
    if (content.length <= maxLength) {
        return content;
    }
    
    // Find a good breaking point
    let end = maxLength;
    const spaceIndex = content.lastIndexOf(' ', end);
    if (spaceIndex > maxLength - 20) {
        end = spaceIndex;
    }
    
    return content.substring(0, end) + '...';
}

/**
 * Extract search terms from a query
 * @param {string} query - The search query
 * @returns {string[]}
 */
function extractSearchTerms(query) {
    // Handle exact phrase search
    if (query.startsWith('"') && query.endsWith('"')) {
        return [query.slice(1, -1)];
    }
    
    // Split by spaces but respect quoted phrases
    const terms = [];
    const regex = /"[^"]+"|[^\s]+/g;
    let match;
    
    while ((match = regex.exec(query)) !== null) {
        let term = match[0];
        // Remove quotes if present
        if (term.startsWith('"') && term.endsWith('"')) {
            term = term.slice(1, -1);
        }
        if (term.length > 0) {
            terms.push(term);
        }
    }
    
    return terms;
}

/**
 * Generate sub-results for anchors that match the search
 * @param {Object} fragment - The raw fragment data
 * @param {string} query - The search query
 * @returns {import('../types/index').SubResult[]}
 */
function generateSubResults(fragment, query) {
    const terms = extractSearchTerms(query);
    if (terms.length === 0) return [];
    
    const subResults = [];
    const content = fragment.content.toLowerCase();
    
    for (const anchor of fragment.anchors) {
        if (!anchor.text) continue;
        
        const anchorText = anchor.text.toLowerCase();
        let matches = false;
        
        // Check if any search term appears in the anchor text
        for (const term of terms) {
            if (anchorText.includes(term.toLowerCase())) {
                matches = true;
                break;
            }
        }
        
        if (matches) {
            // Find content around this anchor
            const startPos = anchor.location;
            const endPos = fragment.anchors.find(a => a.location > startPos)?.location || content.length;
            const sectionContent = fragment.content.substring(startPos, endPos);
            
            subResults.push({
                title: anchor.text,
                url: fragment.url + (anchor.id ? `#${anchor.id}` : ''),
                excerpt: generateDefaultExcerpt(sectionContent, 150),
                locations: [anchor.location]
            });
        }
    }
    
    return subResults;
}