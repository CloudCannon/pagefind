use bit_set::BitSet;

use crate::util::*;
use crate::SearchIndex;

impl SearchIndex {
    // TODO: Move this intersection, and the string formatting, into better methods.
    pub fn get_filters(&self, intersect_pages: Option<Vec<usize>>) -> String {
        let intersect_pages = intersect_pages.as_ref();
        let mut results: Vec<String> = self
            .filters
            .iter()
            .map(|(filter, values)| {
                let mut values: Vec<String> = values
                    .iter()
                    .map(|(value, pages)| {
                        let len = match intersect_pages {
                            Some(intersection) => pages
                                .iter()
                                .filter(|p| intersection.contains(&(**p as usize)))
                                .count(),
                            None => pages.len(),
                        };
                        format!("{}:{}", value, len)
                    })
                    .collect();
                values.sort();

                format!("{}:{}", filter, values.join("__PF_VALUE_DELIM__"))
            })
            .collect();

        results.sort();
        results.join("__PF_FILTER_DELIM__")
    }

    pub fn filter(&self, filter: &str) -> Option<BitSet> {
        let filters = filter.split("__PF_FILTER_DELIM__");

        let mut maps = Vec::new();

        for filter in filters {
            if let Some((filter, value)) = filter.split_once(':') {
                debug!({
                    format! {"Filtering for {}: {}", filter, value}
                });
                if let Some(filter_map) = self.filters.get(filter) {
                    debug!({
                        format! {"Found a map for {}: {:#?}", filter, filter_map}
                    });
                    if let Some(filter_pages) = filter_map.get(value) {
                        debug!({
                            format! {"Found the value {}", value}
                        });
                        let mut set = BitSet::new();
                        for page in filter_pages {
                            set.insert(*page as usize);
                        }
                        maps.push(set);
                    } else {
                        // Filter does not exist, push in a set of 0 pages to force no results
                        maps.push(BitSet::new());
                        debug!({
                            format! {"No value exists for {}", value}
                        });
                    }
                } else {
                    // Filter does not exist, push in a set of 0 pages to force no results
                    maps.push(BitSet::new());
                    debug!({
                        format! {"No map exists for {}", filter}
                    });
                }
            } else {
                debug!({
                    format! {"Bad filter (no `:`): {:?}", filter}
                })
            }
        }

        let mut maps = maps.drain(..);
        let mut results = maps.next()?;

        for map in maps {
            results.intersect_with(&map);
        }

        Some(results)
    }
}
