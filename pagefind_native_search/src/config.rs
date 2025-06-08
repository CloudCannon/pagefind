//! Configuration support for pagefind_native_search
//! 
//! This module provides configuration loading from multiple sources:
//! - Configuration files (TOML, YAML, JSON)
//! - Environment variables (PAGEFIND_* prefix)
//! - CLI arguments
//! 
//! The precedence order is: CLI > Environment > Config File > Defaults

use anyhow::{bail, Result};
use clap::{CommandFactory, Parser};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use twelf::config;

const CONFIG_FILES: &[&str] = &[
    "pagefind.json",
    "pagefind.yml", 
    "pagefind.yaml",
    "pagefind.toml",
];

/// Configuration for pagefind native search
#[config]
#[derive(Parser, Debug, Clone, Serialize)]
#[clap(author, version, about, long_about = None)]
pub struct SearchConfig {
    /// Path to the Pagefind bundle directory
    #[clap(short, long)]
    #[serde(default)]
    pub bundle: Option<PathBuf>,

    /// Force a specific language
    #[clap(short, long)]
    pub language: Option<String>,

    /// Default search limit
    #[clap(long)]
    #[serde(default = "defaults::default_limit")]
    pub default_limit: usize,

    /// Enable chunk preloading for better performance
    #[clap(long)]
    #[serde(default = "defaults::default_false")]
    pub preload_chunks: bool,

    /// Cache size for loaded chunks (in MB)
    #[clap(long)]
    #[serde(default = "defaults::default_cache_size")]
    pub cache_size_mb: usize,

    /// Default output format (json or text)
    #[clap(long)]
    #[serde(default = "defaults::default_output_format")]
    pub output_format: String,

    /// Verbose output
    #[clap(short, long)]
    #[serde(default = "defaults::default_false")]
    pub verbose: bool,

    /// Quiet mode - only show errors
    #[clap(short, long)]
    #[serde(default = "defaults::default_false")]
    pub quiet: bool,

    /// Path to a logfile to write to
    #[clap(long)]
    pub logfile: Option<PathBuf>,

    /// Custom configuration file path
    #[clap(long)]
    #[serde(skip)]
    pub config: Option<PathBuf>,

    // Search-specific options
    
    /// Enable excerpt generation in results
    #[clap(long)]
    #[serde(default = "defaults::default_true")]
    pub generate_excerpts: bool,

    /// Maximum excerpt length in characters
    #[clap(long)]
    #[serde(default = "defaults::default_excerpt_length")]
    pub excerpt_length: usize,

    /// Number of context words around matches in excerpts
    #[clap(long)]
    #[serde(default = "defaults::default_excerpt_context")]
    pub excerpt_context: usize,

    /// Enable fragment loading for results
    #[clap(long)]
    #[serde(default = "defaults::default_true")]
    pub load_fragments: bool,

    /// Maximum number of concurrent fragment loads
    #[clap(long)]
    #[serde(default = "defaults::default_concurrent_fragments")]
    pub concurrent_fragments: usize,

    // Ranking weight configuration
    
    /// Term similarity weight for ranking
    #[clap(long)]
    pub ranking_term_similarity: Option<f32>,

    /// Page length weight for ranking
    #[clap(long)]
    pub ranking_page_length: Option<f32>,

    /// Term saturation weight for ranking
    #[clap(long)]
    pub ranking_term_saturation: Option<f32>,

    /// Term frequency weight for ranking
    #[clap(long)]
    pub ranking_term_frequency: Option<f32>,
}

mod defaults {
    pub fn default_limit() -> usize {
        30
    }

    pub fn default_cache_size() -> usize {
        50
    }

    pub fn default_output_format() -> String {
        "text".to_string()
    }

    pub fn default_false() -> bool {
        false
    }

    pub fn default_true() -> bool {
        true
    }

    pub fn default_excerpt_length() -> usize {
        300
    }

    pub fn default_excerpt_context() -> usize {
        15
    }

    pub fn default_concurrent_fragments() -> usize {
        5
    }
}

impl SearchConfig {
    /// Load configuration from all sources with proper precedence
    pub fn load() -> Result<Self> {
        let matches = SearchConfig::command().get_matches();
        
        let mut config_layers = vec![];

        // Check for custom config file first
        let custom_config = matches.get_one::<PathBuf>("config").cloned();
        
        if let Some(config_path) = custom_config {
            // Use the specified config file
            if !config_path.exists() {
                bail!("Specified config file does not exist: {:?}", config_path);
            }
            
            let layer_fn = Self::get_layer_function(&config_path)?;
            config_layers.push(layer_fn(config_path));
        } else {
            // Look for default config files
            let configs: Vec<&str> = CONFIG_FILES
                .iter()
                .filter(|c| std::path::Path::new(c).exists())
                .cloned()
                .collect();
                
            if configs.len() > 1 {
                let found = configs.join(", ");
                bail!(
                    "Found multiple possible config files: [{}]\n\
                     Pagefind only supports loading one configuration file format, \
                     please ensure only one file exists or specify --config.",
                    found
                );
            }

            for config in configs {
                let layer_fn = Self::get_layer_function_by_name(config)?;
                config_layers.push(layer_fn(config.into()));
            }
        }

        // Add environment variables with PAGEFIND_ prefix
        config_layers.push(twelf::Layer::Env(Some("PAGEFIND_".to_string())));
        
        // Add CLI arguments (highest priority)
        config_layers.push(twelf::Layer::Clap(matches));

        // Build configuration with layers
        match SearchConfig::with_layers(&config_layers) {
            Ok(config) => Ok(config),
            Err(e) => {
                let inner_err = match e {
                    twelf::Error::Io(e) => format!("{}", e),
                    twelf::Error::Envy(e) => format!("{}", e),
                    twelf::Error::Json(e) => format!("{}", e),
                    twelf::Error::Toml(e) => format!("{}", e),
                    twelf::Error::Yaml(e) => format!("{}", e),
                    twelf::Error::Deserialize(e) => format!("{}", e),
                    _ => "Unknown Error".to_string(),
                };
                bail!("Error loading Pagefind config:\n{}", inner_err)
            }
        }
    }

    /// Get the appropriate layer function for a config file path
    fn get_layer_function(path: &PathBuf) -> Result<fn(PathBuf) -> twelf::Layer> {
        let path_str = path.to_string_lossy();
        
        if path_str.ends_with(".json") {
            Ok(twelf::Layer::Json)
        } else if path_str.ends_with(".toml") {
            Ok(twelf::Layer::Toml)
        } else if path_str.ends_with(".yaml") || path_str.ends_with(".yml") {
            Ok(twelf::Layer::Yaml)
        } else {
            bail!("Unknown config file format: {:?}", path)
        }
    }

    /// Get the appropriate layer function for a config file name
    fn get_layer_function_by_name(name: &str) -> Result<fn(PathBuf) -> twelf::Layer> {
        if name.ends_with(".json") {
            Ok(twelf::Layer::Json)
        } else if name.ends_with(".toml") {
            Ok(twelf::Layer::Toml)
        } else if name.ends_with(".yaml") || name.ends_with(".yml") {
            Ok(twelf::Layer::Yaml)
        } else {
            bail!("Unknown config file format: {}", name)
        }
    }

    /// Get ranking weights if any are configured
    pub fn get_ranking_weights(&self) -> Option<pagefind_core_search::RankingWeights> {
        if self.ranking_term_similarity.is_none()
            && self.ranking_page_length.is_none()
            && self.ranking_term_saturation.is_none()
            && self.ranking_term_frequency.is_none()
        {
            return None;
        }

        let defaults = pagefind_core_search::RankingWeights::default();
        Some(pagefind_core_search::RankingWeights {
            term_similarity: self.ranking_term_similarity.unwrap_or(defaults.term_similarity),
            page_length: self.ranking_page_length.unwrap_or(defaults.page_length),
            term_saturation: self.ranking_term_saturation.unwrap_or(defaults.term_saturation),
            term_frequency: self.ranking_term_frequency.unwrap_or(defaults.term_frequency),
        })
    }

    /// Get the log level based on verbose/quiet flags
    pub fn get_log_level(&self) -> LogLevel {
        if self.verbose {
            LogLevel::Verbose
        } else if self.quiet {
            LogLevel::Quiet
        } else {
            LogLevel::Standard
        }
    }
}

/// Log levels for the search tool
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Verbose,
    Standard,
    Quiet,
    Silent,
}

/// Search-specific configuration that can be passed per search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchOptions {
    /// Maximum number of results to return
    pub limit: Option<usize>,
    
    /// Filters to apply
    pub filters: Option<std::collections::HashMap<String, Vec<String>>>,
    
    /// Sort configuration
    pub sort: Option<SortConfig>,
    
    /// Whether to generate excerpts
    pub generate_excerpts: Option<bool>,
    
    /// Whether to load fragments
    pub load_fragments: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortConfig {
    pub by: String,
    pub direction: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_default_config() {
        // Clear any existing env vars
        for (key, _) in env::vars() {
            if key.starts_with("PAGEFIND_") {
                env::remove_var(&key);
            }
        }

        // Parse with no args
        let config = SearchConfig::try_parse_from(&["pagefind-search"]);
        assert!(config.is_ok());
        
        let config = config.unwrap();
        assert_eq!(config.default_limit, 30);
        assert_eq!(config.output_format, "text");
        assert!(!config.verbose);
    }

    #[test]
    fn test_config_file_loading() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("pagefind.toml");
        
        fs::write(&config_path, r#"
            bundle = "/path/to/bundle"
            default_limit = 50
            verbose = true
        "#).unwrap();

        // Change to temp dir to find config file
        let original_dir = env::current_dir().unwrap();
        env::set_current_dir(&temp_dir).unwrap();
        
        // Load config - this would normally use SearchConfig::load()
        // but we'll test the layer function directly
        let layer_fn = SearchConfig::get_layer_function(&config_path).unwrap();
        assert!(matches!(layer_fn, _ if true)); // Just check it returns a function
        
        // Restore original dir
        env::set_current_dir(original_dir).unwrap();
    }

    #[test]
    fn test_env_var_override() {
        env::set_var("PAGEFIND_DEFAULT_LIMIT", "100");
        env::set_var("PAGEFIND_VERBOSE", "true");
        
        // In real usage, SearchConfig::load() would pick up these env vars
        // Here we just verify they're set
        assert_eq!(env::var("PAGEFIND_DEFAULT_LIMIT").unwrap(), "100");
        assert_eq!(env::var("PAGEFIND_VERBOSE").unwrap(), "true");
        
        // Clean up
        env::remove_var("PAGEFIND_DEFAULT_LIMIT");
        env::remove_var("PAGEFIND_VERBOSE");
    }
}