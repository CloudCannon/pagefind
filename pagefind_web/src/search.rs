use std::{
    borrow::Cow,
    cmp::Ordering,
    collections::HashMap,
    ops::{Add, AddAssign, Div},
};

use crate::{util::*, PageWord, RankingWeights};
use bit_set::BitSet;
use pagefind_stem::Stemmer;

use crate::SearchIndex;

pub struct PageSearchResult {
    pub page: String,
    pub page_index: usize,
    pub page_score: f32, // TODO: tf-idf implementation? Paired with the dictionary-in-meta approach
    pub word_locations: Vec<BalancedWordScore>,
}

struct MatchingPageWord<'a> {
    word: &'a PageWord,
    word_str: &'a str,
    length_bonus: f32,
    num_pages_matching: usize,
}

#[derive(Debug, Clone)]
struct VerboseWordLocation<'a> {
    word_str: &'a str,
    weight: u8,
    word_location: u32,
    length_bonus: f32,
}

#[derive(Debug, Clone)]
pub struct BalancedWordScore {
    pub weight: u8,
    pub balanced_score: f32,
    pub word_location: u32,
}

#[derive(Debug)]
struct BM25Params {
    weighted_term_frequency: f32,
    document_length: f32,
    average_page_length: f32,
    total_pages: usize,
    pages_containing_term: usize,
    length_bonus: f32,
}

/// Returns a score between 0.0 and 1.0 for the given word.
/// 1.0 implies the word is the exact length we need,
/// and that decays as the word becomes longer or shorter than the query word.
/// As `term_similarity_ranking` trends to zero, all output trends to 1.0.
/// As `term_similarity_ranking` increases, the score decays faster as differential grows.
fn word_length_bonus(differential: u8, term_similarity_ranking: f32) -> f32 {
    let std_dev = 2.0_f32;
    let base = (-0.5 * (differential as f32).powi(2) / std_dev.powi(2)).exp();
    let max_value = term_similarity_ranking.exp();
    (base * term_similarity_ranking).exp() / max_value
}

fn calculate_bm25_word_score(
    BM25Params {
        weighted_term_frequency,
        document_length,
        average_page_length,
        total_pages,
        pages_containing_term,
        length_bonus,
    }: BM25Params,
    ranking: &RankingWeights,
) -> f32 {
    let weighted_with_length = weighted_term_frequency * length_bonus;

    let k1 = ranking.term_saturation;
    let b = ranking.page_length;

    let idf = (total_pages as f32 - pages_containing_term as f32 + 0.5)
        .div(pages_containing_term as f32 + 0.5)
        .add(1.0) // Prevent IDF from ever being negative
        .ln();

    let bm25_tf = (k1 + 1.0) * weighted_with_length
        / (k1 * (1.0 - b + b * (document_length / average_page_length)) + weighted_with_length);

    // Use ranking.term_frequency to interpolate between only caring about BM25's term frequency,
    // and only caring about the original weighted word count on the page.
    // Attempting to scale the original weighted word count to roughly the same bounds as the BM25 output (k1 + 1)
    let raw_count_scalar = average_page_length / 5.0;
    let scaled_raw_count = (weighted_with_length / raw_count_scalar).min(k1 + 1.0);
    let tf = (1.0 - ranking.term_frequency) * scaled_raw_count + ranking.term_frequency * bm25_tf;

    debug!({
        format! {"TF is {tf:?}, IDF is {idf:?}"}
    });

    idf * tf
}

fn calculate_individual_word_score(
    VerboseWordLocation {
        word_str: _,
        weight,
        length_bonus,
        word_location,
    }: VerboseWordLocation,
) -> BalancedWordScore {
    let balanced_score = (weight as f32).powi(2) * length_bonus;

    BalancedWordScore {
        weight,
        balanced_score,
        word_location,
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
    ) -> (Vec<usize>, Vec<PageSearchResult>) {
        debug!({
            format! {"Searching {:?}", term}
        });

        let total_pages = self.pages.len();

        let mut unfiltered_results: Vec<usize> = vec![];
        let mut maps = Vec::new();
        let mut words: Vec<MatchingPageWord> = Vec::new();
        let split_term = stems_from_term(term);

        for term in split_term.iter() {
            let mut word_maps = Vec::new();
            for (word, word_index) in self.find_word_extensions(&term) {
                let length_differential: u8 = (word.len().abs_diff(term.len()) + 1)
                    .try_into()
                    .unwrap_or(std::u8::MAX);

                words.extend(word_index.iter().map(|pageword| MatchingPageWord {
                    word: pageword,
                    word_str: &word,
                    length_bonus: word_length_bonus(
                        length_differential,
                        self.ranking_weights.term_similarity,
                    ),
                    num_pages_matching: word_index.len(),
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
            let page = &self.pages[page_index];

            let mut word_locations: Vec<_> = words
                .iter()
                .filter_map(|w| {
                    if w.word.page as usize == page_index {
                        Some(
                            w.word
                                .locs
                                .iter()
                                .map(|(weight, location)| VerboseWordLocation {
                                    word_str: w.word_str,
                                    weight: *weight,
                                    word_location: *location,
                                    length_bonus: w.length_bonus,
                                }),
                        )
                    } else {
                        None
                    }
                })
                .flatten()
                .collect();
            word_locations
                .sort_unstable_by_key(|VerboseWordLocation { word_location, .. }| *word_location);

            debug!({
                format! {"Found the raw word locations {:?}", word_locations}
            });

            let mut unique_word_locations: Vec<BalancedWordScore> =
                Vec::with_capacity(word_locations.len());
            let mut weighted_words: HashMap<&str, usize> = HashMap::with_capacity(words.len());

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
                    } else {
                        weighted_words
                            .entry(working_word.word_str)
                            .or_default()
                            .add_assign(working_word.weight as usize);

                        unique_word_locations.push(calculate_individual_word_score(working_word));
                        working_word = next_word;
                    }
                }
                weighted_words
                    .entry(working_word.word_str)
                    .or_default()
                    .add_assign(working_word.weight as usize);

                unique_word_locations.push(calculate_individual_word_score(working_word));
            }

            debug!({
                format! {"Coerced to unique locations {:?}", unique_word_locations}
            });
            debug!({
                format! {"Words have the final weights {:?}", weighted_words}
            });

            let word_scores =
                weighted_words
                    .into_iter()
                    .map(|(word_str, weighted_term_frequency)| {
                        let matched_word = words
                            .iter()
                            .find(|w| w.word_str == word_str)
                            .expect("word should be in the initial set");

                        let params = BM25Params {
                            weighted_term_frequency: (weighted_term_frequency as f32) / 24.0,
                            document_length: page.word_count as f32,
                            average_page_length: self.average_page_length,
                            total_pages,
                            pages_containing_term: matched_word.num_pages_matching,
                            length_bonus: matched_word.length_bonus,
                        };

                        debug!({
                            format! {"Calculating BM25 with the params {:?}", params}
                        });
                        debug!({
                            format! {"And the weights {:?}", self.ranking_weights}
                        });

                        let score = calculate_bm25_word_score(params, &self.ranking_weights);

                        debug!({
                            format! {"BM25 gives us the score {:?}", score}
                        });

                        score
                    });

            let page_score = word_scores.sum();

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

        debug!({ "Sorting by score" });
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
