use bit_set::BitSet;
use microjson::JSONValue;
use microjson::JSONValueType;

use crate::util::*;
use crate::SearchIndex;

#[derive(Debug)]
pub enum FilterBehaviour {
    Any,
    All,
}

fn collapse(mut maps: Vec<BitSet>, behaviour: FilterBehaviour) -> BitSet {
    let mut maps = maps.drain(..);
    let mut output = maps.next().unwrap_or_default();

    for map in maps {
        match behaviour {
            FilterBehaviour::Any => output.union_with(&map),
            FilterBehaviour::All => output.intersect_with(&map),
        }
    }
    output
}

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

    fn parse_filter_value(
        &self,
        filter_key: &str,
        filter: JSONValue,
        behaviour: FilterBehaviour,
    ) -> BitSet {
        use JSONValueType as J;

        debug!({
            format! {"Processing value object {filter:?} with {behaviour:?}" }
        });

        let Some(filter_map) = self.filters.get(filter_key) else {
            debug!({ format! {"No map for {filter_key}"} });
            return BitSet::new();
        };

        let mut maps = Vec::new();
        let build_set = |val: JSONValue| {
            debug!({
                format! {"Adding the filter {filter_key}: {val:?}" }
            });
            if let Some(filter_pages) = filter_map.get(val.read_string().unwrap_or_default()) {
                let mut set = BitSet::new();
                for page in filter_pages {
                    set.insert(*page as usize);
                }
                set
            } else {
                debug!({
                    format! {"Nothing found. . . ." }
                });
                // Filter does not exist, push in a set of 0 pages to force no results
                BitSet::new()
            }
        };

        match filter.value_type {
            JSONValueType::String => maps.push(build_set(filter)),
            JSONValueType::Array => {
                if let Ok(arr) = filter.iter_array() {
                    for value in arr {
                        match value.value_type {
                            J::String => {
                                maps.push(build_set(value));
                            }
                            J::Object => {
                                maps.push(self.parse_filter_value(
                                    filter_key,
                                    value,
                                    FilterBehaviour::All,
                                ));
                            }
                            _ => continue,
                        }
                    }
                }
            }
            JSONValueType::Object => {
                if let Ok(obj) = filter.iter_object() {
                    for (k, value) in obj.filter_map(|o| o.ok()) {
                        match (k, value.value_type) {
                            ("any", J::Object | J::Array) => maps.push(self.parse_filter_value(
                                filter_key,
                                value,
                                FilterBehaviour::Any,
                            )),
                            ("all", J::Object | J::Array) => maps.push(self.parse_filter_value(
                                filter_key,
                                value,
                                FilterBehaviour::All,
                            )),
                            _ => {
                                debug!({
                                    format! {"Unsupported filter key {k} value {:?}", value.value_type}
                                });
                            }
                        }
                    }
                }
            }
            _ => {
                debug!({
                    format! {"Unsupported filter value {:?}", filter.value_type}
                });
                return BitSet::new();
            }
        }

        collapse(maps, behaviour)
    }

    fn parse_filter_arr(&self, filter: JSONValue, behaviour: FilterBehaviour) -> BitSet {
        use JSONValueType as J;
        debug_assert!(matches!(filter.value_type, J::Array));

        debug!({
            format! {"Processing outer array {filter:?} with {behaviour:?}" }
        });

        let mut maps = Vec::new();

        if let Ok(arr) = filter.iter_array() {
            for value in arr {
                if !matches!(value.value_type, J::Object) {
                    debug!({
                        format! {"Skipping {value:?} as it is not an object" }
                    });
                    continue;
                }
                maps.push(self.parse_filter_obj(value, FilterBehaviour::All))
            }
        }

        collapse(maps, behaviour)
    }

    fn parse_filter_obj(&self, filter: JSONValue, behaviour: FilterBehaviour) -> BitSet {
        use JSONValueType as J;
        debug_assert!(matches!(filter.value_type, J::Object));

        debug!({
            format! {"Processing outer object {filter:?} with {behaviour:?}" }
        });

        let mut maps = Vec::new();

        if let Ok(obj) = filter.iter_object() {
            for (k, value) in obj.filter_map(|o| o.ok()) {
                match (k, value.value_type) {
                    ("any", J::Object) => {
                        maps.push(self.parse_filter_obj(value, FilterBehaviour::Any))
                    }
                    ("all", J::Object) => {
                        maps.push(self.parse_filter_obj(value, FilterBehaviour::All))
                    }
                    ("any", J::Array) => {
                        maps.push(self.parse_filter_arr(value, FilterBehaviour::Any))
                    }
                    ("all", J::Array) => {
                        maps.push(self.parse_filter_arr(value, FilterBehaviour::All))
                    }
                    (k, _) => maps.push(self.parse_filter_value(k, value, FilterBehaviour::All)),
                }
            }
        }

        collapse(maps, behaviour)
    }

    pub fn filter(&self, filter: &str) -> Option<BitSet> {
        use JSONValueType as J;

        debug!({
            format! {"Filtering with {:?}", filter}
        });

        if self.filter_chunks(filter).is_none() {
            debug!({ "No filter names anywhere in this object. Pretending it doesn't exist." });
            return None;
        }

        let Ok(all_filters) = JSONValue::parse(filter) else {
            debug!({ "Malformed object passed to Pagefind filters" });
            return None;
        };
        if !matches!(all_filters.value_type, J::Object) {
            debug!({ "Filters was passed a non-object" });
            return None;
        }

        Some(self.parse_filter_obj(all_filters, FilterBehaviour::All))
    }

    pub fn dig_filter(&self, filter: JSONValue) -> Option<Vec<String>> {
        use JSONValueType as J;

        debug!({
            format! {"Digging into {:?} to look for filters", filter}
        });

        let mut has_filters = false;
        let mut indexes = Vec::new();

        match filter.value_type {
            J::Object => {
                if let Ok(obj) = filter.iter_object() {
                    for (k, value) in obj.filter_map(|o| o.ok()) {
                        match (k, value.value_type) {
                            ("any" | "all", J::Object | J::Array) => {
                                if let Some(nested_filters) = self.dig_filter(value) {
                                    has_filters = true;
                                    indexes.extend(nested_filters)
                                }
                            }
                            (filter, _) => {
                                has_filters = true;
                                if let Some(hash) = self.filter_chunks.get(filter) {
                                    debug!({
                                        format! {"Need {:?} for {:?}", hash, filter}
                                    });
                                    indexes.push(hash.clone());
                                } else {
                                    debug!({
                                        format! {"No hash found for {:?}", filter}
                                    })
                                }
                            }
                        }
                    }
                }
            }
            J::Array => {
                if let Ok(arr) = filter.iter_array() {
                    for value in arr {
                        match value.value_type {
                            J::Object | J::Array => {
                                if let Some(nested_filters) = self.dig_filter(value) {
                                    has_filters = true;
                                    indexes.extend(nested_filters)
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }

        if has_filters {
            Some(indexes)
        } else {
            None
        }
    }

    pub fn filter_chunks(&self, filter: &str) -> Option<Vec<String>> {
        debug!({
            format! {"Finding the filter chunks needed for {:?}", filter}
        });

        use JSONValueType as J;

        let Ok(all_filters) = JSONValue::parse(filter) else {
            debug!({ "Malformed object passed to Pagefind filters" });
            return None;
        };
        if !matches!(all_filters.value_type, J::Object) {
            debug!({ "Filters was passed a non-object" });
            return None;
        }

        self.dig_filter(all_filters)
    }
}
