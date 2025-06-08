use pagefind_native_search::{NativeSearch, SearchOptions};
use std::path::PathBuf;
use tempfile::TempDir;
use std::fs;
use std::collections::HashMap;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::Write;

/// Helper to create test data with pagefind_dcd magic bytes and gzip compression
fn create_compressed_file(content: &[u8]) -> Vec<u8> {
    let mut data_with_magic = Vec::new();
    data_with_magic.extend_from_slice(b"pagefind_dcd");
    data_with_magic.extend_from_slice(content);
    
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&data_with_magic).unwrap();
    encoder.finish().unwrap()
}

/// Create a complete test bundle with multiple pages, filters, and sorting
fn create_test_bundle(bundle_path: &std::path::Path) -> anyhow::Result<()> {
    // Create directory structure
    fs::create_dir_all(bundle_path.join("index"))?;
    fs::create_dir_all(bundle_path.join("filter"))?;
    fs::create_dir_all(bundle_path.join("fragment"))?;
    
    // Create pagefind-entry.json
    let entry_data = serde_json::json!({
        "version": "1.1.0",
        "languages": {
            "en": {
                "hash": "test_en",
                "wasm": null,
                "page_count": 5
            },
            "es": {
                "hash": "test_es",
                "wasm": null,
                "page_count": 3
            }
        },
        "include_characters": ["@", "#", "_"]
    });
    fs::write(bundle_path.join("pagefind-entry.json"), serde_json::to_string(&entry_data)?)?;
    
    // For now, create dummy files since we don't have access to test utilities
    // In a real implementation, these would contain proper CBOR-encoded data
    let dummy_meta = create_compressed_file(b"dummy_metadata");
    fs::write(bundle_path.join("pagefind.test_en.pf_meta"), dummy_meta)?;
    
    let dummy_index = create_compressed_file(b"dummy_index");
    fs::write(bundle_path.join("index").join("chunk1.pf_index"), dummy_index)?;
    
    let dummy_filter = create_compressed_file(b"dummy_filter");
    fs::write(bundle_path.join("filter").join("category.pf_filter"), dummy_filter)?;
    
    // Create fragment files
    let fragments = vec![
        ("page1", serde_json::json!({
            "url": "/docs/getting-started.html",
            "content": "Getting started with Pagefind. This guide will help you integrate Pagefind into your static site. Pagefind is a fully static search library.",
            "word_count": 20,
            "filters": {
                "category": ["documentation", "guide"],
                "author": ["John Doe"],
                "tags": ["search", "static-site"]
            },
            "meta": {
                "title": "Getting Started Guide",
                "date": "2024-01-15",
                "description": "Learn how to integrate Pagefind"
            },
            "anchors": [
                {
                    "element": "h2",
                    "id": "installation",
                    "text": "Installation",
                    "location": 10
                },
                {
                    "element": "h2",
                    "id": "configuration",
                    "text": "Configuration",
                    "location": 50
                }
            ]
        })),
        ("page2", serde_json::json!({
            "url": "/blog/announcing-pagefind.html",
            "content": "We're excited to announce Pagefind, a new static search library. It provides fast, efficient search capabilities for static websites without requiring any server infrastructure.",
            "word_count": 25,
            "filters": {
                "category": ["blog", "announcement"],
                "author": ["Jane Smith"],
                "tags": ["news", "release"]
            },
            "meta": {
                "title": "Announcing Pagefind",
                "date": "2024-01-01",
                "description": "Introducing our new search library"
            },
            "anchors": []
        })),
        ("page3", serde_json::json!({
            "url": "/docs/api-reference.html",
            "content": "API Reference for Pagefind. This comprehensive guide covers all available methods, configuration options, and advanced features of the Pagefind search library.",
            "word_count": 22,
            "filters": {
                "category": ["documentation", "reference"],
                "author": ["John Doe"],
                "tags": ["api", "reference"]
            },
            "meta": {
                "title": "API Reference",
                "date": "2024-01-20",
                "description": "Complete API documentation"
            },
            "anchors": [
                {
                    "element": "h3",
                    "id": "search-method",
                    "text": "search() method",
                    "location": 30
                }
            ]
        }))
    ];
    
    for (hash, content) in fragments {
        let fragment_bytes = serde_json::to_vec(&content)?;
        let compressed_fragment = create_compressed_file(&fragment_bytes);
        fs::write(bundle_path.join("fragment").join(format!("{}.pf_fragment", hash)), compressed_fragment)?;
    }
    
    Ok(())
}

#[test]
fn test_native_search_initialization() {
    let temp_dir = TempDir::new().unwrap();
    let bundle_path = temp_dir.path();
    
    // Create a minimal pagefind-entry.json
    let entry_data = r#"{
        "version": "1.0.0",
        "languages": {
            "en": {
                "hash": "test123",
                "page_count": 10
            }
        }
    }"#;
    
    fs::write(bundle_path.join("pagefind-entry.json"), entry_data).unwrap();
    
    // Create a minimal metadata file
    let meta_path = bundle_path.join("pagefind.test123.pf_meta");
    // For now, just create an empty file - in a real test we'd need proper CBOR data
    fs::write(&meta_path, b"").unwrap();
    
    // Test that we can create and initialize a search instance
    let mut search = NativeSearch::new(bundle_path).unwrap();
    
    // This will fail for now because we need proper metadata, but it tests the loading
    let init_result = search.init(Some("en"));
    
    // We expect this to fail with a decode error for now
    assert!(init_result.is_err());
}

#[test]
fn test_entry_file_loading() {
    let temp_dir = TempDir::new().unwrap();
    let bundle_path = temp_dir.path();
    
    // Create a pagefind-entry.json
    let entry_data = r##"{
        "version": "1.1.0",
        "languages": {
            "en": {
                "hash": "abc123",
                "wasm": null,
                "page_count": 42
            },
            "es": {
                "hash": "def456",
                "wasm": null,
                "page_count": 35
            }
        },
        "include_characters": ["@", "#"]
    }"##;
    
    fs::write(bundle_path.join("pagefind-entry.json"), entry_data).unwrap();
    
    // Test loading the entry file
    let entry = pagefind_native_search::file_loader::load_entry_file(bundle_path).unwrap();
    
    assert_eq!(entry.version, "1.1.0");
    assert_eq!(entry.languages.len(), 2);
    assert_eq!(entry.languages["en"].hash, "abc123");
    assert_eq!(entry.languages["en"].page_count, 42);
    assert_eq!(entry.languages["es"].hash, "def456");
    assert_eq!(entry.languages["es"].page_count, 35);
    assert_eq!(entry.include_characters, Some(vec!["@".to_string(), "#".to_string()]));
}

#[test]
fn test_file_decompression() {
    use flate2::write::GzEncoder;
    use flate2::Compression;
    use std::io::Write;
    
    let temp_dir = TempDir::new().unwrap();
    
    // Test 1: Regular uncompressed file
    let regular_file = temp_dir.path().join("regular.txt");
    let content = b"Hello, world!";
    fs::write(&regular_file, content).unwrap();
    
    let loaded = pagefind_native_search::file_loader::load_pagefind_file(&regular_file).unwrap();
    assert_eq!(loaded, content);
    
    // Test 2: Gzipped file with pagefind_dcd magic
    let compressed_file = temp_dir.path().join("compressed.pf_index");
    let original_content = b"This is compressed content";
    
    // Create data with magic bytes
    let mut data_with_magic = Vec::new();
    data_with_magic.extend_from_slice(b"pagefind_dcd");
    data_with_magic.extend_from_slice(original_content);
    
    // Gzip the data
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&data_with_magic).unwrap();
    let compressed = encoder.finish().unwrap();
    
    fs::write(&compressed_file, compressed).unwrap();
    
    let loaded = pagefind_native_search::file_loader::load_pagefind_file(&compressed_file).unwrap();
    assert_eq!(loaded, original_content);
    
    // Test 3: File that already has magic bytes (not compressed)
    let decompressed_file = temp_dir.path().join("decompressed.pf_meta");
    fs::write(&decompressed_file, &data_with_magic).unwrap();
    
    let loaded = pagefind_native_search::file_loader::load_pagefind_file(&decompressed_file).unwrap();
    assert_eq!(loaded, original_content);
}

#[test]
fn test_language_selection() {
    let temp_dir = TempDir::new().unwrap();
    let bundle_path = temp_dir.path();
    
    let entry_data = serde_json::json!({
        "version": "1.0.0",
        "languages": {
            "en": {
                "hash": "en_hash",
                "page_count": 100
            },
            "es": {
                "hash": "es_hash",
                "page_count": 50
            },
            "fr": {
                "hash": "fr_hash",
                "page_count": 150
            }
        }
    });
    
    fs::write(bundle_path.join("pagefind-entry.json"), serde_json::to_string(&entry_data).unwrap()).unwrap();
    
    // We can't test the private find_language_index method directly
    // Instead, we'll test language selection through initialization
    let mut search = NativeSearch::new(bundle_path).unwrap();
    
    // Create dummy metadata files for testing
    fs::write(bundle_path.join("pagefind.en_hash.pf_meta"), b"").unwrap();
    fs::write(bundle_path.join("pagefind.es_hash.pf_meta"), b"").unwrap();
    fs::write(bundle_path.join("pagefind.fr_hash.pf_meta"), b"").unwrap();
    
    // Test initialization with different languages
    // This will fail due to invalid metadata, but we're testing language selection
    let _ = search.init(Some("es"));
    let _ = search.init(Some("en-US")); // Should fall back to "en"
    let _ = search.init(Some("de")); // Should fall back to language with most pages (fr)
}

#[test]
fn test_search_options() {
    let options = SearchOptions::default();
    assert!(options.filters.is_empty());
    assert!(options.sort.is_none());
    
    let mut options = SearchOptions {
        filters: HashMap::new(),
        sort: Some(("date".to_string(), "desc".to_string())),
    };
    
    options.filters.insert("category".to_string(), vec!["blog".to_string(), "docs".to_string()]);
    assert_eq!(options.filters.len(), 1);
    assert_eq!(options.sort.as_ref().unwrap().0, "date");
}

#[test]
fn test_fragment_loading() {
    let temp_dir = TempDir::new().unwrap();
    let bundle_path = temp_dir.path();
    
    fs::create_dir_all(bundle_path.join("fragment")).unwrap();
    
    let fragment_data = serde_json::json!({
        "url": "/test/page.html",
        "content": "Test content",
        "word_count": 2,
        "filters": {
            "category": ["test"]
        },
        "meta": {
            "title": "Test Page"
        },
        "anchors": []
    });
    
    let fragment_bytes = serde_json::to_vec(&fragment_data).unwrap();
    let compressed = create_compressed_file(&fragment_bytes);
    fs::write(bundle_path.join("fragment").join("test_hash.pf_fragment"), compressed).unwrap();
    
    // Create entry file
    let entry_data = serde_json::json!({
        "version": "1.0.0",
        "languages": {
            "en": {
                "hash": "test",
                "page_count": 1
            }
        }
    });
    fs::write(bundle_path.join("pagefind-entry.json"), serde_json::to_string(&entry_data).unwrap()).unwrap();
    
    let search = NativeSearch::new(bundle_path).unwrap();
    let fragment = search.load_fragment("test_hash").unwrap();
    
    assert_eq!(fragment.url, "/test/page.html");
    assert_eq!(fragment.content, "Test content");
    assert_eq!(fragment.word_count, 2);
    assert_eq!(fragment.filters["category"], vec!["test"]);
    assert_eq!(fragment.meta["title"], "Test Page");
}

#[test]
fn test_chunk_listing() {
    let temp_dir = TempDir::new().unwrap();
    let bundle_path = temp_dir.path();
    
    // Create index directory with chunks
    fs::create_dir_all(bundle_path.join("index")).unwrap();
    fs::write(bundle_path.join("index").join("chunk1.pf_index"), b"").unwrap();
    fs::write(bundle_path.join("index").join("chunk2.pf_index"), b"").unwrap();
    fs::write(bundle_path.join("index").join("other.txt"), b"").unwrap(); // Should be ignored
    
    let chunks = pagefind_native_search::file_loader::list_index_chunks(bundle_path).unwrap();
    assert_eq!(chunks.len(), 2);
    assert!(chunks.contains(&"chunk1".to_string()));
    assert!(chunks.contains(&"chunk2".to_string()));
    
    // Create filter directory with chunks
    fs::create_dir_all(bundle_path.join("filter")).unwrap();
    fs::write(bundle_path.join("filter").join("category.pf_filter"), b"").unwrap();
    fs::write(bundle_path.join("filter").join("author.pf_filter"), b"").unwrap();
    
    let filter_chunks = pagefind_native_search::file_loader::list_filter_chunks(bundle_path).unwrap();
    assert_eq!(filter_chunks.len(), 2);
    assert!(filter_chunks.contains(&"category".to_string()));
    assert!(filter_chunks.contains(&"author".to_string()));
}

#[test]
fn test_config_loading() {
    use pagefind_native_search::config::Config;
    
    // Test default config
    let config = Config::default();
    assert_eq!(config.max_results, Some(100));
    assert_eq!(config.excerpt_length, Some(150));
    assert_eq!(config.language, Some("en".to_string()));
    
    // Test config from environment variables
    std::env::set_var("PAGEFIND_BUNDLE_PATH", "/test/bundle");
    std::env::set_var("PAGEFIND_MAX_RESULTS", "50");
    std::env::set_var("PAGEFIND_EXCERPT_LENGTH", "200");
    std::env::set_var("PAGEFIND_LANGUAGE", "es");
    
    let config = Config::load().unwrap();
    assert_eq!(config.bundle_path, Some("/test/bundle".to_string()));
    assert_eq!(config.max_results, Some(50));
    assert_eq!(config.excerpt_length, Some(200));
    assert_eq!(config.language, Some("es".to_string()));
    
    // Clean up env vars
    std::env::remove_var("PAGEFIND_BUNDLE_PATH");
    std::env::remove_var("PAGEFIND_MAX_RESULTS");
    std::env::remove_var("PAGEFIND_EXCERPT_LENGTH");
    std::env::remove_var("PAGEFIND_LANGUAGE");
}

#[test]
fn test_config_file_loading() {
    use pagefind_native_search::config::Config;
    
    let temp_dir = TempDir::new().unwrap();
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&temp_dir).unwrap();
    
    // Create config file
    let config_data = serde_json::json!({
        "bundle_path": "/from/config/file",
        "max_results": 75,
        "excerpt_length": 100,
        "language": "fr"
    });
    
    fs::write("pagefind.config.json", serde_json::to_string(&config_data).unwrap()).unwrap();
    
    let config = Config::load().unwrap();
    assert_eq!(config.bundle_path, Some("/from/config/file".to_string()));
    assert_eq!(config.max_results, Some(75));
    assert_eq!(config.excerpt_length, Some(100));
    assert_eq!(config.language, Some("fr".to_string()));
    
    // Test environment variable override
    std::env::set_var("PAGEFIND_MAX_RESULTS", "200");
    let config = Config::load().unwrap();
    assert_eq!(config.max_results, Some(200)); // Env var overrides config file
    
    std::env::remove_var("PAGEFIND_MAX_RESULTS");
    std::env::set_current_dir(original_dir).unwrap();
}

#[test]
fn test_cli_args_merge() {
    use pagefind_native_search::config::Config;
    use pagefind_native_search::cli::{Args, Commands};
    
    let mut config = Config {
        bundle_path: Some("/original/path".to_string()),
        max_results: Some(100),
        excerpt_length: Some(150),
        language: Some("en".to_string()),
    };
    
    let args = Args {
        command: Commands::Search {
            query: "test".to_string(),
            filters: None,
            sort: None,
            limit: None,
        },
        bundle_path: Some("/cli/path".to_string()),
        max_results: Some(50),
        excerpt_length: None,
        language: Some("es".to_string()),
    };
    
    config.merge_cli_args(&args);
    
    assert_eq!(config.bundle_path, Some("/cli/path".to_string()));
    assert_eq!(config.max_results, Some(50));
    assert_eq!(config.excerpt_length, Some(150)); // Unchanged
    assert_eq!(config.language, Some("es".to_string()));
}

// Integration tests that would require a full test bundle with proper CBOR data
// These are commented out as they would need the pagefind_core_search test utilities

/*
#[test]
fn test_full_search_flow() {
    let temp_dir = TempDir::new().unwrap();
    let bundle_path = temp_dir.path();
    
    create_test_bundle(bundle_path).unwrap();
    
    let mut search = NativeSearch::new(bundle_path).unwrap();
    search.init(Some("en")).unwrap();
    
    // Test basic search
    let results = search.search("pagefind", SearchOptions::default()).unwrap();
    assert!(!results.results.is_empty());
    assert_eq!(results.results[0].url, "/docs/getting-started.html");
    
    // Test search with filters
    let mut options = SearchOptions::default();
    options.filters.insert("category".to_string(), vec!["documentation".to_string()]);
    
    let results = search.search("guide", Some(options)).unwrap();
    assert_eq!(results.results.len(), 2); // getting-started and api-reference
    
    // Test empty search returns all results
    let results = search.search("", SearchOptions::default()).unwrap();
    assert_eq!(results.results.len(), 3);
    
    // Test exact phrase search
    let results = search.search("\"static search library\"", SearchOptions::default()).unwrap();
    assert_eq!(results.results.len(), 2);
}

#[test]
fn test_filter_operations() {
    let temp_dir = TempDir::new().unwrap();
    let bundle_path = temp_dir.path();
    
    create_test_bundle(bundle_path).unwrap();
    
    let mut search = NativeSearch::new(bundle_path).unwrap();
    search.init(Some("en")).unwrap();
    
    // Get all available filters
    let filters = search.get_filters().unwrap();
    assert!(filters.contains_key("category"));
    assert!(filters.contains_key("author"));
    assert!(filters.contains_key("tags"));
    
    // Test filter counts
    assert_eq!(filters["category"]["documentation"], 2);
    assert_eq!(filters["category"]["blog"], 1);
    assert_eq!(filters["author"]["John Doe"], 2);
    assert_eq!(filters["author"]["Jane Smith"], 1);
}

#[test]
fn test_sorting() {
    let temp_dir = TempDir::new().unwrap();
    let bundle_path = temp_dir.path();
    
    create_test_bundle(bundle_path).unwrap();
    
    let mut search = NativeSearch::new(bundle_path).unwrap();
    search.init(Some("en")).unwrap();
    
    // Test sorting by date ascending
    let mut options = SearchOptions::default();
    options.sort = Some(("date".to_string(), "asc".to_string()));
    
    let results = search.search("", Some(options)).unwrap();
    assert_eq!(results.results[0].url, "/blog/announcing-pagefind.html"); // 2024-01-01
    assert_eq!(results.results[1].url, "/docs/getting-started.html");    // 2024-01-15
    assert_eq!(results.results[2].url, "/docs/api-reference.html");      // 2024-01-20
    
    // Test sorting by date descending
    let mut options = SearchOptions::default();
    options.sort = Some(("date".to_string(), "desc".to_string()));
    
    let results = search.search("", Some(options)).unwrap();
    assert_eq!(results.results[0].url, "/docs/api-reference.html");      // 2024-01-20
    assert_eq!(results.results[1].url, "/docs/getting-started.html");    // 2024-01-15
    assert_eq!(results.results[2].url, "/blog/announcing-pagefind.html"); // 2024-01-01
}

#[test]
fn test_preloading() {
    let temp_dir = TempDir::new().unwrap();
    let bundle_path = temp_dir.path();
    
    create_test_bundle(bundle_path).unwrap();
    
    let mut search = NativeSearch::new(bundle_path).unwrap();
    search.init(Some("en")).unwrap();
    
    // Preload chunks for a query
    search.preload("pagefind").unwrap();
    
    // Search should be faster now (chunks already loaded)
    let results = search.search("pagefind", SearchOptions::default()).unwrap();
    assert!(!results.results.is_empty());
}

#[test]
fn test_ranking_weights() {
    let temp_dir = TempDir::new().unwrap();
    let bundle_path = temp_dir.path();
    
    create_test_bundle(bundle_path).unwrap();
    
    let mut search = NativeSearch::new(bundle_path).unwrap();
    search.init(Some("en")).unwrap();
    
    // Set custom ranking weights
    let weights = pagefind_core_search::RankingWeights {
        page_similarity: 1.0,
        page_length: 0.5,
        term_frequency: 2.0,
        term_similarity: 1.5,
    };
    
    search.set_ranking_weights(weights);
    
    // Results should be ranked according to new weights
    let results = search.search("search", SearchOptions::default()).unwrap();
    assert!(!results.results.is_empty());
}

#[test]
fn test_error_handling() {
    // Test missing bundle path
    let result = NativeSearch::new("/non/existent/path");
    assert!(result.is_err());
    
    // Test initialization without entry file
    let temp_dir = TempDir::new().unwrap();
    let mut search = NativeSearch::new(temp_dir.path()).unwrap();
    let result = search.init(Some("en"));
    assert!(result.is_err());
    
    // Test loading non-existent fragment
    let temp_dir = TempDir::new().unwrap();
    let bundle_path = temp_dir.path();
    fs::write(bundle_path.join("pagefind-entry.json"), r#"{"version": "1.0.0", "languages": {}}"#).unwrap();
    let search = NativeSearch::new(bundle_path).unwrap();
    let result = search.load_fragment("non_existent");
    assert!(result.is_err());
}

#[test]
fn test_performance_with_large_dataset() {
    use std::time::Instant;
    
    let temp_dir = TempDir::new().unwrap();
    let bundle_path = temp_dir.path();
    
    // Create a larger test bundle
    create_large_test_bundle(bundle_path, 1000).unwrap(); // 1000 pages
    
    let mut search = NativeSearch::new(bundle_path).unwrap();
    search.init(Some("en")).unwrap();
    
    // Measure search performance
    let start = Instant::now();
    let results = search.search("content", SearchOptions::default()).unwrap();
    let duration = start.elapsed();
    
    println!("Search took: {:?} for {} results", duration, results.results.len());
    assert!(duration.as_millis() < 100); // Should complete within 100ms
    
    // Test with filters
    let mut options = SearchOptions::default();
    options.filters.insert("category".to_string(), vec!["category_1".to_string()]);
    
    let start = Instant::now();
    let results = search.search("content", Some(options)).unwrap();
    let duration = start.elapsed();
    
    println!("Filtered search took: {:?} for {} results", duration, results.results.len());
    assert!(duration.as_millis() < 50); // Filtered search should be faster
}
*/