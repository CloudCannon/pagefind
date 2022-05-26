use std::collections::HashMap;

use super::SearchIndex;
use crate::util::*;
use minicbor::{decode, Decoder};

/*
{} = fixed length array
{
    String,             // filter name
    [
        {
            String,     // filter value
            [
                u32     // page number
                ...
            ]
        },
        ...
    ]
}
*/

impl SearchIndex {
    pub fn decode_filter_index_chunk(&mut self, filter_bytes: &[u8]) -> Result<(), decode::Error> {
        debug!({ format!("Decoding {:#?} filter index bytes", filter_bytes.len()) });
        let mut decoder = Decoder::new(filter_bytes);

        consume_fixed_arr!(decoder);

        debug!({ "Reading filter name" });
        let filter = consume_string!(decoder);

        debug!({ "Reading values array" });
        let values = consume_arr_len!(decoder);

        debug!({ format!("Reading {:#?} values", values) });
        let mut value_map = HashMap::new();
        for _ in 0..values {
            consume_fixed_arr!(decoder);
            let value = consume_string!(decoder);

            let pages = consume_arr_len!(decoder);
            let mut page_arr = Vec::with_capacity(pages as usize);
            for _ in 0..pages {
                page_arr.push(consume_num!(decoder));
            }

            value_map.insert(value, page_arr);
        }

        self.filters.insert(filter, value_map);

        debug!({ "Finished reading values" });

        Ok(())
    }
}
