//! The programmatic service interface for integrating Pagefind into another Rust project.
//!
//! # Examples
//!
//! ```
//! use pagefind::api::{PagefindIndex};
//! use pagefind::options::{PagefindServiceConfig};
//!
//! #[tokio::main]
//! async fn main() {
//!     let options = PagefindServiceConfig::builder()
//!         .keep_index_url(true)
//!         .force_language("en".to_string())
//!         .build();
//!     let mut index = PagefindIndex::new(Some(options)).expect("Options should be valid");
//!     let indexing_response = index
//!         .add_html_file(
//!             Some("test/index.html".into()),
//!             None,
//!             "<html><body><h1>Test content</h1></body></html>".into(),
//!         )
//!         .await;
//!
//!     if let Ok(file) = indexing_response {
//!         println!("Page word count: {}", file.page_word_count);
//!         println!("Page URL: {}", file.page_url);
//!     }
//!
//!     let files_response = index.get_files().await;
//!     if let Ok(files) = files_response {
//!         println!("Have {} files to write to disk", files.len());
//!     }
//! }
//! ```

pub use crate::output::SyntheticFile;
use anyhow::{bail, Result};
use rust_patch::Patch;
use std::{collections::BTreeMap, path::PathBuf};

use crate::{
    fossick::{parser::DomParserResult, Fossicker},
    options::PagefindServiceConfig,
    PagefindInboundConfig, SearchOptions, SearchState, IndexCatalogueRepresentation,
};

#[derive(Debug)]
pub struct IndexedFileResponse {
    pub page_word_count: u32,
    pub page_url: String,
    pub page_meta: BTreeMap<String, String>,
}

pub struct PagefindIndex {
    search_index: SearchState,
}

impl PagefindIndex {
    /// Create a new PagefindIndex instance that files can be added to.
    ///
    /// # Arguments
    /// * `config` - An optional PagefindServiceConfig to override default options for the service.
    ///
    /// # Returns
    /// A PagefindIndex instance if search options are valid, otherwise an Error.
    pub fn new(config: Option<PagefindServiceConfig>) -> Result<Self> {
        let mut service_options: PagefindInboundConfig =
            serde_json::from_str("{}").expect("All fields have serde defaults");

        service_options.service = true;
        if let Some(config) = config {
            service_options = config.apply(service_options);
        }

        let options = SearchOptions::load(service_options)?;
        Ok(Self {
            search_index: SearchState::new(options),
        })
    }

    /// Add an HTML file that isn't on disk into this search index.
    /// Either a filepath or a URL must be provided.
    ///
    /// # Arguments
    /// * `source_path` - The source path of the HTML file if it were to exist on disk.
    /// * `url` - An explicit URL to use, instead of having Pagefind compute the URL based on the `source_path`.
    /// * `content` - The source HTML content of the file to be parsed.
    ///
    /// # Returns
    /// Metadata about the added file if successful, otherwise an Error.
    pub async fn add_html_file(
        &mut self,
        source_path: Option<String>,
        url: Option<String>,
        content: String,
    ) -> Result<IndexedFileResponse> {
        if source_path.is_none() && url.is_none() {
            bail!("Either source_path or url must be provided");
        }

        let file = Fossicker::new_synthetic(source_path.map(PathBuf::from), url, content);
        let data = self.search_index.fossick_one(file).await?;

        Ok(IndexedFileResponse {
            page_word_count: data.fragment.data.word_count as u32,
            page_url: data.fragment.data.url,
            page_meta: data.fragment.data.meta,
        })
    }

    /// Index a custom record that isn't backed by an HTML file.
    ///
    /// # Arguments
    /// * `url` - The output URL of this record. Pagefind will not alter this.
    /// * `content` - The raw content of this record.
    /// * `language` - What language is this record written in. Expects an ISO 639-1 code.
    /// * `meta` - The metadata to attach to this record. Supplying a `title` is highly recommended.
    /// * `filters` - The filters to attach to this record.
    /// * `sort` - The sort keys to attach to this record.
    ///
    /// # Returns
    /// Metadata about the added record if successful, otherwise an Error.
    pub async fn add_custom_record(
        &mut self,
        url: String,
        content: String,
        language: String,
        meta: Option<BTreeMap<String, String>>,
        filters: Option<BTreeMap<String, Vec<String>>>,
        sort: Option<BTreeMap<String, String>>,
    ) -> Result<IndexedFileResponse> {
        let data = DomParserResult {
            digest: content,
            filters: filters.unwrap_or_default(),
            sort: sort.unwrap_or_default(),
            meta: meta.unwrap_or_default(),
            anchor_content: BTreeMap::new(),
            has_custom_body: false,
            force_inclusion: true,
            has_html_element: true,
            has_old_bundle_reference: false,
            language: self
                .search_index
                .options
                .force_language
                .clone()
                .unwrap_or(language),
        };
        let file = Fossicker::new_with_data(url, data);
        let data = self.search_index.fossick_one(file).await?;

        Ok(IndexedFileResponse {
            page_word_count: data.fragment.data.word_count as u32,
            page_url: data.fragment.data.url,
            page_meta: data.fragment.data.meta,
        })
    }

    /// Index a directory of HTML files from disk.
    ///
    /// # Arguments
    /// * `path` - The path to the directory to index.
    /// * `glob` - A glob pattern to match files in the directory. If not provided, the default glob pattern will be used.
    ///
    /// # Returns
    /// The number of pages indexed if successful, otherwise an Error.
    pub async fn add_directory(&mut self, path: String, glob: Option<String>) -> Result<usize> {
        let defaults: PagefindInboundConfig =
            serde_json::from_str("{}").expect("All fields have serde defaults");
        let glob = glob.unwrap_or(defaults.glob);

        let page_count = self
            .search_index
            .fossick_many(PathBuf::from(path), glob)
            .await?;

        Ok(page_count)
    }

    /// Build the search index for this instance and hold it in memory.
    pub async fn build_indexes(&mut self) -> Result<()> {
        self.search_index.build_indexes().await
    }

    /// Build the search index for this instance and write the files to disk.
    ///
    /// # Arguments
    /// * `output_path` - The path to write the files to. If not provided, the default output path will be used.
    ///
    /// # Returns
    /// The path files were written to if successful, otherwise an Error.
    pub async fn write_files(&mut self, output_path: Option<String>) -> Result<String> {
        self.search_index.build_indexes().await?;
        let resolved_output_path = self
            .search_index
            .write_files(output_path.map(Into::into))
            .await;

        Ok(resolved_output_path.to_string_lossy().into())
    }

    /// Build the search index for this instance and return the files as a list of
    /// SyntheticFileResponse.
    ///
    /// # Returns
    /// A list of SyntheticFiles containing the path and content of each file.
    pub async fn get_files(&mut self) -> Result<Vec<SyntheticFile>> {
        self.search_index.build_indexes().await?;
        Ok(self.search_index.get_files().await)
    }

    /// Get the catalogue mappings from hashes to encoded data.
    ///
    /// # Returns
    /// An IndexCatalogueRepresentation containing the hash and content of each fragment.
    pub async fn get_index_catalogue(&mut self) -> Result<IndexCatalogueRepresentation> {
        self.search_index.get_index_catalogue().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_file() {
        let options = PagefindServiceConfig::builder()
            .keep_index_url(true)
            .force_language("en".to_string())
            .build();
        let mut index = PagefindIndex::new(Some(options)).unwrap();
        let file_response = index
            .add_html_file(
                Some("test/index.html".into()),
                None,
                "<html><body><h1>Test content</h1></body></html>".into(),
            )
            .await;

        let file = file_response.expect("Adding a file should succeed");
        assert_eq!(file.page_word_count, 2);
        assert_eq!(file.page_url, "/test/index.html");

        let files_response = index.get_files().await;

        let files = files_response.expect("Getting files should succeed");
        let filenames: Vec<_> = files.into_iter().map(|f| f.filename).collect();
        assert!(filenames.contains(&PathBuf::from("pagefind.js")));
        assert!(filenames.contains(&PathBuf::from("pagefind-ui.js")));
        assert!(filenames.contains(&PathBuf::from("pagefind-ui.css")));
        assert!(filenames.contains(&PathBuf::from("wasm.en.pagefind")));
        assert!(filenames.contains(&PathBuf::from("pagefind-entry.json")));
        assert!(filenames
            .iter()
            .any(|f| f.to_string_lossy().ends_with(".pf_meta")));
        assert!(filenames
            .iter()
            .any(|f| f.to_string_lossy().ends_with(".pf_fragment")));
        assert!(filenames
            .iter()
            .any(|f| f.to_string_lossy().ends_with(".pf_index")));
    }
}
