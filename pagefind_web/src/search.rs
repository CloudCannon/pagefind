use std::{cmp::Ordering, collections::HashMap};

use crate::{util::*, PageWord};
use bit_set::BitSet;
use rust_stemmers::{Algorithm, Stemmer}; // TODO: too big, Stemming should be performed on the JS side

use crate::SearchIndex;

pub struct PageSearchResult {
    pub page: String,
    pub page_index: usize,
    pub word_frequency: f32, // TODO: tf-idf implementation? Paired with the dictionary-in-meta approach
    pub word_locations: Vec<u32>,
}

impl SearchIndex {
    pub fn search_term(&self, term: &str, filter_results: Option<BitSet>) -> Vec<PageSearchResult> {
        let terms = term.split(' ');
        // TODO: i18n
        // TODO: Stemming should be performed on the JS side of the boundary
        //       As the snowball implementation there seems a lot smaller and just as fast.
        let en_stemmer = Stemmer::create(Algorithm::English);

        debug!({
            format! {"Searching {:?}", term}
        });

        let mut maps = Vec::new();
        let mut unique_maps = Vec::new();
        let mut words = Vec::new();
        for term in terms {
            let term = en_stemmer.stem(term).into_owned(); // TODO: Remove this once JS stems

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
            let mut word_maps = word_maps.drain(..);
            if let Some(mut base) = word_maps.next() {
                for map in word_maps {
                    base.union_with(&map);
                }
                maps.push(base)
            }
        }

        let mut maps = maps.drain(..);
        let mut results = if let Some(map) = maps.next() {
            map
        } else {
            return vec![];
            // let _ = Box::into_raw(search_index);
            // return "".into();
        };

        for map in maps {
            results.intersect_with(&map);
        }

        if let Some(filter) = filter_results {
            results.intersect_with(&filter);
        }

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
            let mut word_frequency = word_locations.len() as f32 / page.word_count as f32;
            for (len, map) in unique_maps.iter() {
                // Boost pages that match shorter words, as they are closer
                // to the term that was searched. Combine the weight with
                // a word frequency to boost high quality results.
                if map.contains(page_index) {
                    word_frequency += 1.0 / *len as f32;
                    debug!({
                        format! {"{} contains a word {} longer than the search term, boosting by {} to {}", page.hash, len, 1.0 / *len as f32, word_frequency}
                    });
                }
            }
            let search_result = PageSearchResult {
                page: page.hash.clone(),
                page_index,
                word_frequency,
                word_locations,
            };

            debug!({
                format! {"Page {} has {} matching terms (in {} total words), and has the boosted word frequency of {:?}", search_result.page, search_result.word_locations.len(), page.word_count, search_result.word_frequency}
            });

            pages.push(search_result);
        }

        debug!({ "Sorting by word frequency" });
        pages.sort_unstable_by(|a, b| {
            b.word_frequency
                .partial_cmp(&a.word_frequency)
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
