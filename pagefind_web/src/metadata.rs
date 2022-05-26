use super::{IndexChunk, SearchIndex};
use crate::{util::*, Page};
use minicbor::{decode, Decoder};

/*
{} = fixed length array
{
    String,                 // pagefind generator version
    [ String, ... ],        // ordered page hashes
    [ String, ... ],        // stop words
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

        debug!({ "Finished decoding metadata" });

        Ok(())
    }
}
