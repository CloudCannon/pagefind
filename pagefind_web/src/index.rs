use super::{PageWord, SearchIndex};
use crate::util::*;
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

impl SearchIndex {
    pub fn decode_index_chunk(&mut self, index_bytes: &[u8]) -> Result<(), decode::Error> {
        debug!({ format!("Decoding {:#?} index bytes", index_bytes.len()) });
        let mut decoder = Decoder::new(index_bytes);

        consume_fixed_arr!(decoder);

        debug!({ "Reading words array" });
        let words = consume_arr_len!(decoder);
        debug!({ format!("Reading {:#?} words", words) });
        for _ in 0..words {
            consume_fixed_arr!(decoder);
            let key = consume_string!(decoder);

            let pages = consume_arr_len!(decoder);
            let mut page_arr = Vec::with_capacity(pages as usize);
            for _ in 0..pages {
                consume_fixed_arr!(decoder);
                let mut page = PageWord {
                    page: consume_num!(decoder),
                    locs: vec![],
                };

                let word_locations = consume_arr_len!(decoder);
                let mut weight = 1;
                for _ in 0..word_locations {
                    let loc = consume_inum!(decoder);
                    // Negative numbers represent a change in the weighting of subsequent words.
                    if loc.is_negative() {
                        let abs_weight = loc * -1 + 1;
                        weight = if abs_weight > 255 {
                            255
                        } else {
                            abs_weight.try_into().unwrap_or_default()
                        };
                        debug!({
                            format!("Encountered word position {loc:#?}, weighting subsequent words as {weight:#?}")
                        });
                    } else {
                        page.locs.push((weight, loc as u32));
                    }
                }

                page_arr.push(page);
            }

            self.words.insert(key, page_arr);
        }
        debug!({ "Finished reading words" });

        Ok(())
    }
}
