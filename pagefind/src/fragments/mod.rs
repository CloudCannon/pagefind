use hashbrown::HashMap;

use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct PageAnchorData {
    pub element: String,
    pub id: String,
    pub text: Option<String>,
    pub location: u32,
}

#[derive(Serialize, Debug)]
pub struct PageFragmentData {
    pub url: String,
    pub content: String,
    pub word_count: usize,
    pub filters: HashMap<String, Vec<String>>,
    pub meta: HashMap<String, String>,
    pub anchors: Vec<PageAnchorData>,
}

#[derive(Debug)]
pub struct PageFragment {
    pub page_number: usize,
    pub data: PageFragmentData,
}
