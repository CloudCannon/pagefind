//! CLI-specific utilities and helpers

use anyhow::Result;
use pagefind_core_search::RankingWeights;
use serde::Serialize;
use std::collections::HashMap;

/// CLI-friendly search result format
#[derive(Debug, Serialize)]
pub struct CliSearchResult {
    pub id: String,
    pub url: Option<String>,
    pub title: Option<String>,
    pub score: f32,
    pub words: Vec<String>,
    pub excerpt: Option<String>,
    pub meta: HashMap<String, String>,
}

// Note: This conversion is not used in the current implementation
// as we handle PageSearchResult directly in main.rs

/// CLI search response format
#[derive(Debug, Serialize)]
pub struct CliSearchResponse {
    pub query: String,
    pub total_results: usize,
    pub results: Vec<CliSearchResult>,
    pub filters: Option<HashMap<String, HashMap<String, usize>>>,
    pub time_ms: Option<u64>,
}

/// Parse ranking weights from CLI arguments
pub fn parse_ranking_weights(
    term_similarity: Option<f32>,
    page_length: Option<f32>,
    term_saturation: Option<f32>,
    term_frequency: Option<f32>,
) -> Option<RankingWeights> {
    if term_similarity.is_none() 
        && page_length.is_none() 
        && term_saturation.is_none() 
        && term_frequency.is_none() {
        return None;
    }

    let defaults = RankingWeights::default();
    Some(RankingWeights {
        term_similarity: term_similarity.unwrap_or(defaults.term_similarity),
        page_length: page_length.unwrap_or(defaults.page_length),
        term_saturation: term_saturation.unwrap_or(defaults.term_saturation),
        term_frequency: term_frequency.unwrap_or(defaults.term_frequency),
    })
}

/// Format search results for human-readable output
pub fn format_results_text(response: &CliSearchResponse) -> String {
    let mut output = String::new();
    
    output.push_str(&format!(
        "Found {} results for '{}'\n",
        response.total_results,
        response.query
    ));

    if let Some(time_ms) = response.time_ms {
        output.push_str(&format!("Search completed in {}ms\n", time_ms));
    }

    output.push('\n');

    for (i, result) in response.results.iter().enumerate() {
        output.push_str(&format!("{}. ", i + 1));
        
        if let Some(title) = &result.title {
            output.push_str(&format!("{} ", title));
        } else {
            output.push_str(&format!("{} ", result.id));
        }
        
        output.push_str(&format!("(score: {:.2})\n", result.score));
        
        if let Some(url) = &result.url {
            output.push_str(&format!("   URL: {}\n", url));
        }
        
        if !result.words.is_empty() {
            output.push_str(&format!("   Matched: {}\n", result.words.join(", ")));
        }
        
        if let Some(excerpt) = &result.excerpt {
            output.push_str(&format!("   {}\n", excerpt));
        }
        
        output.push('\n');
    }

    output
}

/// Format filters for human-readable output
pub fn format_filters_text(filters: &HashMap<String, HashMap<String, usize>>) -> String {
    let mut output = String::new();
    
    if filters.is_empty() {
        output.push_str("No filters available\n");
        return output;
    }
    
    output.push_str("Available filters:\n\n");
    
    for (filter_name, values) in filters {
        output.push_str(&format!("{}:\n", filter_name));
        
        let mut sorted_values: Vec<_> = values.iter().collect();
        sorted_values.sort_by(|a, b| b.1.cmp(a.1)); // Sort by count descending
        
        for (value, count) in sorted_values {
            output.push_str(&format!("  {} ({})\n", value, count));
        }
        
        output.push('\n');
    }
    
    output
}

/// Validate CLI arguments
pub fn validate_args(bundle_path: &std::path::Path) -> Result<()> {
    if !bundle_path.exists() {
        anyhow::bail!("Bundle path does not exist: {:?}", bundle_path);
    }
    
    if !bundle_path.is_dir() {
        anyhow::bail!("Bundle path is not a directory: {:?}", bundle_path);
    }
    
    let entry_file = bundle_path.join("pagefind-entry.json");
    if !entry_file.exists() {
        anyhow::bail!(
            "Bundle path does not contain pagefind-entry.json. \
             Is this a valid Pagefind bundle directory?"
        );
    }
    
    Ok(())
}

/// Progress indicator for long operations
pub struct ProgressIndicator {
    #[allow(dead_code)]
    message: String,
    verbose: bool,
}

impl ProgressIndicator {
    pub fn new(message: String, verbose: bool) -> Self {
        if verbose {
            eprint!("{}...", message);
        }
        Self { message, verbose }
    }
    
    pub fn update(&self, status: &str) {
        if self.verbose {
            eprint!(" {}", status);
        }
    }
    
    pub fn finish(&self) {
        if self.verbose {
            eprintln!(" done");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ranking_weights() {
        // No weights provided
        assert!(parse_ranking_weights(None, None, None, None).is_none());
        
        // Some weights provided
        let weights = parse_ranking_weights(Some(2.0), None, Some(1.0), None).unwrap();
        assert_eq!(weights.term_similarity, 2.0);
        assert_eq!(weights.page_length, 0.75); // Default
        assert_eq!(weights.term_saturation, 1.0);
        assert_eq!(weights.term_frequency, 1.0); // Default
    }

    #[test]
    fn test_format_results_text() {
        let response = CliSearchResponse {
            query: "test query".to_string(),
            total_results: 2,
            results: vec![
                CliSearchResult {
                    id: "page1".to_string(),
                    url: Some("/page1.html".to_string()),
                    title: Some("Page One".to_string()),
                    score: 0.95,
                    words: vec!["test".to_string(), "query".to_string()],
                    excerpt: Some("This is a test excerpt...".to_string()),
                    meta: HashMap::new(),
                },
            ],
            filters: None,
            time_ms: Some(25),
        };
        
        let output = format_results_text(&response);
        assert!(output.contains("Found 2 results"));
        assert!(output.contains("test query"));
        assert!(output.contains("25ms"));
        assert!(output.contains("Page One"));
        assert!(output.contains("0.95"));
    }

    #[test]
    fn test_format_filters_text() {
        let mut filters = HashMap::new();
        let mut category_values = HashMap::new();
        category_values.insert("tech".to_string(), 10);
        category_values.insert("news".to_string(), 5);
        filters.insert("category".to_string(), category_values);
        
        let output = format_filters_text(&filters);
        assert!(output.contains("category:"));
        assert!(output.contains("tech (10)"));
        assert!(output.contains("news (5)"));
    }
}