use hashbrown::HashMap;

use crate::{
    fossick::{FossickedData, FossickedWord},
    index::index_metadata::MetaFilter,
    utils::full_hash,
    SearchOptions,
};
use index_filter::{FilterIndex, PackedValue};
use index_metadata::{MetaChunk, MetaIndex, MetaPage};
use index_words::{PackedPage, PackedWord, WordIndex};

use self::index_metadata::MetaSort;

mod index_filter;
mod index_metadata;
mod index_words;

pub struct PagefindIndexes {
    pub word_indexes: HashMap<String, Vec<u8>>,
    pub filter_indexes: HashMap<String, Vec<u8>>,
    pub meta_index: (String, Vec<u8>),
    pub fragments: Vec<(String, String)>,
    pub sorts: Vec<String>,
    pub language: String,
    pub word_count: usize,
}

#[derive(Clone)]
struct IntermediaryPageData {
    full_hash: String,
    encoded_data: String,
    word_count: usize,
    page_number: usize,
}

#[derive(Debug)]
enum SortType {
    String,
    Number,
}

pub async fn build_indexes(
    mut pages: Vec<FossickedData>,
    language: String,
    options: &SearchOptions,
) -> PagefindIndexes {
    let mut meta = MetaIndex {
        version: options.version.into(),
        pages: Vec::new(),
        index_chunks: Vec::new(),
        filters: Vec::new(),
        sorts: Vec::new(),
    };

    /*
        - Collect all sort keys
        - Sort `pages` by one of them and set `default_sort`
        - Do the main enumerate loop as an iter_mut and set page numbers
        - Later on, for each other sort key:
            - Sort the `pages` array and output the page numbers to `alternate_sorts`
    */

    let mut word_map: HashMap<String, PackedWord> = HashMap::new();
    let mut filter_map: HashMap<String, HashMap<String, Vec<usize>>> = HashMap::new();
    let mut fragment_hashes: HashMap<String, IntermediaryPageData> = HashMap::new();
    let mut fragments: Vec<(usize, (String, IntermediaryPageData))> = Vec::new();

    for (page_number, mut page) in pages.iter_mut().enumerate() {
        page.fragment.page_number = page_number;
    }

    // Get all possible sort keys
    let mut sorts: Vec<_> = pages
        .iter()
        .flat_map(|page| page.sort.keys().cloned())
        .collect();
    sorts.sort_unstable();
    sorts.dedup();

    // Determine the best sorting parser that fits all available values for each given key
    let mut sort_types: HashMap<String, SortType> = HashMap::new();
    for sort in sorts.iter() {
        let mut sort_values = pages.iter().flat_map(|page| page.sort.get(sort));
        sort_types.insert(
            sort.clone(),
            if sort_values.all(|v| parse_int_sort(v).is_some() || parse_float_sort(v).is_some()) {
                SortType::Number
            } else {
                SortType::String
            },
        );
    }

    for (sort_key, sort_type) in sort_types {
        let mut page_values: Vec<_> = pages
            .iter()
            .flat_map(|page| {
                page.sort
                    .get(&sort_key)
                    .map(|v| (v, page.fragment.page_number))
            })
            .collect();
        options.logger.v_info(format!(
            "Prebuilding sort order for {sort_key}, processed as type: {sort_type:#?}"
        ));
        match sort_type {
            SortType::String => page_values.sort_by_key(|p| p.0),
            SortType::Number => page_values.sort_by(|p1, p2| {
                let p1 = parse_int_sort(p1.0)
                    .map(|i| i as f32)
                    .unwrap_or_else(|| parse_float_sort(p1.0).unwrap_or_default());
                let p2 = parse_int_sort(p2.0)
                    .map(|i| i as f32)
                    .unwrap_or_else(|| parse_float_sort(p2.0).unwrap_or_default());

                p1.total_cmp(&p2)
            }),
        }
        meta.sorts.push(MetaSort {
            sort: sort_key,
            pages: page_values.into_iter().map(|p| p.1).collect(),
        });
    }

    for page in pages.into_iter() {
        for (word, mut positions) in page.word_data {
            // A page weight of 1 is encoded as 25. Since most words should be this weight,
            // we want to sort them to be first in the locations array to reduce filesize
            // when we inline weight changes
            positions.sort_by_cached_key(|p| if p.weight == 25 { 0 } else { p.weight });

            let mut current_weight = 25;
            let mut weighted_positions = Vec::with_capacity(positions.len());
            // Calculate our output list of positions with weights.
            // This is a vec of page positions, with a change in weight for subsequent positions
            // denoted by a negative integer.
            positions
                .into_iter()
                .for_each(|FossickedWord { position, weight }| {
                    if weight != current_weight {
                        weighted_positions.extend([(weight as i32) * -1 - 1, position as i32]);
                        current_weight = weight;
                    } else {
                        weighted_positions.push(position as i32)
                    }
                });

            let packed_page = PackedPage {
                page_number: page.fragment.page_number,
                locs: weighted_positions,
            };

            match word_map.get_mut(&word) {
                Some(packed) => packed.pages.push(packed_page),
                None => {
                    word_map.insert(
                        word.clone(),
                        PackedWord {
                            word,
                            pages: vec![packed_page],
                        },
                    );
                }
            }
        }

        for (filter, values) in &page.fragment.data.filters {
            for value in values {
                match filter_map.get_mut(filter) {
                    Some(value_map) => match value_map.get_mut(value) {
                        Some(page_array) => page_array.push(page.fragment.page_number),
                        None => {
                            value_map.insert(value.clone(), vec![page.fragment.page_number]);
                        }
                    },
                    None => {
                        let mut value_map = HashMap::new();
                        value_map.insert(value.clone(), vec![page.fragment.page_number]);
                        filter_map.insert(filter.clone(), value_map);
                    }
                }
            }
        }

        let encoded_data = serde_json::to_string(&page.fragment.data).unwrap();
        let encoded_page = IntermediaryPageData {
            full_hash: format!("{}_{}", language, full_hash(encoded_data.as_bytes())),
            word_count: page.fragment.data.word_count,
            page_number: page.fragment.page_number,
            encoded_data,
        };

        let mut short_hash = &encoded_page.full_hash[0..=(language.len() + 7)];

        // If we hit a collision, extend one until we stop colliding
        // TODO: There are some collision issues here.
        // If two builds match a collision in different orders the hashes will swap,
        // which could return incorrect data due to files being cached.
        while let Some(collision) = fragment_hashes.get(short_hash) {
            if collision.full_hash == encoded_page.full_hash {
                // These pages are identical. Add both under the same hash.
                fragments.push((
                    collision.word_count,
                    (collision.full_hash.clone(), collision.clone()),
                ));
            } else {
                let new_length = short_hash.len();
                short_hash = &encoded_page.full_hash[0..=new_length];
            }
        }
        fragment_hashes.insert(short_hash.to_string(), encoded_page);
    }

    fragments.extend(
        fragment_hashes
            .into_iter()
            .map(|(hash, frag)| (frag.word_count, (hash, frag))),
    );
    fragments.sort_by_cached_key(|(_, (_, fragment))| fragment.page_number);

    meta.pages
        .extend(fragments.iter().map(|(word_count, (hash, _))| MetaPage {
            hash: hash.clone(),
            word_count: *word_count as u32,
        }));

    // TODO: Change filter indexes to BTree to give them a stable hash.
    let mut filter_indexes = HashMap::new();
    for (filter, values) in filter_map {
        let mut filter_index: Vec<u8> = Vec::new();
        let _ = minicbor::encode::<FilterIndex, &mut Vec<u8>>(
            FilterIndex {
                filter: filter.clone(),
                values: values
                    .into_iter()
                    .map(|(value, pages)| PackedValue { value, pages })
                    .collect(),
            },
            filter_index.as_mut(),
        );
        let hash = format!("{}_{}", language, full_hash(&filter_index));
        let mut short_hash = &hash[0..=(language.len() + 7)];

        // If we hit a collision, extend one hash until we stop colliding
        // TODO: DRY
        while filter_indexes.contains_key(short_hash) {
            let new_length = short_hash.len() + 1;
            short_hash = &hash[0..=new_length];

            if short_hash.len() == hash.len() {
                break;
            }
        }
        filter_indexes.insert(short_hash.to_string(), filter_index);
        meta.filters.push(MetaFilter {
            filter,
            hash: short_hash.to_string(),
        })
    }

    if TryInto::<u32>::try_into(meta.pages.len()).is_err() {
        options.logger.error(format!(
            "Language {} has too many documents to index, must be < {}",
            language,
            u32::MAX
        ));
        std::process::exit(1);
    }

    // TODO: Parameterize these chunk sizes via options
    let word_count = word_map.len();
    let chunks = chunk_index(word_map, 20000);
    meta.index_chunks = chunk_meta(&chunks);

    let mut word_indexes: HashMap<String, Vec<u8>> = HashMap::new();
    for (i, chunk) in chunks.into_iter().enumerate() {
        let mut word_index: Vec<u8> = Vec::new();
        let _ = minicbor::encode::<WordIndex, &mut Vec<u8>>(
            WordIndex { words: chunk },
            word_index.as_mut(),
        );

        let hash = format!("{}_{}", language, full_hash(&word_index));
        let mut short_hash = &hash[0..=(language.len() + 7)];

        // If we hit a collision, extend one hash until we stop colliding
        while word_indexes.contains_key(short_hash) {
            let new_length = short_hash.len() + 1;
            short_hash = &hash[0..=new_length];

            if short_hash.len() == hash.len() {
                break;
            }
        }
        word_indexes.insert(short_hash.to_string(), word_index);
        meta.index_chunks[i].hash = short_hash.into();
    }

    let mut meta_index: Vec<u8> = Vec::new();
    let _ = minicbor::encode::<MetaIndex, &mut Vec<u8>>(meta, meta_index.as_mut());

    let meta_hash = format!(
        "{}_{}",
        language,
        &full_hash(&meta_index)[0..=(language.len() + 7)]
    );

    PagefindIndexes {
        word_indexes,
        filter_indexes,
        sorts,
        meta_index: (meta_hash, meta_index),
        fragments: fragments
            .into_iter()
            .map(|(_, (hash, frag))| (hash, frag.encoded_data))
            .collect(),
        language,
        word_count,
    }
}

fn chunk_index(word_map: HashMap<String, PackedWord>, chunk_size: usize) -> Vec<Vec<PackedWord>> {
    // TODO: Use ye olde BTree
    let mut words = word_map
        .into_iter()
        .map(|(_, w)| w)
        .collect::<Vec<PackedWord>>();
    words.sort_by_key(|w| w.word.clone());

    let mut index_chunks = Vec::new();

    let mut index_chunk = Vec::new();
    let mut index_chunk_size = 0;
    for word in words.into_iter() {
        index_chunk_size += word.pages.iter().map(|p| p.locs.len() + 1).sum::<usize>();
        index_chunk.push(word);
        if index_chunk_size >= chunk_size {
            index_chunks.push(index_chunk.clone());
            index_chunk.clear();
            index_chunk_size = 0;
        }
    }
    if !index_chunk.is_empty() {
        index_chunks.push(index_chunk);
    }

    index_chunks
}

fn chunk_meta(indexes: &[Vec<PackedWord>]) -> Vec<MetaChunk> {
    let mut named_chunks: Vec<MetaChunk> = Vec::new();

    for chunk in indexes.iter() {
        named_chunks.push(MetaChunk {
            from: chunk.first().map_or("".into(), |w| w.word.clone()),
            to: chunk.last().map_or("".into(), |w| w.word.clone()),
            hash: "".into(),
        });
    }
    if named_chunks.len() > 1 {
        for i in 0..named_chunks.len() - 1 {
            let chunks = &mut named_chunks[i..=i + 1];
            let prefixes = get_prefixes((&chunks[0].to, &chunks[1].from));
            chunks[0].to = prefixes.0;
            chunks[1].from = prefixes.1;
        }
    }

    named_chunks
}

fn get_prefixes((a, b): (&str, &str)) -> (String, String) {
    let common_prefix_length: usize = b
        .chars()
        .zip(a.chars())
        .take_while(|&(a, b)| a == b)
        .count();

    let a_prefix = a.chars().take(common_prefix_length + 1).collect::<String>();
    let b_prefix = b.chars().take(common_prefix_length + 1).collect::<String>();

    (a_prefix, b_prefix)
}

fn parse_int_sort(value: &str) -> Option<i32> {
    lexical_core::parse::<i32>(value.as_bytes()).ok()
}

fn parse_float_sort(value: &str) -> Option<f32> {
    lexical_core::parse::<f32>(value.as_bytes()).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    trait Mock {
        fn word(&mut self, word: &str, page_number: usize, locs: Vec<i32>);
    }
    impl Mock for HashMap<String, PackedWord> {
        fn word(&mut self, word: &str, page_number: usize, locs: Vec<i32>) {
            let page = PackedPage { page_number, locs };
            match self.get_mut(word) {
                Some(w) => w.pages.push(page),
                None => {
                    let _ = self.insert(
                        word.into(),
                        PackedWord {
                            word: word.into(),
                            pages: vec![page],
                        },
                    );
                }
            }
        }
    }

    fn test_words() -> HashMap<String, PackedWord> {
        let mut words = HashMap::new();
        words.word("apple", 1, vec![20, 40, 60]);
        words.word("apple", 5, vec![3, 6, 9]);
        words.word("apricot", 5, vec![45, 3432, 6003]);
        words.word("banana", 5, vec![100, 500, 900, 566]);
        words.word("peach", 5, vec![383, 2, 678]);

        words
    }

    #[test]
    fn build_index_chunks() {
        let chunks = chunk_index(test_words(), 8);

        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks[0][0].word, "apple");
        assert_eq!(chunks[1][0].word, "apricot");
        assert_eq!(chunks[1][1].word, "banana");
        assert_eq!(chunks[2][0].word, "peach");
    }

    #[test]
    fn build_chunk_meta() {
        let chunks = chunk_index(test_words(), 8);
        let meta = chunk_meta(&chunks);
        assert_eq!(meta.len(), 3);
        assert_eq!(
            meta[0],
            MetaChunk {
                from: "apple".into(),
                to: "app".into(),
                hash: "".into(),
            }
        );
        assert_eq!(
            meta[1],
            MetaChunk {
                from: "apr".into(),
                to: "b".into(),
                hash: "".into(),
            }
        );
        assert_eq!(
            meta[2],
            MetaChunk {
                from: "p".into(),
                to: "peach".into(),
                hash: "".into(),
            }
        );
    }

    #[test]
    fn common_prefix() {
        assert_eq!(
            get_prefixes(("apple", "apricot")),
            ("app".into(), "apr".into())
        );
        assert_eq!(
            get_prefixes(("cataraman", "yacht")),
            ("c".into(), "y".into())
        );
        assert_eq!(
            get_prefixes(("cath", "cathartic")),
            ("cath".into(), "catha".into())
        );
        // This should be an invalid state, but just in case:
        assert_eq!(
            get_prefixes(("catha", "cath")),
            ("catha".into(), "cath".into())
        );
    }
}
