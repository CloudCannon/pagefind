use hashbrown::HashMap;

use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct PagefindEntryMeta {
    pub version: &'static str,
    pub languages: HashMap<String, PagefindEntryLanguage>,
}

#[derive(Serialize, Debug)]
pub struct PagefindEntryLanguage {
    pub hash: String,
    pub wasm: Option<String>,
    pub page_count: usize,
}
