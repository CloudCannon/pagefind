use hashbrown::HashMap;

use crate::{
    fossick::FossickedData, fragments::PageFragment, index::index_metadata::MetaFilter,
    utils::full_hash, SearchOptions,
};
use index_filter::{FilterIndex, PackedValue};
use index_metadata::{MetaChunk, MetaIndex, MetaPage};
use index_words::{PackedPage, PackedWord, WordIndex};

mod index_filter;
mod index_metadata;
mod index_words;

pub struct PagefindIndexes {
    pub word_indexes: HashMap<String, Vec<u8>>,
    pub filter_indexes: HashMap<String, Vec<u8>>,
    pub meta_index: Vec<u8>,
    pub fragments: HashMap<String, PageFragment>,
}

pub async fn build_indexes<I>(pages: I, options: &SearchOptions) -> PagefindIndexes
where
    I: Iterator<Item = FossickedData>,
{
    let mut meta = MetaIndex {
        version: options.version.into(),
        pages: Vec::new(),
        index_chunks: Vec::new(),
        filters: Vec::new(),
    };

    let mut word_map: HashMap<String, PackedWord> = HashMap::new();
    let mut filter_map: HashMap<String, HashMap<String, Vec<usize>>> = HashMap::new();
    let mut fragments: HashMap<String, PageFragment> = HashMap::new();

    for (page_number, mut page) in pages.enumerate() {
        page.fragment.page_number = page_number;

        for (word, positions) in page.word_data {
            let packed_page = PackedPage {
                page_number,
                locs: positions.clone(),
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
                        Some(page_array) => page_array.push(page_number),
                        None => {
                            value_map.insert(value.clone(), vec![page_number]);
                        }
                    },
                    None => {
                        let mut value_map = HashMap::new();
                        value_map.insert(value.clone(), vec![page_number]);
                        filter_map.insert(filter.clone(), value_map);
                    }
                }
            }
        }

        let mut short_hash = &page.fragment.hash[0..=6];
        // If we hit a collision, extend both hashes until we stop colliding
        while let Some(collision) = fragments.remove(short_hash) {
            let new_length = short_hash.len() + 1;

            fragments.insert(collision.hash[0..=new_length].to_string(), collision);
            short_hash = &page.fragment.hash[0..=new_length];

            if short_hash.len() == page.fragment.hash.len() {
                break;
            }
        }
        fragments.insert(short_hash.to_string(), page.fragment);
    }

    meta.pages = fragments
        .iter()
        .map(|(hash, fragment)| MetaPage {
            hash: hash.clone(),
            word_count: fragment.data.word_count as u32,
        })
        .collect();

    meta.pages
        .sort_by_cached_key(|p| fragments.get(&p.hash).unwrap().page_number);

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
        let hash = full_hash(&filter_index);
        let mut short_hash = &hash[0..=6];

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
        panic!("Too many documents to index");
    }

    println!("Indexed {:?} pages", meta.pages.len());
    println!("Indexed {:?} words", word_map.len());
    println!("Indexed {:?} filters", meta.filters.len());

    // TODO: Parameterize these chunk sizes via options
    let chunks = chunk_index(word_map, 20000);
    meta.index_chunks = chunk_meta(&chunks);

    let mut word_indexes: HashMap<String, Vec<u8>> = HashMap::new();
    for (i, chunk) in chunks.into_iter().enumerate() {
        let mut word_index: Vec<u8> = Vec::new();
        let _ = minicbor::encode::<WordIndex, &mut Vec<u8>>(
            WordIndex { words: chunk },
            word_index.as_mut(),
        );

        let hash = full_hash(&word_index);

        let mut short_hash = &hash[0..=6];
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

    println!("Created {:?} index chunks", word_indexes.len());

    let mut meta_index: Vec<u8> = Vec::new();
    let _ = minicbor::encode::<MetaIndex, &mut Vec<u8>>(meta, meta_index.as_mut());

    PagefindIndexes {
        word_indexes,
        filter_indexes,
        meta_index,
        fragments,
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

#[cfg(test)]
mod tests {
    use super::*;

    trait Mock {
        fn word(&mut self, word: &str, page_number: usize, locs: Vec<u32>);
    }
    impl Mock for HashMap<String, PackedWord> {
        fn word(&mut self, word: &str, page_number: usize, locs: Vec<u32>) {
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
