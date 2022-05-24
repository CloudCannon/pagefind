use bit_set::BitSet;
use rust_stemmers::{Algorithm, Stemmer}; // TODO: too big, Stemming should be performed on the JS side

#[cfg(debug_assertions)]
use crate::debug_log;
use crate::SearchIndex;

pub struct PageSearchResult {
    pub page: String,
    pub word_frequency: f32, // TODO: tf-idf implementation? Paired with the dictionary-in-meta approach
    pub word_locations: Vec<u32>,
}

impl SearchIndex {
    pub fn search_term(&self, term: &str) -> Vec<PageSearchResult> {
        let terms = term.split(' ');
        // TODO: i18n
        // TODO: Stemming should be performed on the JS side of the boundary
        //       As the snowball implementation there seems a lot smaller and just as fast.
        let en_stemmer = Stemmer::create(Algorithm::English);

        #[cfg(debug_assertions)]
        debug_log(&format! {"Searching {:?}", term});

        let mut maps = Vec::new();
        let mut words = Vec::new();
        for term in terms {
            let term = en_stemmer.stem(term).into_owned(); // TODO: Remove this once JS stems
            if let Some(word_index) = self.words.get(&term) {
                words.extend(word_index);
                let mut set = BitSet::new();
                for page in word_index {
                    set.insert(page.page as usize);
                }
                maps.push(set);
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

        let mut pages: Vec<PageSearchResult> = vec![];

        for page in results.iter() {
            let word_locations: Vec<u32> = words
                .iter()
                .filter_map(|p| {
                    if p.page as usize == page {
                        Some(p.locs.clone())
                    } else {
                        None
                    }
                })
                .flatten()
                .collect();

            let page = &self.pages[page];
            let search_result = PageSearchResult {
                page: page.hash.clone(),
                word_frequency: word_locations.len() as f32 / page.word_count as f32,
                word_locations,
            };

            #[cfg(debug_assertions)]
            debug_log(
                &format! {"Page {} has {} matching terms (in {} total words), giving the word frequency {:?}", search_result.page, search_result.word_locations.len(), page.word_count, search_result.word_frequency},
            );

            pages.push(search_result);
        }

        pages.sort_by(|a, b| b.word_frequency.partial_cmp(&a.word_frequency).unwrap());

        pages
    }
}
