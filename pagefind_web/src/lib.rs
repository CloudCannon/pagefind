#![allow(clippy::not_unsafe_ptr_arg_deref)]

use std::collections::HashMap;

use excerpt::calculate_excerpt;
use util::*;
use wasm_bindgen::prelude::*;

mod excerpt;
mod filter;
mod filter_index;
mod index;
mod metadata;
mod search;
mod util;

pub struct PageWord {
    page: u32,
    locs: Vec<u32>,
}

pub struct IndexChunk {
    from: String,
    to: String,
    hash: String,
}

pub struct Page {
    hash: String,
    word_count: u32,
}

pub struct SearchIndex {
    web_version: &'static str,
    generator_version: Option<String>,
    pages: Vec<Page>,
    chunks: Vec<IndexChunk>,
    filter_chunks: HashMap<String, String>,
    words: HashMap<String, Vec<PageWord>>,
    filters: HashMap<String, HashMap<String, Vec<u32>>>,
    sorts: HashMap<String, Vec<u32>>,
}

#[cfg(debug_assertions)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[cfg(debug_assertions)]
fn debug_log(s: &str) {
    log(&format!("From WASM: {}", s));
}

#[wasm_bindgen]
pub fn init_pagefind(metadata_bytes: &[u8]) -> *mut SearchIndex {
    #[cfg(debug_assertions)]
    debug_log("Initializing Pagefind");
    let mut search_index = SearchIndex {
        web_version: env!("CARGO_PKG_VERSION"),
        generator_version: None,
        pages: Vec::new(),
        chunks: Vec::new(),
        filter_chunks: HashMap::new(),
        words: HashMap::new(),
        filters: HashMap::new(),
        sorts: HashMap::new(),
    };

    match search_index.decode_metadata(metadata_bytes) {
        Ok(_) => Box::into_raw(Box::new(search_index)),
        Err(e) => {
            #[cfg(debug_assertions)]
            debug_log(&format!("{:#?}", e));
            std::ptr::null_mut::<SearchIndex>()
        }
    }
}

#[wasm_bindgen]
pub fn load_index_chunk(ptr: *mut SearchIndex, chunk_bytes: &[u8]) -> *mut SearchIndex {
    debug!({ "Loading Index Chunk" });
    let mut search_index = unsafe { Box::from_raw(ptr) };

    match search_index.decode_index_chunk(chunk_bytes) {
        Ok(_) => Box::into_raw(search_index),
        Err(e) => {
            debug!({ format!("{:#?}", e) });
            std::ptr::null_mut::<SearchIndex>()
        }
    }
}

#[wasm_bindgen]
pub fn load_filter_chunk(ptr: *mut SearchIndex, chunk_bytes: &[u8]) -> *mut SearchIndex {
    debug!({ "Loading Filter Chunk" });
    let mut search_index = unsafe { Box::from_raw(ptr) };

    match search_index.decode_filter_index_chunk(chunk_bytes) {
        Ok(_) => Box::into_raw(search_index),
        Err(e) => {
            debug!({ format!("{:#?}", e) });
            std::ptr::null_mut::<SearchIndex>()
        }
    }
}

#[wasm_bindgen]
pub fn add_synthetic_filter(ptr: *mut SearchIndex, filter: &str) -> *mut SearchIndex {
    debug!({
        format! {"Creating a synthetic index filter for {:?}", filter}
    });

    let mut search_index = unsafe { Box::from_raw(ptr) };
    search_index.decode_synthetic_filter(filter);
    Box::into_raw(search_index)
}

#[wasm_bindgen]
pub fn request_indexes(ptr: *mut SearchIndex, query: &str) -> String {
    let indexes = try_request_indexes(ptr, query, false);
    if indexes.is_empty() {
        debug!({
            "No index chunks found with strict boundaries. Loading all possible extension chunks."
        });
        return try_request_indexes(ptr, query, true);
    }
    indexes
}

fn try_request_indexes(ptr: *mut SearchIndex, query: &str, load_all_possible: bool) -> String {
    debug!({
        format! {"Finding the index chunks needed for {:?}", query}
    });

    let search_index = unsafe { Box::from_raw(ptr) };
    let mut indexes = Vec::new();
    let terms = query.split(' ');

    for term in terms {
        let term_index = search_index.chunks.iter().find(|chunk| {
            if load_all_possible {
                // Trim chunk boundaries down to the length of the search term
                // so that we load any chunk that may contain an extension of the search term
                term >= &chunk.from.chars().take(term.len()).collect::<String>()
                    && term <= &chunk.to.chars().take(term.len()).collect::<String>()
            } else {
                term >= &chunk.from && term <= &chunk.to
            }
        });
        if let Some(index) = term_index {
            debug!({
                format! {"Need {:?} for {:?}", index.hash, term}
            });
            indexes.push(index.hash.clone())
        } else {
            debug!({
                format! {"No hash found for {:?}", term}
            })
        }
    }

    let _ = Box::into_raw(search_index);
    indexes.sort();
    indexes.dedup();
    indexes.join(" ")
}

#[wasm_bindgen]
pub fn request_filter_indexes(ptr: *mut SearchIndex, filters: &str) -> String {
    debug!({
        format! {"Finding the filter chunks needed for {:?}", filters}
    });

    let search_index = unsafe { Box::from_raw(ptr) };
    let mut indexes = Vec::new();
    let filters = filters.split("__PF_FILTER_DELIM__");

    for filter in filters {
        if let Some((filter, _)) = filter.split_once(':') {
            if let Some(hash) = search_index.filter_chunks.get(filter) {
                debug!({
                    format! {"Need {:?} for {:?}", hash, filter}
                });
                indexes.push(hash.clone());
            } else {
                debug!({
                    format! {"No hash found for {:?}", filter}
                })
            }
        }
    }

    let _ = Box::into_raw(search_index);
    indexes.sort();
    indexes.dedup();
    indexes.join(" ")
}

#[wasm_bindgen]
pub fn request_all_filter_indexes(ptr: *mut SearchIndex) -> String {
    debug!({ "Finding all filter chunks" });

    let search_index = unsafe { Box::from_raw(ptr) };
    let mut indexes: Vec<String> = search_index
        .filter_chunks
        .iter()
        .map(|(_, chunk)| chunk.into())
        .collect();

    let _ = Box::into_raw(search_index);
    indexes.sort();
    indexes.dedup();
    indexes.join(" ")
}

#[wasm_bindgen]
pub fn filters(ptr: *mut SearchIndex) -> String {
    debug!({ "Returning all loaded filters" });

    let search_index = unsafe { Box::from_raw(ptr) };

    let results = search_index.get_filters(None);

    let _ = Box::into_raw(search_index);
    results
}

#[wasm_bindgen]
pub fn search(ptr: *mut SearchIndex, query: &str, filter: &str, sort: &str, exact: bool) -> String {
    let search_index = unsafe { Box::from_raw(ptr) };

    if let Some(generator_version) = search_index.generator_version.as_ref() {
        if generator_version != search_index.web_version {
            // TODO: Return this as a warning alongside a search result if possible
            // let _ = Box::into_raw(search_index);
            // return "ERROR: Version mismatch".into();
        }
    }

    let filter_set = search_index.filter(filter);
    let mut results = if exact {
        search_index.exact_term(query, filter_set)
    } else {
        search_index.search_term(query, filter_set)
    };

    let filter_string =
        search_index.get_filters(Some(results.iter().map(|r| r.page_index).collect()));

    if let Some((sort, direction)) = sort.split_once(':') {
        debug!({ format!("Trying to sort by {sort} ({direction})") });
        if let Some(sorted_pages) = search_index.sorts.get(sort) {
            debug!({ format!("Found {} pages sorted by {sort}", sorted_pages.len()) });
            results.retain(|result| sorted_pages.contains(&(result.page_index as u32)));

            for result in results.iter_mut() {
                result.page_score = sorted_pages
                    .iter()
                    .position(|p| p == &(result.page_index as u32))
                    .expect("Sorted pages should contain all remaining results")
                    as f32;
                if direction == "asc" {
                    result.page_score = 0.0 - result.page_score;
                }
            }
        }
    }

    let result_string = results
        .into_iter()
        .map(|result| {
            format!(
                "{}@{}@{},{}@{}",
                &result.page,
                result.page_score,
                calculate_excerpt(&result.word_locations, 30),
                30,
                result
                    .word_locations
                    .iter()
                    .map(|l| l.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            )
        })
        .collect::<Vec<String>>()
        .join(" ");

    let _ = Box::into_raw(search_index);

    #[cfg(debug_assertions)]
    debug_log(&format! {"{:?}", result_string});

    format!("{}:{}", result_string, filter_string)
}
