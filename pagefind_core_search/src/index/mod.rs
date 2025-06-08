use crate::{CoreSearchIndex, PageWord};
use minicbor::{decode, Decoder};

/*
{} = fixed length array
{
    [
        {
            String,             // word
            [
                {
                    u32,        // page number
                    [
                        u32,    // page location
                        ...
                    ]
                },
                ...
            ]
        },
        ...
    ]
}
*/

impl CoreSearchIndex {
    pub fn decode_index_chunk(&mut self, index_bytes: &[u8]) -> Result<(), decode::Error> {
        let mut decoder = Decoder::new(index_bytes);

        // Consume fixed array marker
        decoder.array()?;

        // Read words array
        let words = match decoder.array()? {
            Some(n) => n,
            None => return Err(decode::Error::message("Array length not specified")),
        };

        for _ in 0..words {
            decoder.array()?;
            let key = decoder.str()?.to_owned();

            let pages = match decoder.array()? {
                Some(n) => n,
                None => return Err(decode::Error::message("Array length not specified")),
            };
            let mut page_arr = Vec::with_capacity(pages as usize);
            for _ in 0..pages {
                decoder.array()?;
                let mut page = PageWord {
                    page: decoder.u32()?,
                    locs: vec![],
                };

                let word_locations = match decoder.array()? {
                    Some(n) => n,
                    None => return Err(decode::Error::message("Array length not specified")),
                };
                let mut weight = 25;
                for _ in 0..word_locations {
                    let loc = decoder.i32()?;
                    // Negative numbers represent a change in the weighting of subsequent words.
                    if loc.is_negative() {
                        let abs_weight = (loc + 1) * -1;
                        weight = if abs_weight > 255 {
                            255
                        } else {
                            abs_weight.try_into().unwrap_or_default()
                        };
                    } else {
                        page.locs.push((weight, loc as u32));
                    }
                }

                page_arr.push(page);
            }

            self.words.insert(key, page_arr);
        }

        Ok(())
    }

    pub fn decode_filter_index_chunk(&mut self, filter_bytes: &[u8]) -> Result<(), decode::Error> {
        let mut decoder = Decoder::new(filter_bytes);

        decoder.array()?;

        let filter = decoder.str()?.to_owned();

        let values = match decoder.array()? {
            Some(n) => n,
            None => return Err(decode::Error::message("Array length not specified")),
        };

        let mut value_map = std::collections::BTreeMap::new();
        for _ in 0..values {
            decoder.array()?;
            let value = decoder.str()?.to_owned();

            let pages = match decoder.array()? {
                Some(n) => n,
                None => return Err(decode::Error::message("Array length not specified")),
            };
            let mut page_arr = Vec::with_capacity(pages as usize);
            for _ in 0..pages {
                page_arr.push(decoder.u32()?);
            }

            value_map.insert(value, page_arr);
        }

        self.filters.insert(filter, value_map);

        Ok(())
    }
}