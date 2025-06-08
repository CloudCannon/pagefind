//! CLI interface for Pagefind native search

use anyhow::Result;
use clap::{Parser, Subcommand};
use pagefind_native_search::{NativeSearch, config::SearchConfig};
use std::path::PathBuf;

/// Pagefind native search CLI
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    /// Custom configuration file path
    #[arg(long, global = true)]
    config: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Search a Pagefind index
    Search {
        /// Search query
        #[arg(short, long)]
        query: String,

        /// Path to the Pagefind bundle directory (overrides config)
        #[arg(short, long)]
        bundle: Option<PathBuf>,

        /// Force a specific language (overrides config)
        #[arg(short, long)]
        language: Option<String>,

        /// Output format (json or text) (overrides config)
        #[arg(short, long)]
        output: Option<OutputFormat>,

        /// Filters as JSON string
        #[arg(short, long)]
        filters: Option<String>,

        /// Sort options as JSON string
        #[arg(short, long)]
        sort: Option<String>,

        /// Maximum number of results (overrides config)
        #[arg(long)]
        limit: Option<usize>,

        /// Verbose output (overrides config)
        #[arg(short, long)]
        verbose: bool,
    },

    /// List available filters in the index
    Filters {
        /// Path to the Pagefind bundle directory (overrides config)
        #[arg(short, long)]
        bundle: Option<PathBuf>,

        /// Force a specific language (overrides config)
        #[arg(short, long)]
        language: Option<String>,

        /// Output format (json or text) (overrides config)
        #[arg(short, long)]
        output: Option<OutputFormat>,
    },
}

#[derive(Debug, Clone, Copy)]
enum OutputFormat {
    Json,
    Text,
}

impl std::str::FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "text" => Ok(OutputFormat::Text),
            _ => Err(format!("Unknown output format: {}", s)),
        }
    }
}

fn main() -> Result<()> {
    // Parse CLI args first to get potential config file path
    let cli = Cli::parse();
    
    // Load configuration from all sources
    let config = SearchConfig::load()?;
    
    match cli.command {
        Commands::Search {
            query,
            bundle,
            language,
            output,
            filters,
            sort,
            limit,
            verbose,
        } => {
            // Use CLI args to override config values
            let bundle_path = bundle.or(config.bundle.clone())
                .ok_or_else(|| anyhow::anyhow!("Bundle path not specified. Use --bundle or set in config."))?;
            let language = language.or(config.language.clone());
            let output_format = output.unwrap_or_else(|| {
                match config.output_format.as_str() {
                    "json" => OutputFormat::Json,
                    _ => OutputFormat::Text,
                }
            });
            let limit = limit.or(Some(config.default_limit));
            let verbose = verbose || config.verbose;
            
            search_command(
                bundle_path,
                query,
                language,
                output_format,
                filters,
                sort,
                limit,
                verbose,
                &config,
            )?;
        }
        Commands::Filters {
            bundle,
            language,
            output,
        } => {
            // Use CLI args to override config values
            let bundle_path = bundle.or(config.bundle.clone())
                .ok_or_else(|| anyhow::anyhow!("Bundle path not specified. Use --bundle or set in config."))?;
            let language = language.or(config.language.clone());
            let output_format = output.unwrap_or_else(|| {
                match config.output_format.as_str() {
                    "json" => OutputFormat::Json,
                    _ => OutputFormat::Text,
                }
            });
            
            filters_command(bundle_path, language, output_format)?;
        }
    }

    Ok(())
}

fn search_command(
    bundle: PathBuf,
    query: String,
    language: Option<String>,
    output: OutputFormat,
    filters: Option<String>,
    sort: Option<String>,
    limit: Option<usize>,
    verbose: bool,
    config: &SearchConfig,
) -> Result<()> {
    if verbose {
        eprintln!("Searching in bundle: {:?}", bundle);
        eprintln!("Query: {}", query);
        if let Some(ref lang) = language {
            eprintln!("Language: {}", lang);
        }
    }

    // Create and initialize search
    let mut search = NativeSearch::new(bundle)?;
    search.init(language.as_deref())?;
    
    // Apply ranking weights from config if specified
    if let Some(weights) = config.get_ranking_weights() {
        search.set_ranking_weights(weights);
    }

    // Parse filters from JSON string
    let mut search_options = pagefind_native_search::SearchOptions::default();
    
    if let Some(filters_json) = filters {
        let parsed_filters: std::collections::HashMap<String, Vec<String>> =
            serde_json::from_str(&filters_json)?;
        search_options.filters = parsed_filters;
    }
    
    if let Some(sort_json) = sort {
        let parsed_sort: (String, String) = serde_json::from_str(&sort_json)?;
        search_options.sort = Some(parsed_sort);
    }

    // Perform search
    let results = search.search(&query, search_options)?;
    
    // Apply limit if specified
    let limited_results: Vec<_> = if let Some(limit) = limit {
        results.results.into_iter().take(limit).collect()
    } else {
        results.results
    };

    // Output results
    match output {
        OutputFormat::Json => {
            let json_output = serde_json::json!({
                "results": limited_results.iter().map(|r| {
                    serde_json::json!({
                        "page": r.page,
                        "score": r.page_score,
                        "word_count": r.page_length,
                        "word_locations": r.word_locations.len()
                    })
                }).collect::<Vec<_>>(),
                "total_results": limited_results.len(),
                "unfiltered_count": results.unfiltered_result_count,
                "filters": results.filters
            });
            println!("{}", serde_json::to_string_pretty(&json_output)?);
        }
        OutputFormat::Text => {
            println!("Found {} results for '{}' ({} unfiltered)",
                limited_results.len(), query, results.unfiltered_result_count);
            
            for (i, result) in limited_results.iter().enumerate() {
                println!("\n{}. Page: {} (score: {:.2})",
                    i + 1, result.page, result.page_score);
                println!("   Word count: {}", result.page_length);
                println!("   Matched locations: {}", result.word_locations.len());
                
                // Try to load fragment for more details
                if verbose && config.load_fragments {
                    match search.load_fragment(&result.page) {
                        Ok(fragment) => {
                            println!("   URL: {}", fragment.url);
                            if let Some(title) = fragment.meta.get("title") {
                                println!("   Title: {}", title);
                            }
                            
                            // Generate excerpt if enabled
                            if config.generate_excerpts {
                                // TODO: Implement excerpt generation
                                // This would use the fragment content and word locations
                                // to generate a contextual excerpt
                            }
                        }
                        Err(e) => {
                            eprintln!("   Could not load fragment: {}", e);
                        }
                    }
                }
            }
            
            if !results.filters.is_empty() {
                println!("\nActive filters:");
                for (filter_name, values) in &results.filters {
                    println!("  {}:", filter_name);
                    for (value, count) in values {
                        println!("    - {} ({})", value, count);
                    }
                }
            }
        }
    }

    Ok(())
}

fn filters_command(
    bundle: PathBuf,
    language: Option<String>,
    output: OutputFormat,
) -> Result<()> {
    // Create and initialize search
    let mut search = NativeSearch::new(bundle)?;
    search.init(language.as_deref())?;

    // Get filters
    let filters = search.get_filters()?;

    // Output filters
    match output {
        OutputFormat::Json => {
            let json = serde_json::to_string_pretty(&filters)?;
            println!("{}", json);
        }
        OutputFormat::Text => {
            if filters.is_empty() {
                println!("No filters available");
            } else {
                println!("Available filters:");
                for (filter_name, values) in filters {
                    println!("\n{}:", filter_name);
                    for (value, count) in values {
                        println!("  - {} ({})", value, count);
                    }
                }
            }
        }
    }

    Ok(())
}