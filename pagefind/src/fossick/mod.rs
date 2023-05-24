use async_compression::tokio::bufread::GzipDecoder;
#[cfg(feature = "extended")]
use charabia::Segment;
use hashbrown::HashMap;
use lazy_static::lazy_static;
use pagefind_stem::{Algorithm, Stemmer};
use path_slash::PathExt as _;
use regex::Regex;
use std::io::Error;
use std::path::{Path, PathBuf};
use tokio::fs::File;
use tokio::io::AsyncBufReadExt;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::time::{sleep, Duration};

use crate::fragments::{PageAnchorData, PageFragment, PageFragmentData};
use crate::SearchOptions;
use parser::DomParser;

use self::parser::DomParserResult;

lazy_static! {
    // TODO: i18n?
    static ref SPECIAL_CHARS: Regex = Regex::new("[^\\w]").unwrap();
}

pub mod parser;

#[derive(Debug, Clone, PartialEq)]
pub struct FossickedWord {
    pub position: u32,
    pub weight: u8,
}

#[derive(Debug, Clone)]
pub struct FossickedData {
    pub url: String,
    pub fragment: PageFragment,
    pub word_data: HashMap<String, Vec<FossickedWord>>,
    pub sort: HashMap<String, String>,
    pub has_custom_body: bool,
    pub force_inclusion: bool,
    pub has_html_element: bool,
    pub language: String,
}

#[derive(Debug)]
pub struct Fossicker {
    file_path: Option<PathBuf>,
    page_url: Option<String>,
    synthetic_content: Option<String>,
    data: Option<DomParserResult>,
}

impl Fossicker {
    pub fn new(file_path: PathBuf) -> Self {
        Self {
            file_path: Some(file_path),
            page_url: None,
            synthetic_content: None,
            data: None,
        }
    }

    pub fn new_synthetic(file_path: PathBuf, contents: String) -> Self {
        Self {
            file_path: Some(file_path),
            page_url: None,
            synthetic_content: Some(contents),
            data: None,
        }
    }

    pub fn new_with_data(url: String, data: DomParserResult) -> Self {
        Self {
            file_path: None,
            page_url: Some(url),
            synthetic_content: None,
            data: Some(data),
        }
    }

    async fn read_file(&mut self, options: &SearchOptions) -> Result<(), Error> {
        let Some(file_path) = &self.file_path else { return Ok(()) }; // TODO: Change to thiserror
        let file = File::open(file_path).await?;

        let mut rewriter = DomParser::new(options);

        let mut br = BufReader::new(file);
        let mut buf = [0; 20000];

        let is_gzip = if let Ok(read) = br.fill_buf().await {
            read.len() >= 3 && read[0] == 0x1F && read[1] == 0x8B && read[2] == 0x08
        } else {
            false
        };

        if is_gzip {
            let mut br = GzipDecoder::new(br);
            while let Ok(read) = br.read(&mut buf).await {
                if read == 0 {
                    break;
                }
                if let Err(error) = rewriter.write(&buf[..read]) {
                    println!(
                        "Failed to parse file {} — skipping this file. Error:\n{error}",
                        file_path.to_str().unwrap_or("[unknown file]")
                    );
                    return Ok(());
                }
            }
        } else {
            while let Ok(read) = br.read(&mut buf).await {
                if read == 0 {
                    break;
                }
                if let Err(error) = rewriter.write(&buf[..read]) {
                    println!(
                        "Failed to parse file {} — skipping this file. Error:\n{error}",
                        file_path.to_str().unwrap_or("[unknown file]")
                    );
                    return Ok(());
                }
            }
        }

        let mut data = rewriter.wrap();
        if let Some(forced_language) = &options.force_language {
            data.language = forced_language.clone();
        }

        self.data = Some(data);

        Ok(())
    }

    async fn read_synthetic(&mut self, options: &SearchOptions) -> Result<(), Error> {
        let Some(file_path) = &self.file_path else { return Ok(()) }; // TODO: Change to thiserror
        let Some(contents) = self.synthetic_content.as_ref() else { return Ok(()) };

        let mut rewriter = DomParser::new(options);

        let mut br = BufReader::new(contents.as_bytes());
        let mut buf = [0; 20000];

        while let Ok(read) = br.read(&mut buf).await {
            if read == 0 {
                break;
            }
            if let Err(error) = rewriter.write(&buf[..read]) {
                println!(
                    "Failed to parse file {} — skipping this file. Error:\n{error}",
                    file_path.to_str().unwrap_or("[unknown file]")
                );
                return Ok(());
            }
        }

        let mut data = rewriter.wrap();
        if let Some(forced_language) = &options.force_language {
            data.language = forced_language.clone();
        }

        self.data = Some(data);

        Ok(())
    }

    fn parse_digest(
        &mut self,
    ) -> (
        String,
        HashMap<String, Vec<FossickedWord>>,
        Vec<(String, String, u32)>,
    ) {
        let mut map: HashMap<String, Vec<FossickedWord>> = HashMap::new();
        let mut anchors = Vec::new();
        // TODO: push this error handling up a level and return an Err from parse_digest
        if self.data.as_ref().is_none() {
            return ("".into(), map, anchors); // empty page result, will be dropped from search
        }
        let data = self.data.as_ref().unwrap();
        let stemmer = get_stemmer(&data.language);

        let mut content = String::with_capacity(data.digest.len());

        // TODO: Consider reading newlines and jump the word_index up some amount,
        // so that separate bodies of text don't return exact string
        // matches across the boundaries. Or otherwise use some marker byte for the boundary.

        // TODO: Configure this or use segmenting across all languages

        #[cfg(feature = "extended")]
        let should_segment = matches!(data.language.split('-').next().unwrap(), "zh" | "ja");

        #[cfg(feature = "extended")]
        let segments = if should_segment {
            // Run a segmenter only for any languages which require it.
            data.digest.as_str().segment_str().collect::<Vec<_>>()
        } else {
            // Currently hesistant to run segmentation during indexing
            // that we can't also run during search, since we don't
            // ship a segmenter to the browser. This logic is easier
            // to replicate in the JavaScript that parses a search query.
            data.digest.split_whitespace().collect::<Vec<_>>()
        };

        #[cfg(not(feature = "extended"))]
        let segments = data.digest.split_whitespace();

        let mut offset_word_index = 0;
        let mut weight_stack: Vec<u8> = vec![1];
        for (word_index, word) in segments.into_iter().enumerate() {
            let word_index = word_index - offset_word_index;

            if word.chars().next() == Some('_') {
                if word.starts_with("___PAGEFIND_ANCHOR___") {
                    if let Some((element_name, element_id)) =
                        word.replace("___PAGEFIND_ANCHOR___", "").split_once(':')
                    {
                        anchors.push((
                            element_name.to_string(),
                            element_id.to_string(),
                            word_index as u32,
                        ));
                    }
                    offset_word_index += 1;
                    continue;
                }

                if word.starts_with("___PAGEFIND_WEIGHT___") {
                    let weight = word
                        .replace("___PAGEFIND_WEIGHT___", "")
                        .parse::<u32>()
                        .ok()
                        .unwrap_or(1);
                    weight_stack.push(weight.try_into().unwrap_or(std::u8::MAX));
                    offset_word_index += 1;
                    continue;
                }

                // Auto weights are provided by the parser, and should only
                // apply if we aren't inside an explicitly weighted block,
                // in which case we should just inherit that weight.
                if word.starts_with("___PAGEFIND_AUTO_WEIGHT___") {
                    if weight_stack.len() == 1 {
                        let weight = word
                            .replace("___PAGEFIND_AUTO_WEIGHT___", "")
                            .parse::<u32>()
                            .ok()
                            .unwrap_or(1);
                        weight_stack.push(weight.try_into().unwrap_or(std::u8::MAX));
                    } else {
                        weight_stack.push(weight_stack.last().cloned().unwrap_or_default());
                    }
                    offset_word_index += 1;
                    continue;
                }

                if word.starts_with("___END_PAGEFIND_WEIGHT___") {
                    weight_stack.pop();
                    offset_word_index += 1;
                    continue;
                }
            }

            let word_weight = weight_stack.last().unwrap_or(&1);

            content.push_str(&word.replace('\u{200B}', ""));
            content.push(' ');

            let mut normalized_word = SPECIAL_CHARS
                .replace_all(word, "")
                .into_owned()
                .to_lowercase();

            #[cfg(feature = "extended")]
            if should_segment {
                content.push('\u{200B}');
            }

            if !normalized_word.is_empty() {
                if let Some(stemmer) = &stemmer {
                    normalized_word = stemmer.stem(&normalized_word).into_owned();
                }

                let entry = FossickedWord {
                    position: word_index.try_into().unwrap(),
                    weight: *word_weight,
                };
                if let Some(repeat) = map.get_mut(&normalized_word) {
                    repeat.push(entry);
                } else {
                    map.insert(normalized_word, vec![entry]);
                }
            }
        }
        if content.ends_with(' ') {
            content.pop();
        }
        (content, map, anchors)
    }

    async fn fossick_html(&mut self, options: &SearchOptions) {
        if self.synthetic_content.is_some() {
            while self.read_synthetic(options).await.is_err() {
                sleep(Duration::from_millis(1)).await;
            }
        } else {
            while self.read_file(options).await.is_err() {
                sleep(Duration::from_millis(1)).await;
            }
        }
    }

    pub async fn fossick(mut self, options: &SearchOptions) -> Result<FossickedData, ()> {
        if self.file_path.is_some() && self.data.is_none() {
            self.fossick_html(options).await;
        };

        let (content, word_data, anchors) = self.parse_digest();

        let data = self.data.unwrap();
        let url = if let Some(url) = &self.page_url {
            url.clone()
        } else if let Some(path) = &self.file_path {
            build_url(path, options)
        } else {
            options
                .logger
                .error("Tried to index file with no specified URL or file path, ignoring.");
            return Err(());
        };

        Ok(FossickedData {
            url: url.clone(),
            has_custom_body: data.has_custom_body,
            force_inclusion: data.force_inclusion,
            has_html_element: data.has_html_element,
            language: data.language,
            fragment: PageFragment {
                page_number: 0, // This page number is updated later once determined
                data: PageFragmentData {
                    url,
                    content,
                    filters: data.filters,
                    meta: data.meta,
                    word_count: word_data.len(),
                    anchors: anchors
                        .into_iter()
                        .map(|(element, id, location)| PageAnchorData {
                            element,
                            id,
                            location,
                            text: None,
                        })
                        .collect(),
                },
            },
            word_data,
            sort: data.sort,
        })
    }
}

fn build_url(page_url: &Path, options: &SearchOptions) -> String {
    let trimmed = page_url.strip_prefix(&options.source);
    let Ok(url) = trimmed else {
        options.logger.error(format!(
            "File was found that does not start with the source directory: {}\nSource: {:?}\nFile: {:?}",
            trimmed.err().unwrap(),
            options.source,
            page_url
        ));
        return "/unknown/".to_string();
    };

    let final_url: String = if !options.keep_index_url {
        url.to_slash_lossy().to_owned().replace("index.html", "")
    } else {
        url.to_slash_lossy().to_owned().to_string()
    };

    format!("/{}", final_url)
}

// TODO: These language codes are duplicated with pagefind_web's Cargo.toml
fn get_stemmer(lang: &str) -> Option<Stemmer> {
    match lang.split('-').next().unwrap() {
        "ar" => Some(Stemmer::create(Algorithm::Arabic)),
        "hy" => Some(Stemmer::create(Algorithm::Armenian)),
        "eu" => Some(Stemmer::create(Algorithm::Basque)),
        "ca" => Some(Stemmer::create(Algorithm::Catalan)),
        "da" => Some(Stemmer::create(Algorithm::Danish)),
        "nl" => Some(Stemmer::create(Algorithm::Dutch)),
        "en" => Some(Stemmer::create(Algorithm::English)),
        "fi" => Some(Stemmer::create(Algorithm::Finnish)),
        "fr" => Some(Stemmer::create(Algorithm::French)),
        "de" => Some(Stemmer::create(Algorithm::German)),
        "el" => Some(Stemmer::create(Algorithm::Greek)),
        "hi" => Some(Stemmer::create(Algorithm::Hindi)),
        "hu" => Some(Stemmer::create(Algorithm::Hungarian)),
        "id" => Some(Stemmer::create(Algorithm::Indonesian)),
        "ga" => Some(Stemmer::create(Algorithm::Irish)),
        "it" => Some(Stemmer::create(Algorithm::Italian)),
        "lt" => Some(Stemmer::create(Algorithm::Lithuanian)),
        "ne" => Some(Stemmer::create(Algorithm::Nepali)),
        "no" => Some(Stemmer::create(Algorithm::Norwegian)),
        "pt" => Some(Stemmer::create(Algorithm::Portuguese)),
        "ro" => Some(Stemmer::create(Algorithm::Romanian)),
        "ru" => Some(Stemmer::create(Algorithm::Russian)),
        "sr" => Some(Stemmer::create(Algorithm::Serbian)),
        "es" => Some(Stemmer::create(Algorithm::Spanish)),
        "sv" => Some(Stemmer::create(Algorithm::Swedish)),
        "ta" => Some(Stemmer::create(Algorithm::Tamil)),
        "tr" => Some(Stemmer::create(Algorithm::Turkish)),
        "yi" => Some(Stemmer::create(Algorithm::Yiddish)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::PagefindInboundConfig;
    use twelf::Layer;

    use super::*;

    async fn test_fossick(s: String) -> Fossicker {
        std::env::set_var("PAGEFIND_SOURCE", "somewhere");
        let config =
            PagefindInboundConfig::with_layers(&[Layer::Env(Some("PAGEFIND_".into()))]).unwrap();
        let opts = SearchOptions::load(config).unwrap();

        let mut f = Fossicker {
            file_path: Some("test/index.html".into()),
            page_url: Some("/test/".into()),
            synthetic_content: Some(s),
            data: None,
        };

        _ = f.read_synthetic(&opts).await;

        f
    }

    #[tokio::test]
    async fn parse_file() {
        let mut f =
            test_fossick(["<html><body>", "<p>Hello World!</p>", "</body></html>"].concat()).await;

        let (digest, words, anchors) = f.parse_digest();

        assert_eq!(digest, "Hello World!".to_string());
        assert_eq!(
            words,
            HashMap::from_iter([
                (
                    "hello".to_string(),
                    vec![FossickedWord {
                        position: 0,
                        weight: 1
                    }]
                ),
                (
                    "world".to_string(),
                    vec![FossickedWord {
                        position: 1,
                        weight: 1
                    }]
                )
            ])
        );
    }

    #[tokio::test]
    async fn parse_weighted_file() {
        let mut f = test_fossick(
            [
                "<html><body>",
                "<div>The",
                "<p data-pagefind-weight='2'>Quick Brown</p>",
                "Fox</div>",
                "</body></html>",
            ]
            .concat(),
        )
        .await;

        let (digest, words, anchors) = f.parse_digest();

        assert_eq!(digest, "The Quick Brown. Fox.".to_string());
        assert_eq!(
            words,
            HashMap::from_iter([
                (
                    "the".to_string(),
                    vec![FossickedWord {
                        position: 0,
                        weight: 1
                    }]
                ),
                (
                    "quick".to_string(),
                    vec![FossickedWord {
                        position: 1,
                        weight: 2
                    }]
                ),
                (
                    "brown".to_string(),
                    vec![FossickedWord {
                        position: 2,
                        weight: 2
                    }]
                ),
                (
                    "fox".to_string(),
                    vec![FossickedWord {
                        position: 3,
                        weight: 1
                    }]
                )
            ])
        );
    }

    #[tokio::test]
    async fn parse_auto_weighted_file() {
        let mut f = test_fossick(
            [
                "<html><body>",
                "<h1>Pagefind</h1>",
                "<h2>Pagefind</h2>",
                "<h3>Pagefind</h3>",
                "<h4>Pagefind</h4>",
                "<h5>Pagefind</h5>",
                "<h6>Pagefind</h6>",
                "<p>Pagefind</p>",
                "</body></html>",
            ]
            .concat(),
        )
        .await;

        let (digest, words, anchors) = f.parse_digest();

        assert_eq!(
            words,
            HashMap::from_iter([(
                "pagefind".to_string(),
                vec![
                    FossickedWord {
                        position: 0,
                        weight: 7
                    },
                    FossickedWord {
                        position: 1,
                        weight: 6
                    },
                    FossickedWord {
                        position: 2,
                        weight: 5
                    },
                    FossickedWord {
                        position: 3,
                        weight: 4
                    },
                    FossickedWord {
                        position: 4,
                        weight: 3
                    },
                    FossickedWord {
                        position: 5,
                        weight: 2
                    },
                    FossickedWord {
                        position: 6,
                        weight: 1
                    }
                ]
            )])
        );
    }

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn building_url() {
        std::env::set_var("PAGEFIND_SOURCE", "hello/world");
        let config =
            PagefindInboundConfig::with_layers(&[Layer::Env(Some("PAGEFIND_".into()))]).unwrap();
        let opts = SearchOptions::load(config).unwrap();

        let p: PathBuf = "hello/world/index.html".into();
        assert_eq!(&build_url(&p, &opts), "/");

        let p: PathBuf = "hello/world/about/index.html".into();
        assert_eq!(&build_url(&p, &opts), "/about/");

        let p: PathBuf = "hello/world/about.html".into();
        assert_eq!(&build_url(&p, &opts), "/about.html");

        let p: PathBuf = "hello/world/about/index.htm".into();
        assert_eq!(&build_url(&p, &opts), "/about/index.htm");
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn building_windows_urls() {
        std::env::set_var("PAGEFIND_SOURCE", "C:\\hello\\world");
        let config =
            PagefindInboundConfig::with_layers(&[Layer::Env(Some("PAGEFIND_".into()))]).unwrap();
        let opts = SearchOptions::load(config).unwrap();

        let p: PathBuf = "C:\\hello\\world\\index.html".into();
        assert_eq!(&build_url(&p, &opts), "/");

        let p: PathBuf = "C:\\hello\\world\\about\\index.html".into();
        assert_eq!(&build_url(&p, &opts), "/about/");

        let p: PathBuf = "C:\\hello\\world\\about\\index.htm".into();
        assert_eq!(&build_url(&p, &opts), "/about/index.htm");
    }
}
