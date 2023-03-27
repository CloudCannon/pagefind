use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct ServiceResponse {
    pub(super) message_id: u32,
    pub(super) payload: ResponseAction,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub(super) enum ResponseAction {
    Error {
        message: String,
    },
    CreatedIndex {
        index_id: u32,
    },
    AddedFile {
        page_word_count: u32,
        page_url: String,
        page_meta: HashMap<String, String>,
    },
}
