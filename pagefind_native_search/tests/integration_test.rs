use pagefind_native_search::{NativeSearch, SearchOptions};
use std::path::PathBuf;
use tempfile::TempDir;
use std::fs;

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
    let entry_data = r#"{
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
    }"#;
    
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