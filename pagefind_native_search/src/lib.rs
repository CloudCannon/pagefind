//! Native search implementation for Pagefind
//!
//! This crate provides file system-based search capabilities for Pagefind indexes,
//! allowing searches to be performed directly from Rust without requiring a browser
//! or WebAssembly environment.

pub mod file_loader;
pub mod config;
pub mod cli;

use anyhow::{Context, Result};
use pagefind_core_search::{CoreSearchIndex, PageSearchResult, RankingWeights};
use std::path::{Path, PathBuf};
use std::collections::{HashMap, HashSet};
use bit_set::BitSet;

/// Main native search context
pub struct NativeSearch {
    bundle_path: PathBuf,
    search_index: CoreSearchIndex,
    entry_data: Option<EntryData>,
    current_language: String,
    loaded_chunks: HashSet<String>,
    loaded_filter_chunks: HashSet<String>,
}

impl NativeSearch {
    /// Create a new native search instance from a bundle path
    pub fn new<P: AsRef<Path>>(bundle_path: P) -> Result<Self> {
        let bundle_path = bundle_path.as_ref().to_path_buf();
        
        // Verify the bundle path exists
        if !bundle_path.exists() {
            anyhow::bail!("Bundle path does not exist: {:?}", bundle_path);
        }

        Ok(Self {
            bundle_path,
            search_index: CoreSearchIndex::new(),
            entry_data: None,
            current_language: "unknown".to_string(),
            loaded_chunks: HashSet::new(),
            loaded_filter_chunks: HashSet::new(),
        })
    }

    /// Initialize the search context by loading entry metadata
    pub fn init(&mut self, language: Option<&str>) -> Result<()> {
        // Load pagefind-entry.json
        let entry_data = file_loader::load_entry_file(&self.bundle_path)?;
        
        // Determine which language to use
        let language = language.unwrap_or("unknown");
        let lang_data = self.find_language_index(&entry_data, language)?;
        let lang_hash = lang_data.hash.clone();
        
        self.current_language = language.to_string();
        self.entry_data = Some(entry_data);
        
        // Load the metadata for the selected language
        let meta_bytes = file_loader::load_metadata_file(&self.bundle_path, &lang_hash)?;
        self.search_index.decode_metadata(&meta_bytes)
            .map_err(|e| anyhow::anyhow!("Failed to decode metadata: {:?}", e))?;
        
        Ok(())
    }

    /// Find the appropriate language index from entry data
    fn find_language_index<'a>(&self, entry_data: &'a EntryData, language: &str) -> Result<&'a LanguageData> {
        // Try exact match
        if let Some(lang_data) = entry_data.languages.get(language) {
            return Ok(lang_data);
        }
        
        // Try language without region (e.g., "en" from "en-US")
        let base_lang = language.split('-').next().unwrap_or(language);
        if let Some(lang_data) = entry_data.languages.get(base_lang) {
            return Ok(lang_data);
        }
        
        // Fall back to the language with the most pages
        entry_data.languages
            .values()
            .max_by_key(|l| l.page_count)
            .ok_or_else(|| anyhow::anyhow!("No language indexes found"))
    }

    /// Set custom ranking weights
    pub fn set_ranking_weights(&mut self, weights: RankingWeights) {
        self.search_index.ranking_weights = weights;
    }

    /// Load chunks required for a search term
    fn load_required_chunks(&mut self, term: &str) -> Result<()> {
        // Get the list of required chunks for this search term
        let required_chunks = self.get_required_chunks(term);
        
        for chunk_hash in required_chunks {
            if !self.loaded_chunks.contains(&chunk_hash) {
                let chunk_data = file_loader::load_index_chunk(&self.bundle_path, &chunk_hash)?;
                self.search_index.decode_index_chunk(&chunk_data)
                    .map_err(|e| anyhow::anyhow!("Failed to decode index chunk: {:?}", e))?;
                self.loaded_chunks.insert(chunk_hash);
            }
        }
        
        Ok(())
    }

    /// Get the list of chunk hashes required for a search term
    fn get_required_chunks(&self, term: &str) -> Vec<String> {
        let mut required = Vec::new();
        let stems = pagefind_core_search::search::stems_from_term(term);
        
        for stem in stems {
            let stem_str = stem.as_ref();
            for chunk in &self.search_index.chunks {
                if stem_str >= chunk.from.as_str() && stem_str <= chunk.to.as_str() {
                    required.push(chunk.hash.clone());
                }
            }
        }
        
        required
    }

    /// Load filter chunks
    fn load_filter_chunks(&mut self, filters: &HashMap<String, Vec<String>>) -> Result<()> {
        for (filter_key, _) in filters {
            if let Some(chunk_hash) = self.search_index.filter_chunks.get(filter_key).cloned() {
                if !self.loaded_filter_chunks.contains(&chunk_hash) {
                    let filter_data = file_loader::load_filter_chunk(&self.bundle_path, &chunk_hash)?;
                    self.search_index.decode_filter_index_chunk(&filter_data)
                        .map_err(|e| anyhow::anyhow!("Failed to decode filter chunk: {:?}", e))?;
                    self.loaded_filter_chunks.insert(chunk_hash);
                }
            }
        }
        
        Ok(())
    }

    /// Build filter bitset from filter options
    fn build_filter_bitset(&self, filters: &HashMap<String, Vec<String>>) -> Option<BitSet> {
        if filters.is_empty() {
            return None;
        }
        
        let mut result_set: Option<BitSet> = None;
        
        for (filter_key, filter_values) in filters {
            if let Some(filter_data) = self.search_index.filters.get(filter_key) {
                let mut filter_set = BitSet::new();
                
                for value in filter_values {
                    if let Some(pages) = filter_data.get(value) {
                        for &page in pages {
                            filter_set.insert(page as usize);
                        }
                    }
                }
                
                result_set = match result_set {
                    Some(existing) => {
                        let mut new_set = existing;
                        new_set.intersect_with(&filter_set);
                        Some(new_set)
                    }
                    None => Some(filter_set),
                };
            }
        }
        
        result_set
    }

    /// Perform a search
    pub fn search(&mut self, query: &str, options: SearchOptions) -> Result<SearchResults> {
        // Load required chunks for the search term
        if !query.trim().is_empty() {
            self.load_required_chunks(query)?;
        }
        
        // Load filter chunks if filters are specified
        if !options.filters.is_empty() {
            self.load_filter_chunks(&options.filters)?;
        }
        
        // Build filter bitset
        let filter_bitset = self.build_filter_bitset(&options.filters);
        
        // Perform the search
        let exact_search = query.trim().starts_with('"') && query.trim().ends_with('"');
        let (unfiltered_results, mut results) = if exact_search {
            let clean_query = query.trim().trim_matches('"');
            self.search_index.exact_term(clean_query, filter_bitset, false)
        } else {
            self.search_index.search_term(query, filter_bitset, false)
        };
        
        // Apply sorting if requested
        if let Some((sort_key, sort_dir)) = &options.sort {
            if let Some(sort_data) = self.search_index.sorts.get(sort_key) {
                results.sort_by(|a, b| {
                    let a_pos = sort_data.iter().position(|&p| p as usize == a.page_index);
                    let b_pos = sort_data.iter().position(|&p| p as usize == b.page_index);
                    
                    match (a_pos, b_pos, sort_dir.as_str()) {
                        (Some(a), Some(b), "asc") => a.cmp(&b),
                        (Some(a), Some(b), "desc") => b.cmp(&a),
                        (Some(_), None, _) => std::cmp::Ordering::Less,
                        (None, Some(_), _) => std::cmp::Ordering::Greater,
                        _ => std::cmp::Ordering::Equal,
                    }
                });
            }
        }
        
        // Calculate filter counts
        let filter_counts = self.calculate_filter_counts(&results);
        let total_filter_counts = self.calculate_total_filter_counts();
        
        Ok(SearchResults {
            results,
            unfiltered_result_count: unfiltered_results.len(),
            filters: filter_counts,
            total_filters: total_filter_counts,
        })
    }

    /// Calculate filter counts for the current result set
    fn calculate_filter_counts(&self, results: &[PageSearchResult]) -> HashMap<String, HashMap<String, usize>> {
        let mut counts = HashMap::new();
        let result_pages: HashSet<usize> = results.iter().map(|r| r.page_index).collect();
        
        for (filter_key, filter_values) in &self.search_index.filters {
            let mut value_counts = HashMap::new();
            
            for (value, pages) in filter_values {
                let count = pages.iter()
                    .filter(|&&p| result_pages.contains(&(p as usize)))
                    .count();
                if count > 0 {
                    value_counts.insert(value.clone(), count);
                }
            }
            
            if !value_counts.is_empty() {
                counts.insert(filter_key.clone(), value_counts);
            }
        }
        
        counts
    }

    /// Calculate total filter counts across all pages
    fn calculate_total_filter_counts(&self) -> HashMap<String, HashMap<String, usize>> {
        let mut counts = HashMap::new();
        
        for (filter_key, filter_values) in &self.search_index.filters {
            let mut value_counts = HashMap::new();
            
            for (value, pages) in filter_values {
                value_counts.insert(value.clone(), pages.len());
            }
            
            counts.insert(filter_key.clone(), value_counts);
        }
        
        counts
    }

    /// Get available filters (loads all filter chunks)
    pub fn get_filters(&mut self) -> Result<HashMap<String, HashMap<String, usize>>> {
        // Load all filter chunks
        let filter_chunks: Vec<String> = self.search_index.filter_chunks.values().cloned().collect();
        for chunk_hash in filter_chunks {
            if !self.loaded_filter_chunks.contains(&chunk_hash) {
                let filter_data = file_loader::load_filter_chunk(&self.bundle_path, &chunk_hash)?;
                self.search_index.decode_filter_index_chunk(&filter_data)
                    .map_err(|e| anyhow::anyhow!("Failed to decode filter chunk: {:?}", e))?;
                self.loaded_filter_chunks.insert(chunk_hash);
            }
        }
        
        Ok(self.calculate_total_filter_counts())
    }

    /// Load a fragment for a search result
    pub fn load_fragment(&self, page_hash: &str) -> Result<PageFragment> {
        let fragment_data = file_loader::load_fragment(&self.bundle_path, page_hash)?;
        let fragment: PageFragment = serde_json::from_slice(&fragment_data)
            .context("Failed to parse fragment JSON")?;
        Ok(fragment)
    }

    /// Preload chunks for a query
    pub fn preload(&mut self, query: &str) -> Result<()> {
        if !query.trim().is_empty() {
            self.load_required_chunks(query)?;
        }
        Ok(())
    }
}

/// Search options
#[derive(Debug, Clone, Default)]
pub struct SearchOptions {
    pub filters: HashMap<String, Vec<String>>,
    pub sort: Option<(String, String)>, // (sort_key, direction)
}

/// Search results
#[derive(Debug)]
pub struct SearchResults {
    pub results: Vec<PageSearchResult>,
    pub unfiltered_result_count: usize,
    pub filters: HashMap<String, HashMap<String, usize>>,
    pub total_filters: HashMap<String, HashMap<String, usize>>,
}

/// Page fragment data
#[derive(Debug, serde::Deserialize)]
pub struct PageFragment {
    pub url: String,
    pub content: String,
    pub word_count: u32,
    pub filters: HashMap<String, Vec<String>>,
    pub meta: HashMap<String, String>,
    pub anchors: Vec<PageAnchor>,
}

#[derive(Debug, serde::Deserialize)]
pub struct PageAnchor {
    pub element: String,
    pub id: Option<String>,
    pub text: Option<String>,
    pub location: u32,
}

/// Configuration for native search
#[derive(Debug, Clone)]
pub struct NativeSearchConfig {
    pub bundle_path: PathBuf,
    pub language: Option<String>,
    pub ranking_weights: Option<RankingWeights>,
}

impl NativeSearchConfig {
    /// Create a new configuration with default values
    pub fn new<P: AsRef<Path>>(bundle_path: P) -> Self {
        Self {
            bundle_path: bundle_path.as_ref().to_path_buf(),
            language: None,
            ranking_weights: None,
        }
    }
}

/// Entry point data structure (mirrors pagefind-entry.json)
#[derive(Debug, serde::Deserialize)]
pub struct EntryData {
    pub version: String,
    pub languages: HashMap<String, LanguageData>,
    pub include_characters: Option<Vec<String>>,
}

#[derive(Debug, serde::Deserialize)]
pub struct LanguageData {
    pub hash: String,
    pub wasm: Option<String>, // Not used in native search
    pub page_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[test]
    fn test_native_search_creation() {
        let temp_dir = TempDir::new().unwrap();
        let bundle_path = temp_dir.path();
        
        // Create a dummy entry file
        let entry_path = bundle_path.join("pagefind-entry.json");
        fs::write(&entry_path, r#"{"version": "1.0.0", "languages": {}}"#).unwrap();
        
        let search = NativeSearch::new(bundle_path);
        assert!(search.is_ok());
    }

    #[test]
    fn test_invalid_bundle_path() {
        let result = NativeSearch::new("/non/existent/path");
        assert!(result.is_err());
    }
}