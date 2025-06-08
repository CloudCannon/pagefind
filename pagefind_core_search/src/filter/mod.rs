use bit_set::BitSet;
use pagefind_microjson::JSONValue;
use pagefind_microjson::JSONValueType;

use crate::CoreSearchIndex;

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

impl CoreSearchIndex {
    pub fn get_filters(&self, obj: &mut write_json::Object, intersect_pages: Option<Vec<usize>>) {
        let intersect_pages = intersect_pages.as_ref();
        for (filter, values) in &self.filters {
            let mut filter_obj = obj.object(filter);

            for (value, pages) in values {
                let len = match intersect_pages {
                    Some(intersection) => pages
                        .iter()
                        .filter(|p| intersection.contains(&(**p as usize)))
                        .count(),
                    None => pages.len(),
                };
                filter_obj.number(value, len as f64);
            }
        }
    }

    fn invert(&self, set: &mut BitSet) {
        set.symmetric_difference_with(&BitSet::<u32>::from_iter(0..self.pages.len()));
    }

    fn parse_filter_value(
        &self,
        filter_key: &str,
        filter: JSONValue,
        behaviour: FilterBehaviour,
    ) -> Option<BitSet> {
        use JSONValueType as J;

        let filter_map = self.filters.get(filter_key);

        let mut maps = Vec::new();
        let build_set = |val: JSONValue| {
            if let Some(Some(filter_pages)) =
                filter_map.map(|m| m.get(val.read_string().unwrap_or_default()))
            {
                let mut set = BitSet::new();
                for page in filter_pages {
                    set.insert(*page as usize);
                }
                set
            } else {
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
                                if let Some(inner_set) =
                                    self.parse_filter_value(filter_key, value, FilterBehaviour::All)
                                {
                                    maps.push(inner_set);
                                }
                            }
                            _ => continue,
                        }
                    }
                }
            }
            JSONValueType::Object => {
                if let Ok(obj) = filter.iter_object() {
                    for (k, value) in obj.filter_map(|o| o.ok()) {
                        if let Some(Some(mut map)) = match (k, value.value_type) {
                            ("any" | "none", J::Object | J::Array | J::String) => Some(
                                self.parse_filter_value(filter_key, value, FilterBehaviour::Any),
                            ),
                            ("all" | "not", J::Object | J::Array | J::String) => Some(
                                self.parse_filter_value(filter_key, value, FilterBehaviour::All),
                            ),
                            _ => None,
                        } {
                            if matches!(k, "none" | "not") {
                                self.invert(&mut map);
                            }
                            maps.push(map);
                        }
                    }
                }
            }
            _ => {
                return None;
            }
        }

        if maps.is_empty() {
            None
        } else {
            Some(collapse(maps, behaviour))
        }
    }

    fn parse_filter_arr(&self, filter: JSONValue, behaviour: FilterBehaviour) -> Option<BitSet> {
        use JSONValueType as J;
        debug_assert!(matches!(filter.value_type, J::Array));

        let mut maps = Vec::new();

        if let Ok(arr) = filter.iter_array() {
            for value in arr {
                if !matches!(value.value_type, J::Object) {
                    continue;
                }
                if let Some(map) = self.parse_filter_obj(value, FilterBehaviour::All) {
                    maps.push(map)
                }
            }
        }

        if maps.is_empty() {
            None
        } else {
            Some(collapse(maps, behaviour))
        }
    }

    fn parse_filter_obj(&self, filter: JSONValue, behaviour: FilterBehaviour) -> Option<BitSet> {
        use JSONValueType as J;
        debug_assert!(matches!(filter.value_type, J::Object));

        let mut maps = Vec::new();

        if let Ok(obj) = filter.iter_object() {
            for (k, value) in obj.filter_map(|o| o.ok()) {
                let map = match (k, value.value_type) {
                    ("any" | "none", J::Object) => {
                        self.parse_filter_obj(value, FilterBehaviour::Any)
                    }
                    ("all" | "not", J::Object) => {
                        self.parse_filter_obj(value, FilterBehaviour::All)
                    }
                    ("any" | "none", J::Array) => {
                        self.parse_filter_arr(value, FilterBehaviour::Any)
                    }
                    ("all" | "not", J::Array) => self.parse_filter_arr(value, FilterBehaviour::All),
                    (k, _) => self.parse_filter_value(k, value, FilterBehaviour::All),
                };
                if let Some(mut map) = map {
                    if matches!(k, "none" | "not") {
                        self.invert(&mut map);
                    }
                    maps.push(map);
                }
            }
        }

        if maps.is_empty() {
            None
        } else {
            Some(collapse(maps, behaviour))
        }
    }

    pub fn filter(&self, filter: &str) -> Option<BitSet> {
        use JSONValueType as J;

        if self.filter_chunks(filter).is_none() {
            return None;
        }

        let Ok(all_filters) = JSONValue::parse(filter) else {
            return None;
        };
        if !matches!(all_filters.value_type, J::Object) {
            return None;
        }

        self.parse_filter_obj(all_filters, FilterBehaviour::All)
    }

    pub fn dig_filter(&self, filter: JSONValue) -> Option<Vec<String>> {
        use JSONValueType as J;

        let mut has_filters = false;
        let mut indexes = Vec::new();

        match filter.value_type {
            J::Object => {
                if let Ok(obj) = filter.iter_object() {
                    for (k, value) in obj.filter_map(|o| o.ok()) {
                        match (k, value.value_type) {
                            ("any" | "all" | "not" | "none", J::Object | J::Array) => {
                                if let Some(nested_filters) = self.dig_filter(value) {
                                    has_filters = true;
                                    indexes.extend(nested_filters)
                                }
                            }
                            (filter, _) => {
                                has_filters = true;
                                if let Some(hash) = self.filter_chunks.get(filter) {
                                    indexes.push(hash.clone());
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
        use JSONValueType as J;

        let Ok(all_filters) = JSONValue::parse(filter) else {
            return None;
        };
        if !matches!(all_filters.value_type, J::Object) {
            return None;
        }

        self.dig_filter(all_filters)
    }

    // Used to parse one-off filters that were generated by the JS API, not the Rust CLI
    pub fn decode_synthetic_filter(&mut self, filter: &str) {
        use JSONValueType as J;

        let Ok(all_filters) = JSONValue::parse(filter) else {
            return;
        };
        if !matches!(all_filters.value_type, J::Object) {
            return;
        }

        let all_pages = Vec::from_iter(0..self.pages.len() as u32);

        if let Ok(obj) = all_filters.iter_object() {
            for (filter_name, value) in obj.filter_map(|o| o.ok()) {
                if !self.filters.contains_key(filter_name) {
                    let filter_map = std::collections::BTreeMap::new();
                    self.filters.insert(filter_name.to_string(), filter_map);
                }

                let filter_map = self
                    .filters
                    .get_mut(filter_name)
                    .expect("Filter should have just been created");

                match value.value_type {
                    J::String => {
                        filter_map
                            .insert(value.read_string().unwrap().to_string(), all_pages.clone());
                    }
                    J::Array => {
                        for value in value.iter_array().unwrap() {
                            if !matches!(value.value_type, J::String) {
                                continue;
                            }
                            filter_map.insert(
                                value.read_string().unwrap().to_string(),
                                all_pages.clone(),
                            );
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}