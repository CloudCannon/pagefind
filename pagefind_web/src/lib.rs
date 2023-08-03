#![allow(clippy::not_unsafe_ptr_arg_deref)]

use std::collections::HashMap;

use util::*;
use wasm_bindgen::prelude::*;

mod filter;
mod filter_index;
mod index;
mod metadata;
mod search;
mod util;

pub struct PageWord {
    page: u32,
    locs: Vec<(u8, u32)>,
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
    if indexes.is_empty() && !query.trim().is_empty() {
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
                // Trim chunk boundaries and search terms to the shortest of either,
                // so that we load any chunk that may contain an extension or prefix of the search term
                let from_length = term.len().min(chunk.from.len());
                let to_length = term.len().min(chunk.to.len());

                term[0..from_length] >= chunk.from[0..from_length]
                    && term[0..to_length] <= chunk.to[0..to_length]
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
    let search_index = unsafe { Box::from_raw(ptr) };
    let mut indexes = search_index.filter_chunks(filters).unwrap_or_default();
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
    let (unfiltered_results, mut results) = if exact {
        search_index.exact_term(query, filter_set)
    } else {
        search_index.search_term(query, filter_set)
    };
    let unfiltered_total = unfiltered_results.len();
    debug!({ format!("Raw total of {} results", unfiltered_total) });
    debug!({ format!("Filtered total of {} results", query.len()) });

    let filter_string =
        search_index.get_filters(Some(results.iter().map(|r| r.page_index).collect()));
    let unfiltered_string = search_index.get_filters(Some(unfiltered_results));

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

    debug!({ "Building the result string" });
    let result_string = results
        .into_iter()
        .map(|result| {
            format!(
                "{}@{}@{}",
                &result.page,
                result.page_score,
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

    debug!({ "Boxing and returning the result" });
    let _ = Box::into_raw(search_index);

    #[cfg(debug_assertions)]
    debug_log(&format! {"{:?}", result_string});

    format!(
        "{}:{}:{}__PF_UNFILTERED_DELIM__{}",
        unfiltered_total, result_string, filter_string, unfiltered_string
    )
}
