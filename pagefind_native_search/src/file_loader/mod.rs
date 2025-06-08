//! File loading and decompression functionality for native search

use anyhow::{Context, Result};
use flate2::read::GzDecoder;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Magic bytes for pagefind_dcd format
const PAGEFIND_DCD_MAGIC: &[u8] = b"pagefind_dcd";

/// Load and decompress a Pagefind file
pub fn load_pagefind_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    let path = path.as_ref();
    let mut file = File::open(path)
        .with_context(|| format!("Failed to open file: {:?}", path))?;
    
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)
        .with_context(|| format!("Failed to read file: {:?}", path))?;
    
    // Check if this is already decompressed (has magic bytes at start)
    if contents.starts_with(PAGEFIND_DCD_MAGIC) {
        // File is already decompressed, return data after magic bytes
        Ok(contents[PAGEFIND_DCD_MAGIC.len()..].to_vec())
    } else {
        // Try to decompress with gzip
        let mut gz = GzDecoder::new(&contents[..]);
        let mut decompressed = Vec::new();
        match gz.read_to_end(&mut decompressed) {
            Ok(_) => {
                // Check if decompressed data has the correct signature
                if decompressed.starts_with(PAGEFIND_DCD_MAGIC) {
                    Ok(decompressed[PAGEFIND_DCD_MAGIC.len()..].to_vec())
                } else {
                    anyhow::bail!("Decompressed file does not have the correct pagefind_dcd signature");
                }
            }
            Err(_) => {
                // Not gzipped, return as-is
                Ok(contents)
            }
        }
    }
}

/// Load the pagefind-entry.json file
pub fn load_entry_file(bundle_path: &Path) -> Result<crate::EntryData> {
    let entry_path = bundle_path.join("pagefind-entry.json");
    let contents = load_pagefind_file(&entry_path)?;
    let entry_data: crate::EntryData = serde_json::from_slice(&contents)
        .with_context(|| "Failed to parse pagefind-entry.json")?;
    Ok(entry_data)
}

/// Load a metadata file
pub fn load_metadata_file(bundle_path: &Path, hash: &str) -> Result<Vec<u8>> {
    let meta_path = bundle_path.join(format!("pagefind.{}.pf_meta", hash));
    load_pagefind_file(&meta_path)
}

/// Load an index chunk file
pub fn load_index_chunk(bundle_path: &Path, chunk_hash: &str) -> Result<Vec<u8>> {
    let chunk_path = bundle_path.join("index").join(format!("{}.pf_index", chunk_hash));
    load_pagefind_file(&chunk_path)
}

/// Load a filter chunk file
pub fn load_filter_chunk(bundle_path: &Path, filter_hash: &str) -> Result<Vec<u8>> {
    let filter_path = bundle_path.join("filter").join(format!("{}.pf_filter", filter_hash));
    load_pagefind_file(&filter_path)
}

/// Load a fragment file
pub fn load_fragment(bundle_path: &Path, fragment_hash: &str) -> Result<Vec<u8>> {
    let fragment_path = bundle_path.join("fragment").join(format!("{}.pf_fragment", fragment_hash));
    load_pagefind_file(&fragment_path)
}

/// List all available index chunks in a bundle
pub fn list_index_chunks(bundle_path: &Path) -> Result<Vec<String>> {
    let index_dir = bundle_path.join("index");
    if !index_dir.exists() {
        return Ok(Vec::new());
    }
    
    let mut chunks = Vec::new();
    for entry in std::fs::read_dir(&index_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("pf_index") {
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                chunks.push(stem.to_string());
            }
        }
    }
    
    Ok(chunks)
}

/// List all available filter chunks in a bundle
pub fn list_filter_chunks(bundle_path: &Path) -> Result<Vec<String>> {
    let filter_dir = bundle_path.join("filter");
    if !filter_dir.exists() {
        return Ok(Vec::new());
    }
    
    let mut chunks = Vec::new();
    for entry in std::fs::read_dir(&filter_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("pf_filter") {
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                chunks.push(stem.to_string());
            }
        }
    }
    
    Ok(chunks)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;
    use flate2::write::GzEncoder;
    use flate2::Compression;
    use std::io::Write;

    #[test]
    fn test_load_regular_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        let content = b"Hello, world!";
        fs::write(&file_path, content).unwrap();
        
        let loaded = load_pagefind_file(&file_path).unwrap();
        assert_eq!(loaded, content);
    }

    #[test]
    fn test_load_compressed_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.pf_index");
        let content = b"Compressed content";
        
        // Create a gzipped file with pagefind_dcd magic bytes
        let mut compressed_data = Vec::new();
        compressed_data.extend_from_slice(PAGEFIND_DCD_MAGIC);
        compressed_data.extend_from_slice(content);
        
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&compressed_data).unwrap();
        let gzipped = encoder.finish().unwrap();
        
        fs::write(&file_path, gzipped).unwrap();
        
        let loaded = load_pagefind_file(&file_path).unwrap();
        assert_eq!(loaded, content);
    }

    #[test]
    fn test_load_already_decompressed_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.pf_meta");
        let content = b"Already decompressed content";
        
        // Create a file that starts with magic bytes (already decompressed)
        let mut data = Vec::new();
        data.extend_from_slice(PAGEFIND_DCD_MAGIC);
        data.extend_from_slice(content);
        
        fs::write(&file_path, data).unwrap();
        
        let loaded = load_pagefind_file(&file_path).unwrap();
        assert_eq!(loaded, content);
    }

    #[test]
    fn test_list_chunks() {
        let temp_dir = TempDir::new().unwrap();
        let bundle_path = temp_dir.path();
        
        // Create index directory with some chunk files
        let index_dir = bundle_path.join("index");
        fs::create_dir(&index_dir).unwrap();
        fs::write(index_dir.join("chunk1.pf_index"), b"").unwrap();
        fs::write(index_dir.join("chunk2.pf_index"), b"").unwrap();
        fs::write(index_dir.join("other.txt"), b"").unwrap(); // Should be ignored
        
        let chunks = list_index_chunks(bundle_path).unwrap();
        assert_eq!(chunks.len(), 2);
        assert!(chunks.contains(&"chunk1".to_string()));
        assert!(chunks.contains(&"chunk2".to_string()));
    }
}