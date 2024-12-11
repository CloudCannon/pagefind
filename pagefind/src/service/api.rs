use hashbrown::HashMap;
use std::path::PathBuf;

use crate::{
    fossick::{parser::DomParserResult, Fossicker},
    PagefindInboundConfig, SearchOptions, SearchState,
};
use base64::{engine::general_purpose, Engine as _};

use super::{IndexedFileResponse, SyntheticFileResponse};

pub struct PagefindIndex {
    search_index: SearchState,
}

impl PagefindIndex {
    /// Create a new PagefindIndex instance.
    ///
    /// # Arguments
    /// * `config` - An optional PagefindServiceConfig to apply to the service.
    ///
    /// # Returns
    /// An optional PagefindIndex instance. If the search options are invalid, it
    /// will return None.
    pub fn new(config: PagefindInboundConfig) -> Option<Self> {
        match SearchOptions::load(config) {
            Ok(opts) => Some(Self {
                search_index: SearchState::new(opts),
            }),
            Err(_) => None,
        }
    }

    /// Add a file into this search index.
    /// Either a filepath or a URL must be provided.
    ///
    /// # Arguments
    /// * `file_path` - The path to the file to add.
    /// * `url` - The URL to the file to add.
    /// * `file_contents` - The contents of the file to add.
    ///
    /// # Returns
    /// Either the PageFragmentData of the file added or an error message, if it fails to add the
    /// file.
    pub async fn add_file(
        &mut self,
        file_path: Option<String>,
        url: Option<String>,
        file_contents: String,
    ) -> Result<IndexedFileResponse, String> {
        if file_path.is_none() && url.is_none() {
            return Err("Either file_path or url must be provided".into());
        }

        let file = Fossicker::new_synthetic(file_path.map(PathBuf::from), url, file_contents);
        let data = self.search_index.fossick_one(file).await;

        match data {
            Ok(data) => Ok(IndexedFileResponse {
                page_word_count: data.fragment.data.word_count as u32,
                page_url: data.fragment.data.url,
                page_meta: data.fragment.data.meta
            }),
            Err(_) => Err("Failed to add file".to_string()),
        }
    }

    /// Add a record to the search index.
    /// This is a more manual way to add a record to the search index, allowing for more control
    /// over the data. This is useful for adding records that are not files.
    ///
    /// # Arguments
    /// * `url` - The URL of the record.
    /// * `content` - The content of the record.
    /// * `language` - The language of the record.
    /// * `meta` - Optional metadata to add to the record.
    /// * `filters` - Optional filters to apply to the record.
    /// * `sort` - Optional sorting to apply to the record.
    pub async fn add_record(
        &mut self,
        url: String,
        content: String,
        language: String,
        meta: Option<HashMap<String, String>>,
        filters: Option<HashMap<String, Vec<String>>>,
        sort: Option<HashMap<String, String>>,
    ) -> Result<IndexedFileResponse, String> {
        let data = DomParserResult {
            digest: content,
            filters: filters.unwrap_or_default(),
            sort: sort.unwrap_or_default(),
            meta: meta.unwrap_or_default(),
            anchor_content: HashMap::new(),
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
        let data = self.search_index.fossick_one(file).await;

        match data {
            Ok(data) => Ok(IndexedFileResponse {
                page_word_count: data.fragment.data.word_count as u32,
                page_url: data.fragment.data.url,
                page_meta: data.fragment.data.meta
            }),
            Err(_) => Err("Failed to add file".to_string()),
        }
    }

    /// Add a directory to the search index with a glob pattern.
    ///
    /// # Arguments
    /// * `path` - The path to the directory to index.
    /// * `glob` - A glob pattern to match files in the directory. If not provided, the default glob pattern will be used.
    ///
    /// # Returns
    /// Either the number of pages indexed or an error message, if it fails to index the directory.
    pub async fn add_dir(&mut self, path: String, glob: Option<String>) -> Result<usize, String> {
        let defaults: PagefindInboundConfig =
            serde_json::from_str("{}").expect("All fields have serde defaults");
        let glob = glob.unwrap_or(defaults.glob);

        let data = self
            .search_index
            .fossick_many(PathBuf::from(path), glob)
            .await;
        match data {
            Ok(page_count) => Ok(page_count),
            Err(_) => Err("Failed to index directory".to_string()),
        }
    }

    /// Build the search index for this instance and hold it in memory.
    pub async fn build_indexes(&mut self) {
        self.search_index.build_indexes().await;
    }

    /// Build the search index for this instance and write the files to disk.
    ///
    /// # Arguments
    /// * `output_path` - The path to write the files to. If not provided, the default output path will be used.
    pub async fn write_files(&mut self, output_path: Option<String>) -> String {
        self.search_index.build_indexes().await;
        let resolved_output_path = self
            .search_index
            .write_files(output_path.map(Into::into))
            .await;

        resolved_output_path.to_string_lossy().into()
    }

    /// Build the search index for this instance and return the files as a list of
    /// SyntheticFileResponse.
    ///
    /// # Returns
    /// A list of SyntheticFileResponse containing the path and content of each file.
    pub async fn get_files(&mut self) -> Vec<SyntheticFileResponse> {
        self.search_index.build_indexes().await;
        self.search_index
            .get_files()
            .await
            .into_iter()
            .map(|file| SyntheticFileResponse {
                path: file.filename.to_string_lossy().into(),
                content: general_purpose::STANDARD.encode(file.contents),
            })
            .collect()
    }
}
