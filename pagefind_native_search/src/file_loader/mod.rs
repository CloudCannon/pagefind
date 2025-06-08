//! File loading and decompression functionality for native search

use anyhow::{Context, Result};
use flate2::read::GzDecoder;
use std::fs::File;
use std::io::{Read, BufReader};
use std::path::{Path, PathBuf};

/// Magic bytes for pagefind_dcd format
const PAGEFIND_DCD_MAGIC: &[u8] = b"pagefind_dcd";

/// Load and decompress a Pagefind file
pub fn load_pagefind_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    let path = path.as_ref();
    let mut file = File::open(path)
        .with_context(|| format!("Failed to open file: {:?}", path))?;
    
    // Read the first bytes to check for magic signature
    let mut magic_buffer = vec![0u8; PAGEFIND_DCD_MAGIC.len()];
    file.read_exact(&mut magic_buffer)
        .with_context(|| format!("Failed to read magic bytes from: {:?}", path))?;
    
    // Reset file position
    file = File::open(path)?;
    
    if &magic_buffer == PAGEFIND_DCD_MAGIC {
        // This is a pagefind_dcd file, skip the magic bytes and decompress
        let mut reader = BufReader::new(file);
        reader.read_exact(&mut magic_buffer)?; // Skip magic bytes
        
        let mut gz = GzDecoder::new(reader);
        let mut decompressed = Vec::new();
        gz.read_to_end(&mut decompressed)
            .with_context(|| format!("Failed to decompress file: {:?}", path))?;
        
        Ok(decompressed)
    } else {
        // Regular file, read as-is
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)
            .with_context(|| format!("Failed to read file: {:?}", path))?;
        Ok(contents)
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
        
        // Create a pagefind_dcd compressed file
        let mut file = File::create(&file_path).unwrap();
        file.write_all(PAGEFIND_DCD_MAGIC).unwrap();
        
        let mut encoder = GzEncoder::new(file, Compression::default());
        encoder.write_all(content).unwrap();
        encoder.finish().unwrap();
        
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