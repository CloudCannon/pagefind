use minicbor::Encode;

/// The pagefind.pf_meta file loaded on init

/// All metadata we need to glue together search queries & results
#[derive(Encode, Debug)]
pub struct MetaIndex {
    #[n(0)]
    pub version: String,
    #[n(1)]
    pub pages: Vec<MetaPage>,
    #[n(2)]
    pub index_chunks: Vec<MetaChunk>,
    #[n(3)]
    pub filters: Vec<MetaFilter>,
    #[n(4)]
    pub sorts: Vec<MetaSort>,
}

/// Communicates the pagefind/index/*.pf_index file we need to load
/// when searching for a word that sorts between `from` and `to`
#[derive(Encode, PartialEq, Debug)]
pub struct MetaChunk {
    #[n(0)]
    pub from: String,
    #[n(1)]
    pub to: String,
    #[n(2)]
    pub hash: String,
}

#[derive(Encode, Debug)]
pub struct MetaPage {
    #[n(0)]
    pub hash: String,
    #[n(1)]
    pub word_count: u32,
}

#[derive(Encode, Debug)]
pub struct MetaFilter {
    #[n(0)]
    pub filter: String,
    #[n(1)]
    pub hash: String,
}

#[derive(Encode, Debug)]
pub struct MetaSort {
    #[n(0)]
    pub sort: String,
    #[n(1)]
    pub pages: Vec<usize>,
}
