use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct ServiceResponse {
    pub(super) message_id: u32,
    pub(super) payload: ResponseAction,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct SyntheticFileResponse {
    pub(super) path: String,
    pub(super) content: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub(super) enum ResponseAction {
    Error {
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
        bundle_path: String,
    },
    GetFiles {
        files: Vec<SyntheticFileResponse>,
    },
}
