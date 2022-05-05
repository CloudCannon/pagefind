use hashbrown::HashMap;
use lol_html::html_content::ContentType;
use lol_html::{element, text, HtmlRewriter, Settings};
use regex::Regex;
use rust_stemmers::{Algorithm, Stemmer};
use sha1::{Digest, Sha1};
use std::borrow::Cow;
use std::cell::RefCell;
use std::io::Error;
use std::path::{Path, PathBuf};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::time::{sleep, Duration};

use crate::fragments::{PageFragment, PageFragmentData};
use crate::utils::full_hash;
use crate::SearchOptions;

pub struct FossickedData {
    pub file_path: PathBuf,
    pub fragment: PageFragment,
    pub word_data: HashMap<String, Vec<u32>>,
}

pub struct Fossicker {
    file_path: PathBuf,
    title: String,
    digest: String,
}

impl Fossicker {
    pub fn new(file_path: PathBuf) -> Self {
        Self {
            file_path,
            title: String::new(),
            digest: String::new(),
        }
    }

    async fn read_file(&mut self) -> Result<(), Error> {
        let file = File::open(&self.file_path).await?;

        let mut output = vec![];

        let digest = RefCell::new(Vec::new());
        let removals = RefCell::new(Vec::new());
        let mut title = None;
        let file_needs_write = false;

        let remove_selectors =
            "*script, *noscript, *label, *form, *svg, *footer, *header, *nav, *iframe"
                .replace("*", "[data-pagefind] ");

        let mut rewriter = HtmlRewriter::new(
            Settings {
                element_content_handlers: vec![
                    text!(
                        // "p, span, li, pre, code, blockquote, td, h1, h2, h3, h4, h5, h6",
                        "body",
                        |el| {
                            let text = el.as_str().to_string();
                            digest.borrow_mut().push(text);
                            Ok(())
                        }
                    ),
                    text!(remove_selectors, |el| {
                        // TODO: write some tests to ensure that these chunks always match
                        // 1:1 with the chunks in the text handler above,
                        // and will thus be removed.
                        // Especially for large elements that might not come through in one chunk.
                        let text = el.as_str().to_string();
                        removals.borrow_mut().push(text);
                        Ok(())
                    }),
                    text!("h1", |el| {
                        let text = normalize_content(el.as_str());
                        if title.is_none() && !text.is_empty() {
                            title = Some(text);
                        }
                        Ok(())
                    }),
                    // element!("head", |el| {
                    //     el.append("<script>alert(\"Hey pals\");</script>", ContentType::Html);
                    //     file_needs_write = true;

                    //     Ok(())
                    // }),
                ],
                ..Settings::default()
            },
            |c: &[u8]| output.extend_from_slice(c),
        );

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
        drop(rewriter);

        let removals = removals.into_inner();
        self.digest = digest
            .into_inner()
            .into_iter()
            .filter(|x| !removals.contains(x))
            .collect::<Vec<String>>()
            .join(" ");
        self.title = title.unwrap_or_default();

        if file_needs_write {
            let mut outfile = File::create(&self.file_path).await;
            while outfile.is_err() {
                sleep(Duration::from_millis(100)).await;
                outfile = File::create(&self.file_path).await;
            }

            outfile.unwrap().write_all(&output).await.unwrap();
        }

        Ok(())
    }

    fn retrieve_words_from_digest(&mut self) -> HashMap<String, Vec<u32>> {
        let mut map: HashMap<String, Vec<u32>> = HashMap::new();
        let en_stemmer = Stemmer::create(Algorithm::English);
        let special_chars = Regex::new("[^\\w\\s]").unwrap(); // TODO: i18n?

        // TODO: Improve stop words in general
        let mut words_to_remove = stop_words::get(stop_words::LANGUAGE::English);
        words_to_remove.retain(|w| w.len() < 5);

        let base_content = self.digest.to_lowercase().replace('\'', "");
        let raw_content = special_chars.replace_all(&base_content, " ");

        // TODO: Read newlines and jump the word_index up some amount,
        // so that separate bodies of text don't return exact string
        // matches across the boundaries.

        for (word_index, word) in raw_content.split_whitespace().enumerate() {
            let word = en_stemmer.stem(word).into_owned();
            if words_to_remove.contains(&word) {
                continue;
            }
            if let Some(repeat) = map.get_mut(&word) {
                repeat.push(word_index.try_into().unwrap());
            } else {
                map.insert(word, vec![word_index.try_into().unwrap()]);
            }
        }

        map
    }

    pub async fn fossick(&mut self, options: &SearchOptions) -> Result<FossickedData, ()> {
        while self.read_file().await.is_err() {
            sleep(Duration::from_millis(100)).await;
        }

        let word_data = self.retrieve_words_from_digest();
        let hash = full_hash(self.digest.as_bytes());
        let content = normalize_content(&self.digest);

        Ok(FossickedData {
            file_path: self.file_path.clone(),
            fragment: PageFragment {
                hash,
                page_number: 0,
                data: PageFragmentData {
                    url: build_url(&self.file_path, options),
                    title: self.title.clone(),
                    content,
                    attributes: HashMap::new(),
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

fn normalize_content(content: &str) -> String {
    let extraneous_newlines = Regex::new("(^|\\s)*((\n|\r\n)\\s*)+($|\\s)*").unwrap();
    let trim_newlines = Regex::new("^\n|\n$").unwrap();
    let extraneous_spaces = Regex::new("\\s{2,}").unwrap();

    let content = extraneous_newlines.replace_all(content, "\n");
    let content = trim_newlines.replace_all(&content, "");
    let content = extraneous_spaces.replace_all(&content, " ");

    content.to_string()
}
