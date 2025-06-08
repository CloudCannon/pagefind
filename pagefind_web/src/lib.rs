#![allow(clippy::not_unsafe_ptr_arg_deref)]

use pagefind_core_search::{CoreSearchIndex, PageSearchResult, RankingWeights};
use wasm_bindgen::prelude::*;
use bit_set::BitSet;

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

// Re-export the core search index as SearchIndex for compatibility
type SearchIndex = CoreSearchIndex;

#[wasm_bindgen]
pub fn init_pagefind(metadata_bytes: &[u8]) -> *mut SearchIndex {
    #[cfg(debug_assertions)]
    debug_log("Initializing Pagefind");
    
    let mut search_index = SearchIndex::new();
    
    match search_index.decode_metadata(metadata_bytes) {
        Ok(_) => Box::into_raw(Box::new(search_index)),
        #[allow(unused_variables)]
        Err(e) => {
            #[cfg(debug_assertions)]
            debug_log(&format!("{:#?}", e));
            std::ptr::null_mut::<SearchIndex>()
        }
    }
}

#[wasm_bindgen]
pub fn enter_playground_mode(ptr: *mut SearchIndex) -> *mut SearchIndex {
    #[cfg(debug_assertions)]
    debug_log("Entering Pagefind Playground Mode");
    
    // Playground mode is now handled at the search level
    ptr
}

#[wasm_bindgen]
pub fn set_ranking_weights(ptr: *mut SearchIndex, weights: &str) -> *mut SearchIndex {
    #[cfg(debug_assertions)]
    debug_log("Loading Ranking Weights");
    
    let Ok(weights) = pagefind_microjson::JSONValue::parse(weights) else {
        return ptr;
    };
    
    let mut search_index = unsafe { Box::from_raw(ptr) };
    
    if let Ok(term_similarity) = weights
        .get_key_value("term_similarity")
        .and_then(|v| v.read_float())
    {
        search_index.ranking_weights.term_similarity = term_similarity.max(0.0);
    }
    
    if let Ok(page_length) = weights
        .get_key_value("page_length")
        .and_then(|v| v.read_float())
    {
        search_index.ranking_weights.page_length = page_length.clamp(0.0, 1.0);
    }
    
    if let Ok(term_saturation) = weights
        .get_key_value("term_saturation")
        .and_then(|v| v.read_float())
    {
        search_index.ranking_weights.term_saturation = term_saturation.clamp(0.0, 2.0);
    }
    
    if let Ok(term_frequency) = weights
        .get_key_value("term_frequency")
        .and_then(|v| v.read_float())
    {
        search_index.ranking_weights.term_frequency = term_frequency.clamp(0.0, 1.0);
    }
    
    Box::into_raw(search_index)
}

#[wasm_bindgen]
pub fn load_index_chunk(ptr: *mut SearchIndex, chunk_bytes: &[u8]) -> *mut SearchIndex {
    #[cfg(debug_assertions)]
    debug_log("Loading Index Chunk");
    
    let mut search_index = unsafe { Box::from_raw(ptr) };
    
    match search_index.decode_index_chunk(chunk_bytes) {
        Ok(_) => Box::into_raw(search_index),
        #[allow(unused_variables)]
        Err(e) => {
            #[cfg(debug_assertions)]
            debug_log(&format!("{:#?}", e));
            std::ptr::null_mut::<SearchIndex>()
        }
    }
}

#[wasm_bindgen]
pub fn load_filter_chunk(ptr: *mut SearchIndex, chunk_bytes: &[u8]) -> *mut SearchIndex {
    #[cfg(debug_assertions)]
    debug_log("Loading Filter Chunk");
    
    let mut search_index = unsafe { Box::from_raw(ptr) };
    
    match search_index.decode_filter_index_chunk(chunk_bytes) {
        Ok(_) => Box::into_raw(search_index),
        #[allow(unused_variables)]
        Err(e) => {
            #[cfg(debug_assertions)]
            debug_log(&format!("{:#?}", e));
            std::ptr::null_mut::<SearchIndex>()
        }
    }
}

#[wasm_bindgen]
pub fn add_synthetic_filter(ptr: *mut SearchIndex, filter: &str) -> *mut SearchIndex {
    #[cfg(debug_assertions)]
    debug_log(&format!("Creating a synthetic index filter for {:?}", filter));
    
    let mut search_index = unsafe { Box::from_raw(ptr) };
    search_index.decode_synthetic_filter(filter);
    Box::into_raw(search_index)
}

#[wasm_bindgen]
pub fn request_indexes(ptr: *mut SearchIndex, query: &str) -> String {
    let mut indexes = try_request_indexes(ptr, query, false);
    if indexes.is_empty() && !query.trim().is_empty() {
        #[cfg(debug_assertions)]
        debug_log("No index chunks found with strict boundaries. Loading all possible extension chunks.");
        indexes = try_request_indexes(ptr, query, true);
    }
    
    let mut output = String::new();
    {
        let mut arr = write_json::array(&mut output);
        indexes.into_iter().for_each(|i| {
            arr.string(&i);
        });
    }
    
    output
}

fn try_request_indexes(ptr: *mut SearchIndex, query: &str, load_all_possible: bool) -> Vec<String> {
    #[cfg(debug_assertions)]
    debug_log(&format!("Finding the index chunks needed for {:?}", query));
    
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
                
                let (Some(term_pre), Some(chunk_pre)) =
                    (term.get(0..from_length), chunk.from.get(0..from_length))
                else {
                    return false;
                };
                
                let (Some(term_post), Some(chunk_post)) =
                    (term.get(0..to_length), chunk.to.get(0..to_length))
                else {
                    return false;
                };
                
                term_pre >= chunk_pre && term_post <= chunk_post
            } else {
                term >= &chunk.from && term <= &chunk.to
            }
        });
        if let Some(index) = term_index {
            #[cfg(debug_assertions)]
            debug_log(&format!("Need {:?} for {:?}", index.hash, term));
            indexes.push(index.hash.clone())
        } else {
            #[cfg(debug_assertions)]
            debug_log(&format!("No hash found for {:?}", term))
        }
    }
    
    let _ = Box::into_raw(search_index);
    indexes.sort();
    indexes.dedup();
    
    indexes
}

#[wasm_bindgen]
pub fn request_filter_indexes(ptr: *mut SearchIndex, filters: &str) -> String {
    let search_index = unsafe { Box::from_raw(ptr) };
    let mut indexes = search_index.filter_chunks(filters).unwrap_or_default();
    let _ = Box::into_raw(search_index);
    indexes.sort();
    indexes.dedup();
    let mut output = String::new();
    {
        let mut arr = write_json::array(&mut output);
        indexes.into_iter().for_each(|i| {
            arr.string(&i);
        });
    }
    
    output
}

#[wasm_bindgen]
pub fn request_all_filter_indexes(ptr: *mut SearchIndex) -> String {
    #[cfg(debug_assertions)]
    debug_log("Finding all filter chunks");
    
    let search_index = unsafe { Box::from_raw(ptr) };
    let mut indexes: Vec<String> = search_index
        .filter_chunks
        .iter()
        .map(|(_, chunk)| chunk.into())
        .collect();
    
    let _ = Box::into_raw(search_index);
    indexes.sort();
    indexes.dedup();
    let mut output = String::new();
    {
        let mut arr = write_json::array(&mut output);
        indexes.into_iter().for_each(|i| {
            arr.string(&i);
        });
    }
    
    output
}

#[wasm_bindgen]
pub fn filters(ptr: *mut SearchIndex) -> String {
    #[cfg(debug_assertions)]
    debug_log("Returning all loaded filters");
    
    let search_index = unsafe { Box::from_raw(ptr) };
    
    let mut output = String::new();
    {
        let mut obj = write_json::object(&mut output);
        search_index.get_filters(&mut obj, None);
    }
    
    let _ = Box::into_raw(search_index);
    output
}

#[wasm_bindgen]
pub fn search(ptr: *mut SearchIndex, query: &str, filter: &str, sort: &str, exact: bool) -> String {
    let search_index = unsafe { Box::from_raw(ptr) };
    let mut output = String::new();
    {
        let mut output_obj = write_json::object(&mut output);
        
        let filter_set = search_index.filter(filter);
        let playground_mode = false; // This can be tracked separately if needed
        let (unfiltered_results, mut results) = if exact {
            search_index.exact_term(query, filter_set, playground_mode)
        } else {
            search_index.search_term(query, filter_set, playground_mode)
        };
        let unfiltered_total = unfiltered_results.len();
        
        #[cfg(debug_assertions)]
        debug_log(&format!("Raw total of {} results", unfiltered_total));
        #[cfg(debug_assertions)]
        debug_log(&format!("Filtered total of {} results", results.len()));
        
        {
            let mut filter_obj = output_obj.object("filtered_counts");
            search_index.get_filters(
                &mut filter_obj,
                Some(results.iter().map(|r| r.page_index).collect()),
            );
        }
        {
            let mut unfilter_obj = output_obj.object("total_counts");
            search_index.get_filters(&mut unfilter_obj, Some(unfiltered_results));
        }
        
        // Apply sorting if requested
        search_index.apply_sort(&mut results, sort);
        
        {
            #[cfg(debug_assertions)]
            debug_log("Building the result string");
            let mut arr = output_obj.array("results");
            
            for result in results {
                let mut page_obj = arr.object();
                page_obj
                    .string("p", &result.page)
                    .number("s", result.page_score as f64);
                if playground_mode {
                    let mut params_obj = page_obj.object("params");
                    
                    params_obj
                        .number("tp", search_index.pages.len() as f64)
                        .number("apl", search_index.average_page_length as f64)
                        .number("dl", result.page_length as f64);
                }
                if let Some(verbose_scores) = result.verbose_scores {
                    let mut score_arr = page_obj.array("scores");
                    for (
                        word,
                        scoring_metrics,
                        params,
                    ) in verbose_scores
                    {
                        let mut score_obj = score_arr.object();
                        
                        score_obj
                            .string("w", &word)
                            .number("idf", scoring_metrics.idf as f64)
                            .number("b_tf", scoring_metrics.bm25_tf as f64)
                            .number("r_tf", scoring_metrics.raw_tf as f64)
                            .number("p_tf", scoring_metrics.pagefind_tf as f64)
                            .number("s", scoring_metrics.score as f64);
                        
                        {
                            let mut params_obj = score_obj.object("params");
                            
                            params_obj
                                .number("w_tf", params.weighted_term_frequency as f64)
                                .number("pct", params.pages_containing_term as f64)
                                .number("lb", params.length_bonus as f64);
                        }
                    }
                }
                {
                    let mut locs_arr = page_obj.array("l");
                    
                    for word_score in result.word_locations
                    {
                        let mut locs_obj = locs_arr.object();
                        locs_obj
                            .number("w", word_score.weight as f64)
                            .number("s", word_score.balanced_score as f64)
                            .number("l", word_score.word_location as f64);
                        if let Some(verbose_word_info) = word_score.verbose_word_info {
                            locs_obj
                                .object("v")
                                .string("ws", &verbose_word_info.word)
                                .number("lb", verbose_word_info.length_bonus as f64);
                        }
                    }
                }
            }
        }
        
        output_obj.number("unfiltered_total", unfiltered_total as f64);
    }
    
    let _ = Box::into_raw(search_index);
    output
}
