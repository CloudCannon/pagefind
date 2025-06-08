//! Native search implementation for Pagefind
//! 
//! This crate provides file system-based search capabilities for Pagefind indexes,
//! allowing searches to be performed directly from Rust without requiring a browser
//! or WebAssembly environment.

pub mod file_loader;
pub mod config;
pub mod cli;

use anyhow::{Context, Result};
use pagefind_core_search::{SearchIndex, SearchOptions, SearchResult, RankingWeights};
use std::path::{Path, PathBuf};
use std::collections::HashMap;

/// Main native search context
pub struct NativeSearch {
    bundle_path: PathBuf,
    search_index: SearchIndex,
    language: Option<String>,
    ranking_weights: RankingWeights,
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
            search_index: SearchIndex::new(),
            language: None,
            ranking_weights: RankingWeights::default(),
        })
    }

    /// Initialize the search context by loading entry metadata
    pub fn init(&mut self) -> Result<()> {
        // Load pagefind-entry.json
        let entry_data = file_loader::load_entry_file(&self.bundle_path)?;
        
        // TODO: Parse entry data and initialize search index
        
        Ok(())
    }

    /// Set the language to use for searching
    pub fn set_language(&mut self, language: String) {
        self.language = Some(language);
    }

    /// Set custom ranking weights
    pub fn set_ranking_weights(&mut self, weights: RankingWeights) {
        self.ranking_weights = weights;
        self.search_index.set_ranking_weights(weights);
    }

    /// Perform a search
    pub fn search(&mut self, query: &str, options: Option<SearchOptions>) -> Result<Vec<SearchResult>> {
        let options = options.unwrap_or_default();
        
        // TODO: Implement chunk loading based on query
        // For now, return empty results
        Ok(self.search_index.search(query, options))
    }

    /// Get available filters
    pub fn get_filters(&self) -> Result<HashMap<String, HashMap<String, usize>>> {
        // TODO: Load and return filter data
        Ok(HashMap::new())
    }

    /// Preload data for a query
    pub fn preload(&mut self, query: &str) -> Result<()> {
        // TODO: Implement preloading logic
        Ok(())
    }
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
    pub include_characters: Option<String>,
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