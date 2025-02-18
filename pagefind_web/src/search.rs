use std::{
    borrow::Cow,
    cmp::Ordering,
    collections::BTreeMap,
    ops::{Add, AddAssign, Div},
};

use crate::{util::*, PageWord, RankingWeights};
use bit_set::BitSet;
use pagefind_stem::Stemmer;

use crate::SearchIndex;

pub struct PageSearchResult {
    pub page: String,
    pub page_index: usize,
    pub page_length: u32,
    pub page_score: f32,
    pub word_locations: Vec<BalancedWordScore>,
    pub verbose_scores: Option<Vec<(String, ScoringMetrics, BM25Params)>>,
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
    pub verbose_word_info: Option<VerboseWordInfo>,
}

#[derive(Debug, Clone)]
pub struct VerboseWordInfo {
    pub word: String,
    pub length_bonus: f32,
}

#[derive(Debug)]
pub struct BM25Params {
    pub weighted_term_frequency: f32,
    pub document_length: f32,
    pub average_page_length: f32,
    pub total_pages: usize,
    pub pages_containing_term: usize,
    pub length_bonus: f32,
}

#[derive(Clone, Copy)]
pub struct ScoringMetrics {
    pub idf: f32,
    pub bm25_tf: f32,
    pub raw_tf: f32,
    pub pagefind_tf: f32,
    pub score: f32,
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
) -> ScoringMetrics {
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
    let raw_tf = (weighted_with_length / raw_count_scalar).min(k1 + 1.0);
    let pagefind_tf = (1.0 - ranking.term_frequency) * raw_tf + ranking.term_frequency * bm25_tf;

    debug!({
        format! {"TF is {pagefind_tf:?}, IDF is {idf:?}"}
    });

    ScoringMetrics {
        idf,
        bm25_tf,
        raw_tf,
        pagefind_tf,
        score: idf * pagefind_tf,
    }
}

fn calculate_individual_word_score(
    VerboseWordLocation {
        word_str,
        weight,
        length_bonus,
        word_location,
    }: VerboseWordLocation,
    playground_mode: bool,
) -> BalancedWordScore {
    let balanced_score = (weight as f32).powi(2) * length_bonus;

    BalancedWordScore {
        weight,
        balanced_score,
        word_location,
        verbose_word_info: if playground_mode {
            Some(VerboseWordInfo {
                word: word_str.to_string(),
                length_bonus,
            })
        } else {
            None
        },
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
            let map = match intersect_maps(maps) {
                Some(maps) => maps,
                // Results must exist at this point.
                None => std::process::abort(),
            };
            unfiltered_results.extend(map.iter());
            maps = vec![map];
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

            if let (Some(loc_0), Some(loc_rest)) = (word_locations.get(0), word_locations.get(1..))
            {
                'indexes: for (_, pos) in loc_0 {
                    let mut i = *pos;
                    for subsequent in loc_rest {
                        i += 1;
                        // Test each subsequent word map to try and find a contiguous block
                        if !subsequent.iter().any(|(_, p)| *p == i) {
                            continue 'indexes;
                        }
                    }
                    let page = match self.pages.get(page_index) {
                        Some(p) => p,
                        None => std::process::abort(),
                    };
                    let search_result = PageSearchResult {
                        page: page.hash.clone(),
                        page_index,
                        page_score: 1.0,
                        page_length: page.word_count,
                        word_locations: ((*pos..=i).map(|w| BalancedWordScore {
                            weight: 1,
                            balanced_score: 1.0,
                            word_location: w,
                            verbose_word_info: None, // TODO: bring playground info to quoted searches
                        }))
                        .collect(),
                        verbose_scores: None, // TODO: bring playground info to quoted searches
                    };
                    pages.push(search_result);
                    break 'indexes;
                }
            } else {
                let page = match self.pages.get(page_index) {
                    Some(p) => p,
                    None => std::process::abort(),
                };
                if let Some(loc_0) = word_locations.get(0) {
                    let search_result = PageSearchResult {
                        page: page.hash.clone(),
                        page_index,
                        page_score: 1.0,
                        page_length: page.word_count,
                        word_locations: loc_0
                            .iter()
                            .map(|(weight, word_location)| BalancedWordScore {
                                weight: *weight,
                                balanced_score: *weight as f32,
                                word_location: *word_location,
                                verbose_word_info: None, // TODO: bring playground info to quoted searches
                            })
                            .collect(),
                        verbose_scores: None, // TODO: bring playground info to quoted searches
                    };
                    pages.push(search_result);
                }
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
            let map = match intersect_maps(maps) {
                Some(maps) => maps,
                // Results must exist at this point.
                None => std::process::abort(),
            };
            unfiltered_results.extend(map.iter());
            maps = vec![map];
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

        for (page_index, page) in results
            .iter()
            .flat_map(|p| self.pages.get(p).map(|page| (p, page)))
        {
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
            let mut weighted_words: BTreeMap<&str, usize> = BTreeMap::new();

            if let Some(mut working_word) = word_locations.get(0).cloned() {
                for next_word in word_locations.into_iter().skip(1) {
                    // If we're matching the same position again (this Vec is in location order)
                    if working_word.word_location == next_word.word_location {
                        if next_word.weight < working_word.weight {
                            // If the new word is weighted _lower_ than the working word,
                            // we want to use the lower value. (Lowest weight wins)
                            working_word.weight = next_word.weight;
                            working_word.length_bonus = next_word.length_bonus;
                        } else if next_word.weight == working_word.weight {
                            // If the new word is weighted the same,
                            // we want to combine them to boost matching both halves of a compound word
                            working_word.weight += next_word.weight;
                            working_word.length_bonus += next_word.length_bonus;
                        }
                        // We don't want to do anything if the new word is weighted higher
                        // (Lowest weight wins)
                    } else {
                        weighted_words
                            .entry(working_word.word_str)
                            .or_default()
                            .add_assign(working_word.weight as usize);

                        unique_word_locations.push(calculate_individual_word_score(
                            working_word,
                            self.playground_mode,
                        ));
                        working_word = next_word;
                    }
                }
                weighted_words
                    .entry(working_word.word_str)
                    .or_default()
                    .add_assign(working_word.weight as usize);

                unique_word_locations.push(calculate_individual_word_score(
                    working_word,
                    self.playground_mode,
                ));
            }

            debug!({
                format! {"Coerced to unique locations {:?}", unique_word_locations}
            });
            debug!({
                format! {"Words have the final weights {:?}", weighted_words}
            });

            let mut verbose_scores = if self.playground_mode {
                Some(vec![])
            } else {
                None
            };
            let word_scores =
                weighted_words
                    .into_iter()
                    .map(|(word_str, weighted_term_frequency)| {
                        let matched_word = words
                            .iter()
                            .find(|w| w.word_str == word_str)
                            .expect("word should be in the initial set");

                        let params = || BM25Params {
                            weighted_term_frequency: (weighted_term_frequency as f32) / 24.0,
                            document_length: page.word_count as f32,
                            average_page_length: self.average_page_length,
                            total_pages,
                            pages_containing_term: matched_word.num_pages_matching,
                            length_bonus: matched_word.length_bonus,
                        };

                        debug!({
                            format! {"Calculating BM25 with the params {:?}", params()}
                        });
                        debug!({
                            format! {"And the weights {:?}", self.ranking_weights}
                        });

                        let score = calculate_bm25_word_score(params(), &self.ranking_weights);

                        debug!({
                            format! {"BM25 gives us the score {:?}", score.score}
                        });

                        if let Some(verbose_scores) = verbose_scores.as_mut() {
                            verbose_scores.push((word_str.to_string(), score, params()));
                        }

                        score.score
                    });

            let page_score = word_scores.sum();

            let search_result = PageSearchResult {
                page: page.hash.clone(),
                page_index,
                page_score,
                page_length: page.word_count,
                word_locations: unique_word_locations,
                verbose_scores,
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

pub fn stems_from_term(term: &str) -> Vec<Cow<str>> {
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
