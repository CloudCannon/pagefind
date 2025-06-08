// Sort functionality for search results
// This module handles sorting of search results based on various criteria

use crate::CoreSearchIndex;

impl CoreSearchIndex {
    /// Apply sorting to search results based on the specified sort key and direction
    pub fn apply_sort(&self, results: &mut Vec<crate::search::PageSearchResult>, sort: &str) {
        if let Some((sort_key, direction)) = sort.split_once(':') {
            if let Some(sorted_pages) = self.sorts.get(sort_key) {
                // Filter results to only include pages that have sort values
                results.retain(|result| sorted_pages.contains(&(result.page_index as u32)));

                // Apply sort scores based on position in sorted array
                for result in results.iter_mut() {
                    result.page_score = sorted_pages
                        .iter()
                        .position(|p| p == &(result.page_index as u32))
                        .expect("Sorted pages should contain all remaining results")
                        as f32;
                    
                    // Reverse scores for ascending order
                    if direction == "asc" {
                        result.page_score = 0.0 - result.page_score;
                    }
                }
            }
        }
    }
}