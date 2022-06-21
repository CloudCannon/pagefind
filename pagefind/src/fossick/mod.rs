use hashbrown::HashMap;
use regex::Regex;
use rust_stemmers::{Algorithm, Stemmer};
use std::io::Error;
use std::path::{Path, PathBuf};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::time::{sleep, Duration};

use crate::fragments::{PageFragment, PageFragmentData};
use crate::utils::full_hash;
use crate::SearchOptions;
use parser::DomParser;

use self::parser::DomParserResult;

mod parser;

#[derive(Debug)]
pub struct FossickedData {
    pub file_path: PathBuf,
    pub fragment: PageFragment,
    pub word_data: HashMap<String, Vec<u32>>,
    pub has_custom_body: bool,
}

#[derive(Debug)]
pub struct Fossicker {
    file_path: PathBuf,
    data: Option<DomParserResult>,
}

impl Fossicker {
    pub fn new(file_path: PathBuf) -> Self {
        Self {
            file_path,
            data: None,
        }
    }

    async fn read_file(&mut self) -> Result<(), Error> {
        let file = File::open(&self.file_path).await?;

        let mut rewriter = DomParser::new();

        let mut br = BufReader::new(file);
        let mut buf = [0; 20000];
        while let Ok(read) = br.read(&mut buf).await {
            if read == 0 {
                break;
            }
            if let Err(error) = rewriter.write(&buf[..read]) {
                panic!("HTML parse encountered an error: {:#?}", error);
            }
        }

        self.data = Some(rewriter.wrap());

        Ok(())
    }

    fn retrieve_words_from_digest(&mut self) -> HashMap<String, Vec<u32>> {
        let mut map: HashMap<String, Vec<u32>> = HashMap::new();
        let en_stemmer = Stemmer::create(Algorithm::English); // TODO: i18n
        let special_chars = Regex::new("[^\\w]").unwrap(); // TODO: i18n?

        // TODO: Reconsider stopwords. Removing them for now as they seem to remove too much,
        // let mut words_to_remove = stop_words::get(stop_words::LANGUAGE::English);
        // words_to_remove.retain(|w| w.len() < 5);

        // TODO: Read newlines and jump the word_index up some amount,
        // so that separate bodies of text don't return exact string
        // matches across the boundaries.

        for (word_index, word) in self
            .data
            .as_ref()
            .unwrap()
            .digest
            .to_lowercase()
            .split_whitespace()
            .enumerate()
        {
            let mut word = special_chars.replace_all(word, "").into_owned();
            word = en_stemmer.stem(&word).into_owned();
            // if words_to_remove.contains(&word) {
            //     continue; // Removing stopwords for now...
            // }
            if !word.is_empty() {
                if let Some(repeat) = map.get_mut(&word) {
                    repeat.push(word_index.try_into().unwrap());
                } else {
                    map.insert(word, vec![word_index.try_into().unwrap()]);
                }
            }
        }

        map
    }

    pub async fn fossick(&mut self, options: &SearchOptions) -> Result<FossickedData, ()> {
        while self.read_file().await.is_err() {
            sleep(Duration::from_millis(100)).await;
        }

        let word_data = self.retrieve_words_from_digest();

        let data = self.data.as_ref().unwrap();
        let hash = full_hash(data.digest.as_bytes());

        Ok(FossickedData {
            file_path: self.file_path.clone(),
            has_custom_body: data.has_custom_body,
            fragment: PageFragment {
                hash,
                page_number: 0,
                data: PageFragmentData {
                    url: build_url(&self.file_path, options),
                    content: data.digest.clone(),
                    filters: data.filters.clone(),
                    meta: data.meta.clone(),
                    word_count: word_data.len(),
                },
            },
            word_data,
        })
    }
}

fn build_url(page_url: &Path, options: &SearchOptions) -> String {
    let url = page_url
        .strip_prefix(&options.source)
        .expect("File was found that does not start with the source directory");

    format!(
        "/{}",
        url.to_str().unwrap().to_owned().replace("index.html", "")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn building_url() {
        let opts = SearchOptions {
            source: "hello/world".into(),
            ..Default::default()
        };

        let p: PathBuf = "hello/world/index.html".into();
        assert_eq!(&build_url(&p, &opts), "/");

        let p: PathBuf = "hello/world/about/index.html".into();
        assert_eq!(&build_url(&p, &opts), "/about/");

        let p: PathBuf = "hello/world/about.html".into();
        assert_eq!(&build_url(&p, &opts), "/about.html");
    }
}
