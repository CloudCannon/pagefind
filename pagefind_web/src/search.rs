use std::{borrow::Cow, cmp::Ordering};

use crate::{util::*, PageWord};
use bit_set::BitSet;
use pagefind_stem::Stemmer;

use crate::SearchIndex;

pub struct PageSearchResult {
    pub page: String,
    pub page_index: usize,
    pub page_score: f32, // TODO: tf-idf implementation? Paired with the dictionary-in-meta approach
    pub word_locations: Vec<u32>,
}

impl SearchIndex {
    pub fn exact_term(&self, term: &str, filter_results: Option<BitSet>) -> Vec<PageSearchResult> {
        debug!({
            format! {"Searching {:?}", term}
        });

        let mut maps = Vec::new();
        let mut words = Vec::new();
        for term in stems_from_term(term) {
            if let Some(word_index) = self.words.get(term.as_ref()) {
                words.extend(word_index);
                let mut set = BitSet::new();
                for page in word_index {
                    set.insert(page.page as usize);
                }
                maps.push(set);
            } else {
                // If we can't find this word, there are obviously no exact matches
                return vec![];
            }
        }

        if let Some(filter) = filter_results {
            maps.push(filter);
        }

        let results = match intersect_maps(maps) {
            Some(map) => map,
            None => return vec![],
        };

        let mut pages: Vec<PageSearchResult> = vec![];

        for page_index in results.iter() {
            let word_locations: Vec<Vec<u32>> = words
                .iter()
                .filter_map(|p| {
                    if p.page as usize == page_index {
                        Some(p.locs.clone())
                    } else {
                        None
                    }
                })
                .collect();
            debug!({
                format! {"Word locations {:?}", word_locations}
            });

            if word_locations.len() > 1 {
                'indexes: for pos in &word_locations[0] {
                    let mut i = *pos;
                    for subsequent in &word_locations[1..] {
                        i += 1;
                        // Test each subsequent word map to try and find a contiguous block
                        if !subsequent.contains(&i) {
                            continue 'indexes;
                        }
                    }
                    let page = &self.pages[page_index];
                    let search_result = PageSearchResult {
                        page: page.hash.clone(),
                        page_index,
                        page_score: 1.0,
                        word_locations: (*pos..=i).collect(),
                    };
                    pages.push(search_result);
                    break 'indexes;
                }
            } else {
                let page = &self.pages[page_index];
                let search_result = PageSearchResult {
                    page: page.hash.clone(),
                    page_index,
                    page_score: 1.0,
                    word_locations: word_locations[0].clone(),
                };
                pages.push(search_result);
            }
        }

        pages
    }

    pub fn search_term(&self, term: &str, filter_results: Option<BitSet>) -> Vec<PageSearchResult> {
        debug!({
            format! {"Searching {:?}", term}
        });

        let mut maps = Vec::new();
        let mut unique_maps = Vec::new();
        let mut words = Vec::new();
        for term in stems_from_term(term) {
            let mut word_maps = Vec::new();
            for (word, word_index) in self.find_word_extensions(&term) {
                words.extend(word_index);
                let mut set = BitSet::new();
                for page in word_index {
                    set.insert(page.page as usize);
                }
                unique_maps.push((word.len() - term.len() + 1, set.clone()));
                word_maps.push(set);
            }
            if let Some(result) = union_maps(word_maps) {
                maps.push(result);
            }
        }

        if let Some(filter) = filter_results {
            maps.push(filter);
        }

        let results = match intersect_maps(maps) {
            Some(map) => map,
            None => return vec![],
        };

        let mut pages: Vec<PageSearchResult> = vec![];

        for page_index in results.iter() {
            let mut word_locations: Vec<u32> = words
                .iter()
                .filter_map(|p| {
                    if p.page as usize == page_index {
                        Some(p.locs.clone())
                    } else {
                        None
                    }
                })
                .flatten()
                .collect();
            debug!({
                format! {"Word locations {:?}", word_locations}
            });
            word_locations.sort_unstable();
            debug!({
                format! {"Word locations {:?}", word_locations}
            });

            let page = &self.pages[page_index];
            let mut page_score = word_locations.len() as f32 / page.word_count as f32;
            for (len, map) in unique_maps.iter() {
                // Boost pages that match shorter words, as they are closer
                // to the term that was searched. Combine the weight with
                // a word frequency to boost high quality results.
                if map.contains(page_index) {
                    page_score += 1.0 / *len as f32;
                    debug!({
                        format! {"{} contains a word {} longer than the search term, boosting by {} to {}", page.hash, len, 1.0 / *len as f32, page_score}
                    });
                }
            }
            let search_result = PageSearchResult {
                page: page.hash.clone(),
                page_index,
                page_score,
                word_locations,
            };

            debug!({
                format! {"Page {} has {} matching terms (in {} total words), and has the boosted word frequency of {:?}", search_result.page, search_result.word_locations.len(), page.word_count, search_result.page_score}
            });

            pages.push(search_result);
        }

        debug!({ "Sorting by word frequency" });
        pages.sort_unstable_by(|a, b| {
            b.page_score
                .partial_cmp(&a.page_score)
                .unwrap_or(Ordering::Equal)
        });

        pages
    }

    fn find_word_extensions(&self, term: &str) -> Vec<(&String, &Vec<PageWord>)> {
        let mut extensions = vec![];
        for (key, results) in self.words.iter() {
            if key.starts_with(term) {
                debug!({
                    format! {"Adding {:#?} to the query", key}
                });
                extensions.push((key, results));
            }
        }
        extensions
    }
}

fn stems_from_term(term: &str) -> Vec<Cow<str>> {
    let stemmer = Stemmer::try_create_default();
    term.split(' ')
        .map(|word| match &stemmer {
            Ok(stemmer) => stemmer.stem(word),
            // If we wound up without a stemmer,
            // charge ahead without stemming.
            Err(_) => word.into(),
        })
        .collect()
}

fn intersect_maps(mut maps: Vec<BitSet>) -> Option<BitSet> {
    let mut maps = maps.drain(..);
    if let Some(mut base) = maps.next() {
        for map in maps {
            base.intersect_with(&map);
        }
        Some(base)
    } else {
        None
    }
}

fn union_maps(mut maps: Vec<BitSet>) -> Option<BitSet> {
    let mut maps = maps.drain(..);
    if let Some(mut base) = maps.next() {
        for map in maps {
            base.union_with(&map);
        }
        Some(base)
    } else {
        None
    }
}
