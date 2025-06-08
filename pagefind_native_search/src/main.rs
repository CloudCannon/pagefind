//! CLI interface for Pagefind native search

use anyhow::Result;
use clap::{Parser, Subcommand};
use pagefind_native_search::{NativeSearch, NativeSearchConfig};
use std::path::PathBuf;

/// Pagefind native search CLI
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Search a Pagefind index
    Search {
        /// Path to the Pagefind bundle directory
        #[arg(short, long)]
        bundle: PathBuf,

        /// Search query
        #[arg(short, long)]
        query: String,

        /// Force a specific language
        #[arg(short, long)]
        language: Option<String>,

        /// Output format (json or text)
        #[arg(short, long, default_value = "text")]
        output: OutputFormat,

        /// Filters as JSON string
        #[arg(short, long)]
        filters: Option<String>,

        /// Sort options as JSON string
        #[arg(short, long)]
        sort: Option<String>,

        /// Maximum number of results
        #[arg(long)]
        limit: Option<usize>,

        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },

    /// List available filters in the index
    Filters {
        /// Path to the Pagefind bundle directory
        #[arg(short, long)]
        bundle: PathBuf,

        /// Force a specific language
        #[arg(short, long)]
        language: Option<String>,

        /// Output format (json or text)
        #[arg(short, long, default_value = "text")]
        output: OutputFormat,
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
    let cli = Cli::parse();

    match cli.command {
        Commands::Search {
            bundle,
            query,
            language,
            output,
            filters,
            sort,
            limit,
            verbose,
        } => {
            search_command(bundle, query, language, output, filters, sort, limit, verbose)?;
        }
        Commands::Filters {
            bundle,
            language,
            output,
        } => {
            filters_command(bundle, language, output)?;
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
) -> Result<()> {
    if verbose {
        eprintln!("Searching in bundle: {:?}", bundle);
        eprintln!("Query: {}", query);
    }

    // Create and initialize search
    let mut search = NativeSearch::new(bundle)?;
    search.init()?;

    if let Some(lang) = language {
        search.set_language(lang);
    }

    // TODO: Parse filters and sort options from JSON strings
    let search_options = pagefind_core_search::SearchOptions {
        limit,
        ..Default::default()
    };

    // Perform search
    let results = search.search(&query, Some(search_options))?;

    // Output results
    match output {
        OutputFormat::Json => {
            let json = serde_json::to_string_pretty(&results)?;
            println!("{}", json);
        }
        OutputFormat::Text => {
            println!("Found {} results for '{}'", results.len(), query);
            for (i, result) in results.iter().enumerate() {
                println!("\n{}. {} (score: {:.2})", i + 1, result.page_id, result.score);
                if !result.words.is_empty() {
                    println!("   Matched words: {}", result.words.join(", "));
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
    search.init()?;

    if let Some(lang) = language {
        search.set_language(lang);
    }

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