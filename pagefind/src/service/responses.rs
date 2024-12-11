use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct ServiceResponse {
    pub(super) message_id: Option<u32>,
    pub(super) payload: ResponseAction,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub(super) enum ResponseAction {
    Error {
        original_message: Option<String>,
        message: String,
    },
    NewIndex {
        index_id: u32,
    },
    IndexedFile {
        page_word_count: u32,
        page_url: String,
        page_meta: HashMap<String, String>,
    },
    IndexedDir {
        page_count: u32,
    },
    BuildIndex {},
    WriteFiles {
        output_path: String,
    },
    GetFiles {
        files: Vec<SyntheticFileResponse>,
    },
    DeleteIndex {},
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IndexedFileResponse {
    pub page_word_count: u32,
    pub page_url: String,
    pub page_meta: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SyntheticFileResponse {
    pub path: String,
    pub content: String,
}
