use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct ServiceRequest {
    pub(super) message_id: u32,
    pub(super) payload: RequestAction,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub(super) enum RequestAction {
    NewIndex,
    AddFile {
        index_id: u32,
        file_path: String,
        file_contents: String,
    },
    WriteFiles {
        index_id: u32,
    },
}
