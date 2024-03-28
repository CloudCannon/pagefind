use super::{IndexChunk, SearchIndex};
use crate::{util::*, Page};
use minicbor::{decode, Decoder};

/*
{} = fixed length array
{
    String,                 // pagefind generator version
    [
        {
            String,         // page hash
            u32,            // word count
        }
        ...
    ]
    [
        {
            String,         // start word of index chunk
            String,         // end word of index chunk
            String,         // hash of index chunk
        },
        ...
    ],
    [
        {
            String,         // value of filter chunk
            String,         // hash of filter chunk
        },
        ...
    ],
    [
        {
            String,         // sort key
            [ usize, ... ], // sorted page numbers
        }
    ]
}
*/

impl SearchIndex {
    pub fn decode_metadata(&mut self, metadata_bytes: &[u8]) -> Result<(), decode::Error> {
        debug!({ format!("Decoding {:#?} metadata bytes", metadata_bytes.len()) });
        let mut decoder = Decoder::new(metadata_bytes);

        consume_fixed_arr!(decoder);

        debug!({ "Reading version number" });
        self.generator_version = Some(consume_string!(decoder));

        debug!({ "Reading pages array" });
        let page_hashes = consume_arr_len!(decoder);
        debug!({ format!("Reading {:#?} pages", page_hashes) });
        self.pages = Vec::with_capacity(page_hashes as usize);
        for _ in 0..page_hashes {
            consume_fixed_arr!(decoder);
            self.pages.push(Page {
                hash: consume_string!(decoder),
                word_count: consume_num!(decoder),
            });
        }

        if !self.pages.is_empty() {
            self.average_page_length = self.pages.iter().map(|p| p.word_count as f32).sum::<f32>()
                / self.pages.len() as f32;
        }

        debug!({ "Reading index chunks array" });
        let index_chunks = consume_arr_len!(decoder);
        debug!({ format!("Reading {:#?} index chunks", index_chunks) });
        self.chunks = Vec::with_capacity(index_chunks as usize);
        for _ in 0..index_chunks {
            consume_fixed_arr!(decoder);
            self.chunks.push(IndexChunk {
                from: consume_string!(decoder),
                to: consume_string!(decoder),
                hash: consume_string!(decoder),
            })
        }

        debug!({ "Reading filter chunks array" });
        let filter_chunks = consume_arr_len!(decoder);
        debug!({ format!("Reading {:#?} filter chunks", filter_chunks) });
        for _ in 0..filter_chunks {
            consume_fixed_arr!(decoder);
            self.filter_chunks
                .insert(consume_string!(decoder), consume_string!(decoder));
        }

        debug!({ "Reading sorts array" });
        let sorts = consume_arr_len!(decoder);
        debug!({ format!("Reading {:#?} sorts", sorts) });
        for _ in 0..sorts {
            consume_fixed_arr!(decoder);
            let sort_key = consume_string!(decoder);

            debug!({ format!("Reading array of page numbers sorted by {:#?}", sort_key) });
            let page_num_num = consume_arr_len!(decoder);
            debug!({ format!("Reading {:#?} page numbers", page_num_num) });
            let mut sorted_pages = Vec::with_capacity(page_num_num as usize);
            for _ in 0..page_num_num {
                sorted_pages.push(consume_num!(decoder));
            }

            self.sorts.insert(sort_key, sorted_pages);
        }

        debug!({ "Finished decoding metadata" });

        Ok(())
    }
}
