use bit_set::BitSet;

use crate::util::*;
use crate::SearchIndex;

impl SearchIndex {
    pub fn filter(&self, filter: &str) -> Option<BitSet> {
        let filters = filter.split("__PF_FILTER_DELIM__");

        let mut maps = Vec::new();

        for filter in filters {
            if let Some((filter, value)) = filter.split_once(":") {
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
                        debug!({
                            format! {"No value exists for {}", value}
                        });
                    }
                } else {
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
