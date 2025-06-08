//! Configuration structures and parsing for native search

use anyhow::{Context, Result};
use pagefind_core_search::RankingWeights;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Search configuration that can be loaded from files or provided programmatically
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchConfig {
    /// Path to the Pagefind bundle
    pub bundle_path: PathBuf,
    
    /// Force a specific language
    pub language: Option<String>,
    
    /// Custom ranking weights
    pub ranking_weights: Option<RankingWeightsConfig>,
    
    /// Default filters to apply
    pub default_filters: Option<HashMap<String, Vec<String>>>,
    
    /// Default sort options
    pub default_sort: Option<SortConfig>,
    
    /// Maximum number of results to return by default
    pub default_limit: Option<usize>,
    
    /// Excerpt length in characters
    pub excerpt_length: Option<usize>,
}

impl SearchConfig {
    /// Create a new search configuration with minimal settings
    pub fn new<P: AsRef<Path>>(bundle_path: P) -> Self {
        Self {
            bundle_path: bundle_path.as_ref().to_path_buf(),
            language: None,
            ranking_weights: None,
            default_filters: None,
            default_sort: None,
            default_limit: None,
            excerpt_length: None,
        }
    }

    /// Load configuration from a JSON file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let contents = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {:?}", path))?;
        let config: Self = serde_json::from_str(&contents)
            .with_context(|| format!("Failed to parse config file: {:?}", path))?;
        Ok(config)
    }

    /// Save configuration to a JSON file
    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();
        let json = serde_json::to_string_pretty(self)
            .context("Failed to serialize config")?;
        std::fs::write(path, json)
            .with_context(|| format!("Failed to write config file: {:?}", path))?;
        Ok(())
    }

    /// Convert to core RankingWeights
    pub fn get_ranking_weights(&self) -> RankingWeights {
        self.ranking_weights
            .as_ref()
            .map(|w| w.to_core_weights())
            .unwrap_or_default()
    }
}

/// Ranking weights configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankingWeightsConfig {
    pub page_length: Option<f32>,
    pub term_frequency: Option<f32>,
    pub term_similarity: Option<f32>,
    pub term_saturation: Option<f32>,
}

impl RankingWeightsConfig {
    /// Convert to core RankingWeights, using defaults for missing values
    pub fn to_core_weights(&self) -> RankingWeights {
        let defaults = RankingWeights::default();
        RankingWeights {
            page_length: self.page_length.unwrap_or(defaults.page_length),
            term_frequency: self.term_frequency.unwrap_or(defaults.term_frequency),
            term_similarity: self.term_similarity.unwrap_or(defaults.term_similarity),
            term_saturation: self.term_saturation.unwrap_or(defaults.term_saturation),
        }
    }
}

/// Sort configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortConfig {
    pub field: String,
    pub order: String, // "asc" or "desc"
}

/// Runtime search options that can override config defaults
#[derive(Debug, Clone, Default)]
pub struct RuntimeSearchOptions {
    pub filters: Option<HashMap<String, Vec<String>>>,
    pub sort: Option<SortConfig>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub excerpt_length: Option<usize>,
}

impl RuntimeSearchOptions {
    /// Parse from JSON strings (for CLI usage)
    pub fn from_json_strings(
        filters_json: Option<&str>,
        sort_json: Option<&str>,
    ) -> Result<Self> {
        let mut options = Self::default();

        if let Some(json) = filters_json {
            options.filters = Some(
                serde_json::from_str(json)
                    .context("Failed to parse filters JSON")?
            );
        }

        if let Some(json) = sort_json {
            options.sort = Some(
                serde_json::from_str(json)
                    .context("Failed to parse sort JSON")?
            );
        }

        Ok(options)
    }

    /// Merge with config defaults to produce final search options
    pub fn merge_with_config(&self, config: &SearchConfig) -> pagefind_core_search::SearchOptions {
        let filters = self.filters.clone()
            .or_else(|| config.default_filters.clone());

        let sort = self.sort.as_ref()
            .or(config.default_sort.as_ref())
            .map(|s| {
                pagefind_core_search::SortOptions::new(
                    s.field.clone(),
                    s.order.as_str().into(),
                )
            });

        pagefind_core_search::SearchOptions {
            filters: filters.map(|f| {
                let mut filter_set = pagefind_core_search::FilterSet::new();
                for (name, values) in f {
                    filter_set.add_filter(name, values);
                }
                filter_set
            }),
            sort,
            limit: self.limit.or(config.default_limit),
            offset: self.offset,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_config_creation() {
        let config = SearchConfig::new("/test/path");
        assert_eq!(config.bundle_path, PathBuf::from("/test/path"));
        assert!(config.language.is_none());
    }

    #[test]
    fn test_config_serialization() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.json");

        let mut config = SearchConfig::new("/test/bundle");
        config.language = Some("en".to_string());
        config.default_limit = Some(50);

        // Save and reload
        config.to_file(&config_path).unwrap();
        let loaded = SearchConfig::from_file(&config_path).unwrap();

        assert_eq!(loaded.bundle_path, config.bundle_path);
        assert_eq!(loaded.language, Some("en".to_string()));
        assert_eq!(loaded.default_limit, Some(50));
    }

    #[test]
    fn test_ranking_weights_conversion() {
        let mut weights_config = RankingWeightsConfig {
            page_length: Some(0.5),
            term_frequency: None,
            term_similarity: Some(2.0),
            term_saturation: None,
        };

        let core_weights = weights_config.to_core_weights();
        assert_eq!(core_weights.page_length, 0.5);
        assert_eq!(core_weights.term_frequency, 1.0); // Default
        assert_eq!(core_weights.term_similarity, 2.0);
        assert_eq!(core_weights.term_saturation, 1.5); // Default
    }

    #[test]
    fn test_runtime_options_parsing() {
        let filters_json = r#"{"category": ["tech", "news"], "author": ["alice"]}"#;
        let sort_json = r#"{"field": "date", "order": "desc"}"#;

        let options = RuntimeSearchOptions::from_json_strings(
            Some(filters_json),
            Some(sort_json),
        ).unwrap();

        assert!(options.filters.is_some());
        let filters = options.filters.unwrap();
        assert_eq!(filters.get("category").unwrap(), &vec!["tech", "news"]);

        assert!(options.sort.is_some());
        let sort = options.sort.unwrap();
        assert_eq!(sort.field, "date");
        assert_eq!(sort.order, "desc");
    }
}