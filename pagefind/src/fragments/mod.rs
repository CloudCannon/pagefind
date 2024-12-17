use std::collections::BTreeMap;

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct PageAnchorData {
    pub element: String,
    pub id: String,
    pub text: String,
    pub location: u32,
}

#[derive(Serialize, Debug, Clone)]
pub struct PageFragmentData {
    pub url: String,
    pub content: String,
    pub word_count: usize,
    pub filters: BTreeMap<String, Vec<String>>,
    pub meta: BTreeMap<String, String>,
    pub anchors: Vec<PageAnchorData>,
}

#[derive(Debug, Clone)]
pub struct PageFragment {
    pub page_number: usize,
    pub data: PageFragmentData,
}
