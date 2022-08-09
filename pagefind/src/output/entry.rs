use hashbrown::HashMap;

use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct PagefindEntryMeta {
    pub version: &'static str,
    pub languages: HashMap<String, String>,
}
