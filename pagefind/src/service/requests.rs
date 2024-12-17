use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::options::PagefindServiceConfig;

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct ServiceRequest {
    pub(super) message_id: u32,
    pub(super) payload: RequestAction,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub(super) enum RequestAction {
    NewIndex {
        config: Option<PagefindServiceConfig>,
    },
    AddFile {
        index_id: u32,
        file_path: Option<String>,
        url: Option<String>,
        file_contents: String,
    },
    AddRecord {
        index_id: u32,
        url: String,
        content: String,
        language: String,
        meta: Option<BTreeMap<String, String>>,
        filters: Option<BTreeMap<String, Vec<String>>>,
        sort: Option<BTreeMap<String, String>>,
    },
    AddDir {
        index_id: u32,
        path: String,
        glob: Option<String>,
    },
    BuildIndex {
        index_id: u32,
    },
    WriteFiles {
        index_id: u32,
        output_path: Option<String>,
    },
    GetFiles {
        index_id: u32,
    },
    DeleteIndex {
        index_id: u32,
    },
}
