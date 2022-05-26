use minicbor::Encode;

/// The filter index chunks in `_pagefind/filter/`

/// A single filter index chunk: `_pagefind/filter/*.pf_filter`
#[derive(Encode)]
pub struct FilterIndex {
    #[n(0)]
    pub filter: String,
    #[n(1)]
    pub values: Vec<PackedValue>,
}

/// A single filter value as an inverse index of all locations on the site
#[derive(Encode, Clone, Debug)]
pub struct PackedValue {
    #[n(0)]
    pub value: String,
    #[n(1)]
    pub pages: Vec<usize>, // Won't exceed u32 but saves us some into()s
}
