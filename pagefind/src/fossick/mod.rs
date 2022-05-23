use hashbrown::HashMap;
use lazy_static::lazy_static;
use lol_html::html_content::ContentType;
use lol_html::{element, text, HtmlRewriter, Settings};
use regex::Regex;
use rust_stemmers::{Algorithm, Stemmer};
use sha1::{Digest, Sha1};
use std::borrow::Cow;
use std::cell::RefCell;
use std::io::Error;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::time::{sleep, Duration};

use crate::fragments::{PageFragment, PageFragmentData};
use crate::utils::full_hash;
use crate::SearchOptions;

lazy_static! {
    static ref NEWLINES: Regex = Regex::new("(\n|\r\n)+").unwrap();
    static ref TRIM_NEWLINES: Regex = Regex::new("^[\n\r\\s]+|[\n\r\\s]+$").unwrap();
    static ref EXTRANEOUS_SPACES: Regex = Regex::new("\\s{2,}").unwrap();
    static ref SENTENCE_CHARS: Regex = Regex::new("[\\w'\"\\)\\$\\*]").unwrap();
}
lazy_static! {
    static ref SENTENCE_SELECTORS: Vec<&'static str> =
        vec!("p", "td", "div", "ul", "article", "section");
    static ref LIST_SELECTORS: Vec<&'static str> = vec!("li");
    static ref REMOVE_SELECTORS: Vec<&'static str> =
        vec!("script", "noscript", "label", "form", "svg", "footer", "header", "nav", "iframe");
}

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

        let digest = Rc::new(RefCell::new(Vec::new()));
        let current_value: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));
        let mut title = None;
        let file_needs_write = false;

        let mut rewriter = HtmlRewriter::new(
            Settings {
                element_content_handlers: vec![
                    text!("body", |el| {
                        let mut current_value = current_value.borrow_mut();
                        match &mut *current_value {
                            Some(v) => v.push_str(el.as_str()),
                            None => {
                                let _ = current_value.insert(el.as_str().to_string());
                            }
                        };
                        Ok(())
                    }),
                    element!("body *", |el| {
                        let current_value = Rc::clone(&current_value);
                        let digest = Rc::clone(&digest);

                        // This will error if the element can not have an end tag
                        // We don't care about this,
                        // as that means it has no content for us anyway.
                        let _ = el.on_end_tag(move |end| {
                            let tag_name = end.name();
                            if REMOVE_SELECTORS.contains(&tag_name.as_str()) {
                                let _ = current_value.take();
                                return Ok(());
                            }
                            let mut current_value = current_value.borrow_mut();
                            if let Some(ref mut v) = &mut *current_value {
                                if v.chars()
                                    .last()
                                    .filter(|c| SENTENCE_CHARS.is_match(&c.to_string()))
                                    .is_some()
                                {
                                    if SENTENCE_SELECTORS.contains(&tag_name.as_str()) {
                                        v.push('.');
                                    } else if LIST_SELECTORS.contains(&tag_name.as_str()) {
                                        v.push(',');
                                    }
                                }
                            }
                            if current_value.is_some() {
                                digest.borrow_mut().push(current_value.take().unwrap());
                            }
                            Ok(())
                        });
                        Ok(())
                    }),
                    // Track the first h1 on the page as the title to return in search
                    // TODO: This doesn't handle a chunk boundary
                    text!("h1", |el| {
                        let text = normalize_content(el.as_str());
                        if title.is_none() && !text.is_empty() {
                            title = Some(text);
                        }
                        Ok(())
                    }),
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

        let strings = Rc::try_unwrap(digest).unwrap().into_inner();
        self.digest = normalize_content(&strings.join(" "));
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
        let special_chars = Regex::new("[^\\w]").unwrap(); // TODO: i18n?

        // TODO: Improve stop words in general
        let mut words_to_remove = stop_words::get(stop_words::LANGUAGE::English);
        words_to_remove.retain(|w| w.len() < 5);

        // TODO: Read newlines and jump the word_index up some amount,
        // so that separate bodies of text don't return exact string
        // matches across the boundaries.

        for (word_index, word) in self.digest.to_lowercase().split_whitespace().enumerate() {
            let mut word = special_chars.replace_all(word, "").into_owned();
            word = en_stemmer.stem(&word).into_owned();
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

        Ok(FossickedData {
            file_path: self.file_path.clone(),
            fragment: PageFragment {
                hash,
                page_number: 0,
                data: PageFragmentData {
                    url: build_url(&self.file_path, options),
                    title: self.title.clone(),
                    content: self.digest.clone(),
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
    let content = TRIM_NEWLINES.replace_all(content, "");
    let content = NEWLINES.replace_all(&content, " ");
    let content = EXTRANEOUS_SPACES.replace_all(&content, " ");

    content.to_string()
}
