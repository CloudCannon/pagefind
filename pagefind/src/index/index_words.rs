use minicbor::Encode;

/// The word index chunks in `pagefind/index/`

/// A single word index chunk: `pagefind/index/*.pf_index`
#[derive(Encode)]
pub struct WordIndex {
    #[n(0)]
    pub words: Vec<PackedWord>,
}

/// A single word as an inverse index of all locations on the site
#[derive(Encode, Clone, Debug)]
pub struct PackedWord {
    #[n(0)]
    pub word: String,
    #[n(1)]
    pub pages: Vec<PackedPage>,
}

/// A set of locations on a given page
#[derive(Encode, Clone, Debug)]
pub struct PackedPage {
    #[n(0)]
    pub page_number: usize, // Won't exceed u32 but saves us some into()s
    #[n(1)]
    pub locs: Vec<i32>,
}
