use super::{IndexChunk, SearchIndex};
use crate::util::*;
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

        debug!({ "Reading page hashes array" });
        let page_hashes = consume_arr_len!(decoder);
        debug!({ format!("Reading {:#?} page hashes", page_hashes) });
        self.pages = Vec::with_capacity(page_hashes as usize);
        for _ in 0..page_hashes {
            self.pages.push(consume_string!(decoder));
        }

        debug!({ "Reading stop words array" });
        let stop_words = consume_arr_len!(decoder);
        debug!({ format!("Reading {:#?} stop words", stop_words) });
        self.stops = Vec::with_capacity(stop_words as usize);
        for _ in 0..stop_words {
            self.stops.push(consume_string!(decoder));
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

        debug!({ "Finished decoding metadata" });

        Ok(())
    }
}
