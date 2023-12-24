use std::{borrow::Cow, cmp::Ordering};

use crate::{util::*, PageWord};
use bit_set::BitSet;
use pagefind_stem::Stemmer;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::SearchIndex;

pub struct PageSearchResult {
    pub page: String,
    pub page_index: usize,
    pub page_score: f32, // TODO: tf-idf implementation? Paired with the dictionary-in-meta approach
    pub word_locations: Vec<BalancedWordScore>,
}

struct ScoredPageWord<'a> {
    word: &'a PageWord,
    length_differential: u8,
    word_frequency: f32,
}

#[derive(Debug, Clone)]
struct VerboseWordLocation {
    weight: u8,
    length_differential: u8,
    word_frequency: f32,
    word_location: u32,
}

#[derive(Debug, Clone)]
pub struct BalancedWordScore {
    pub weight: u8,
    pub balanced_score: f32,
    pub word_location: u32,
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct RankingWeights {
    pub page_frequency: f32,
}

#[wasm_bindgen]
impl RankingWeights {
    #[wasm_bindgen(constructor)]
    pub fn new(
        page_frequency: f32,
    ) -> RankingWeights {
        RankingWeights {
            page_frequency,
        }
    }
}

impl From<VerboseWordLocation> for BalancedWordScore {
    fn from(
        VerboseWordLocation {
            weight,
            length_differential,
            word_frequency,
            word_location,
        }: VerboseWordLocation,
    ) -> Self {
        let word_length_bonus = if length_differential > 0 {
            (2.0 / length_differential as f32).max(0.2)
        } else {
            3.0
        };

        // Starting with the raw user-supplied (or derived) weight of the word,
        // we take it to the power of two to make the weight scale non-linear.
        // We then multiply it with word_length_bonus, which should be a roughly 0 -> 3 scale of how close
        // this was was in length to the target word.
        // That result is then multiplied by the word frequency, which is again a roughly 0 -> 2 scale
        // of how unique this word is in the entire site. (tf-idf ish)
        let balanced_score =
            ((weight as f32).powi(2) * word_length_bonus) * word_frequency.max(0.5);

        Self {
            weight,
            balanced_score,
            word_location,
        }
    }
}

impl SearchIndex {
    pub fn exact_term(
        &self,
        term: &str,
        filter_results: Option<BitSet>,
    ) -> (Vec<usize>, Vec<PageSearchResult>) {
        debug!({
            format! {"Searching {:?}", term}
        });

        let mut unfiltered_results: Vec<usize> = vec![];
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
                return (vec![], vec![]);
            }
        }

        if !maps.is_empty() {
            maps = vec![intersect_maps(maps).expect("some search results should exist here")];
            unfiltered_results.extend(maps[0].iter());
        }

        if let Some(filter) = filter_results {
            maps.push(filter);
        }

        let results = match intersect_maps(maps) {
            Some(map) => map,
            None => return (vec![], vec![]),
        };

        let mut pages: Vec<PageSearchResult> = vec![];

        for page_index in results.iter() {
            let word_locations: Vec<Vec<(u8, u32)>> = words
                .iter()
                .filter_map(|p| {
                    if p.page as usize == page_index {
                        Some(p.locs.iter().map(|d| *d).collect())
                    } else {
                        None
                    }
                })
                .collect();
            debug!({
                format! {"Word locations {:?}", word_locations}
            });

            if word_locations.len() > 1 {
                'indexes: for (_, pos) in &word_locations[0] {
                    let mut i = *pos;
                    for subsequent in &word_locations[1..] {
                        i += 1;
                        // Test each subsequent word map to try and find a contiguous block
                        if !subsequent.iter().any(|(_, p)| *p == i) {
                            continue 'indexes;
                        }
                    }
                    let page = &self.pages[page_index];
                    let search_result = PageSearchResult {
                        page: page.hash.clone(),
                        page_index,
                        page_score: 1.0,
                        word_locations: ((*pos..=i).map(|w| BalancedWordScore {
                            weight: 1,
                            balanced_score: 1.0,
                            word_location: w,
                        }))
                        .collect(),
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
                    word_locations: word_locations[0]
                        .iter()
                        .map(|(weight, word_location)| BalancedWordScore {
                            weight: *weight,
                            balanced_score: *weight as f32,
                            word_location: *word_location,
                        })
                        .collect(),
                };
                pages.push(search_result);
            }
        }

        (unfiltered_results, pages)
    }

    pub fn search_term(
        &self,
        term: &str,
        filter_results: Option<BitSet>,
        ranking: &RankingWeights,
    ) -> (Vec<usize>, Vec<PageSearchResult>) {
        debug!({
            format! {"Searching {:?}", term}
        });

        let total_pages = self.pages.len();

        let mut unfiltered_results: Vec<usize> = vec![];
        let mut maps = Vec::new();
        let mut words: Vec<ScoredPageWord> = Vec::new();
        let split_term = stems_from_term(term);

        for term in split_term.iter() {
            let mut word_maps = Vec::new();
            for (word, word_index) in self.find_word_extensions(&term) {
                let length_differential: u8 = (word.len().abs_diff(term.len()) + 1)
                    .try_into()
                    .unwrap_or(std::u8::MAX);
                let word_frequency: f32 = (total_pages
                    .checked_div(word_index.len())
                    .unwrap_or_default() as f32)
                    .log10();

                words.extend(word_index.iter().map(|pageword| ScoredPageWord {
                    word: pageword,
                    length_differential,
                    word_frequency,
                }));
                let mut set = BitSet::new();
                for page in word_index {
                    set.insert(page.page as usize);
                }
                word_maps.push(set);
            }
            if let Some(result) = union_maps(word_maps) {
                maps.push(result);
            }
        }
        // In the case where a search term was passed, but not found,
        // make sure we cause the entire search to return no results.
        if !split_term.is_empty() && maps.is_empty() {
            maps.push(BitSet::new());
        }

        if !maps.is_empty() {
            maps = vec![intersect_maps(maps).expect("some search results should exist here")];
            unfiltered_results.extend(maps[0].iter());
        }

        if let Some(filter) = filter_results {
            maps.push(filter);
        } else if maps.is_empty() {
            let mut all_filter = BitSet::new();
            for i in 0..self.pages.len() {
                all_filter.insert(i);
            }
            maps.push(all_filter);
        }

        let results = match intersect_maps(maps) {
            Some(map) => map,
            None => return (vec![], vec![]),
        };

        let mut pages: Vec<PageSearchResult> = vec![];

        for page_index in results.iter() {
            //                      length diff, word weight, word position
            let mut word_locations: Vec<VerboseWordLocation> = words
                .iter()
                .filter_map(
                    |ScoredPageWord {
                         word,
                         length_differential,
                         word_frequency,
                     }| {
                        if word.page as usize == page_index {
                            Some(
                                word.locs
                                    .iter()
                                    .map(|loc| VerboseWordLocation {
                                        weight: loc.0,
                                        length_differential: *length_differential,
                                        word_frequency: *word_frequency,
                                        word_location: loc.1,
                                    })
                                    .collect::<Vec<_>>(),
                            )
                        } else {
                            None
                        }
                    },
                )
                .flatten()
                .collect();
            debug!({
                format! {"Word locations {:?}", word_locations}
            });
            word_locations
                .sort_unstable_by_key(|VerboseWordLocation { word_location, .. }| *word_location);

            let mut unique_word_locations: Vec<BalancedWordScore> =
                Vec::with_capacity(word_locations.len());
            if !word_locations.is_empty() {
                let mut working_word = word_locations[0].clone();
                for next_word in word_locations.into_iter().skip(1) {
                    // If we're matching the same position again (this Vec is in location order)
                    if working_word.word_location == next_word.word_location {
                        if next_word.weight < working_word.weight {
                            // If the new word is weighted _lower_ than the working word,
                            // we want to use the lower value. (Lowest weight wins)
                            working_word.weight = next_word.weight;
                        } else if next_word.weight == working_word.weight {
                            // If the new word is weighted the same,
                            // we want to combine them to boost matching both halves of a compound word
                            working_word.weight += next_word.weight;
                        }
                        // We don't want to do anything if the new word is weighted higher
                        // (Lowest weight wins)

                        if next_word.length_differential > working_word.length_differential {
                            // If the new word is further from target than the working word,
                            // we want to use that value. (Longest diff wins)
                            working_word.length_differential = next_word.length_differential;
                        }
                    } else {
                        unique_word_locations.push(working_word.into());
                        working_word = next_word;
                    }
                }
                unique_word_locations.push(working_word.into());
            }

            let page = &self.pages[page_index];
            debug!({
                format! {"Sorted word locations {:?}, {:?} word(s)", unique_word_locations, page.word_count}
            });

            let page_score = (unique_word_locations
                .iter()
                .map(|BalancedWordScore { balanced_score, .. }| balanced_score)
                .sum::<f32>()
                / 24.0)
                / ((page.word_count as f32).ln() * (*ranking).page_frequency).exp();

            let search_result = PageSearchResult {
                page: page.hash.clone(),
                page_index,
                page_score,
                word_locations: unique_word_locations,
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

        (unfiltered_results, pages)
    }

    fn find_word_extensions(&self, term: &str) -> Vec<(&String, &Vec<PageWord>)> {
        let mut extensions = vec![];
        let mut longest_prefix = None;
        for (key, results) in self.words.iter() {
            if key.starts_with(term) {
                debug!({
                    format! {"Adding {:#?} to the query", key}
                });
                extensions.push((key, results));
            } else if term.starts_with(key)
                && key.len() > longest_prefix.map(String::len).unwrap_or_default()
            {
                longest_prefix = Some(key);
            }
        }
        if extensions.is_empty() {
            debug!({ "No word extensions found, checking the inverse" });
            if let Some(longest_prefix) = longest_prefix {
                if let Some(results) = self.words.get(longest_prefix) {
                    debug!({
                        format! {"Adding the prefix {:#?} to the query", longest_prefix}
                    });
                    extensions.push((longest_prefix, results));
                }
            }
        }
        extensions
    }
}

fn stems_from_term(term: &str) -> Vec<Cow<str>> {
    if term.trim().is_empty() {
        return vec![];
    }
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
