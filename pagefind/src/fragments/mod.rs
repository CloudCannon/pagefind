use hashbrown::HashMap;

use serde::Serialize;

#[derive(Serialize)]
pub struct PageFragmentData {
    pub url: String,
    pub title: String,
    pub content: String,
    pub attributes: HashMap<String, String>,
}

pub struct PageFragment {
    pub hash: String,
    pub page_number: usize,
    pub data: PageFragmentData,
}
