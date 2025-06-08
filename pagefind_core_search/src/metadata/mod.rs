use crate::{CoreSearchIndex, IndexChunk, Page};
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

impl CoreSearchIndex {
    pub fn decode_metadata(&mut self, metadata_bytes: &[u8]) -> Result<(), decode::Error> {
        let mut decoder = Decoder::new(metadata_bytes);

        decoder.array()?;

        // Read version number
        self.generator_version = Some(decoder.str()?.to_owned());

        // Read pages array
        let page_hashes = match decoder.array()? {
            Some(n) => n,
            None => return Err(decode::Error::message("Array length not specified")),
        };
        self.pages = Vec::with_capacity(page_hashes as usize);
        for _ in 0..page_hashes {
            decoder.array()?;
            self.pages.push(Page {
                hash: decoder.str()?.to_owned(),
                word_count: decoder.u32()?,
            });
        }

        if !self.pages.is_empty() {
            self.average_page_length = self.pages.iter().map(|p| p.word_count as f32).sum::<f32>()
                / self.pages.len() as f32;
        }

        // Read index chunks array
        let index_chunks = match decoder.array()? {
            Some(n) => n,
            None => return Err(decode::Error::message("Array length not specified")),
        };
        self.chunks = Vec::with_capacity(index_chunks as usize);
        for _ in 0..index_chunks {
            decoder.array()?;
            self.chunks.push(IndexChunk {
                from: decoder.str()?.to_owned(),
                to: decoder.str()?.to_owned(),
                hash: decoder.str()?.to_owned(),
            })
        }

        // Read filter chunks array
        let filter_chunks = match decoder.array()? {
            Some(n) => n,
            None => return Err(decode::Error::message("Array length not specified")),
        };
        for _ in 0..filter_chunks {
            decoder.array()?;
            self.filter_chunks
                .insert(decoder.str()?.to_owned(), decoder.str()?.to_owned());
        }

        // Read sorts array
        let sorts = match decoder.array()? {
            Some(n) => n,
            None => return Err(decode::Error::message("Array length not specified")),
        };
        for _ in 0..sorts {
            decoder.array()?;
            let sort_key = decoder.str()?.to_owned();

            let page_num_num = match decoder.array()? {
                Some(n) => n,
                None => return Err(decode::Error::message("Array length not specified")),
            };
            let mut sorted_pages = Vec::with_capacity(page_num_num as usize);
            for _ in 0..page_num_num {
                sorted_pages.push(decoder.u32()?);
            }

            self.sorts.insert(sort_key, sorted_pages);
        }

        Ok(())
    }
}