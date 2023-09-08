use async_compression::tokio::bufread::GzipDecoder;
#[cfg(feature = "extended")]
use charabia::Segment;
use hashbrown::HashMap;
use lazy_static::lazy_static;
use pagefind_stem::{Algorithm, Stemmer};
use path_slash::PathExt as _;
use regex::Regex;
use std::io::Error;
use std::ops::Mul;
use std::path::{Path, PathBuf};
use tokio::fs::File;
use tokio::io::AsyncBufReadExt;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::time::{sleep, Duration};

use crate::fragments::{PageAnchorData, PageFragment, PageFragmentData};
use crate::SearchOptions;
use parser::DomParser;

use self::parser::DomParserResult;
use self::splitting::get_discrete_words;

lazy_static! {
    static ref NEWLINES: Regex = Regex::new("(\n|\r\n)+").unwrap();
    static ref TRIM_NEWLINES: Regex = Regex::new("^[\n\r\\s]+|[\n\r\\s]+$").unwrap();
    static ref EXTRANEOUS_SPACES: Regex = Regex::new("\\s{2,}").unwrap();
    // TODO: i18n?
    static ref SPECIAL_CHARS: Regex = Regex::new("[^\\w]").unwrap();
}

pub mod parser;
mod splitting;

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
    pub has_old_bundle_reference: bool,
    pub language: String,
}

#[derive(Debug)]
pub struct Fossicker {
    file_path: Option<PathBuf>,
    /// Built URLs should be relative to this directory
    root_path: Option<PathBuf>,
    page_url: Option<String>,
    synthetic_content: Option<String>,
    data: Option<DomParserResult>,
}

impl Fossicker {
    pub fn new(file_path: PathBuf) -> Self {
        Self {
            file_path: Some(file_path),
            root_path: None,
            page_url: None,
            synthetic_content: None,
            data: None,
        }
    }

    pub fn new_relative_to(file_path: PathBuf, root_path: PathBuf) -> Self {
        Self {
            file_path: Some(file_path),
            root_path: Some(root_path),
            page_url: None,
            synthetic_content: None,
            data: None,
        }
    }

    pub fn new_synthetic(
        file_path: Option<PathBuf>,
        page_url: Option<String>,
        contents: String,
    ) -> Self {
        Self {
            file_path,
            root_path: None,
            page_url,
            synthetic_content: Some(contents),
            data: None,
        }
    }

    pub fn new_with_data(url: String, data: DomParserResult) -> Self {
        Self {
            file_path: None,
            root_path: None,
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
                    &self
                        .file_path
                        .as_ref()
                        .map(|p| p.to_str())
                        .flatten()
                        .or(self.page_url.as_ref().map(|u| u.as_str()))
                        .unwrap_or("[unknown file]")
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
        Vec<(String, String, String, u32)>,
        usize,
    ) {
        let mut map: HashMap<String, Vec<FossickedWord>> = HashMap::new();
        let mut anchors = Vec::new();
        // TODO: push this error handling up a level and return an Err from parse_digest
        if self.data.as_ref().is_none() {
            return ("".into(), map, anchors, 0); // empty page result, will be dropped from search
        }
        let data = self.data.as_ref().unwrap();
        let stemmer = get_stemmer(&data.language);

        let mut content = String::with_capacity(data.digest.len());

        let mut store_word = |full_word: &str, word_index: usize, word_weight: u8| {
            let word = if let Some(stemmer) = &stemmer {
                stemmer.stem(&full_word).into_owned()
            } else {
                full_word.to_string()
            };

            let entry = FossickedWord {
                position: word_index.try_into().unwrap(),
                weight: word_weight,
            };
            if let Some(repeat) = map.get_mut(&word) {
                repeat.push(entry);
            } else {
                map.insert(word, vec![entry]);
            }
        };

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
        let mut max_word_index = 0;
        let weight_multiplier = 25.0;
        let weight_max = 10.0;
        debug_assert!(((weight_max * weight_multiplier) as u8) < std::u8::MAX);

        let mut weight_stack: Vec<u8> = vec![(1.0 * weight_multiplier) as u8];
        for (word_index, word) in segments.into_iter().enumerate() {
            let word_index = word_index - offset_word_index;

            if word.chars().next() == Some('_') {
                if word.starts_with("___PAGEFIND_ANCHOR___") {
                    if let Some((element_name, anchor_id)) =
                        word.replace("___PAGEFIND_ANCHOR___", "").split_once(':')
                    {
                        let element_text = data
                            .anchor_content
                            .get(anchor_id)
                            .map(|t| normalize_content(t))
                            .unwrap_or_default();

                        if let Some((_, element_id)) = anchor_id.split_once(':') {
                            anchors.push((
                                element_name.to_string(),
                                element_id.to_string(),
                                normalize_content(&element_text),
                                word_index as u32,
                            ));
                        }
                    }
                    offset_word_index += 1;
                    continue;
                }

                if word.starts_with("___PAGEFIND_WEIGHT___") {
                    let weight = word
                        .replace("___PAGEFIND_WEIGHT___", "")
                        .parse::<f32>()
                        .ok()
                        .unwrap_or(1.0);
                    if weight <= 0.0 {
                        weight_stack.push(0);
                    } else {
                        weight_stack.push(
                            (weight.clamp(0.0, weight_max).mul(weight_multiplier) as u8).max(1),
                        );
                    }
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
                            .parse::<f32>()
                            .ok()
                            .unwrap_or(1.0);
                        weight_stack
                            .push(weight.clamp(0.0, weight_max).mul(weight_multiplier) as u8);
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
            #[cfg(feature = "extended")]
            if should_segment {
                content.push('\u{200B}');
            }

            let normalized_word = SPECIAL_CHARS
                .replace_all(word, "")
                .into_owned()
                .to_lowercase();

            if !normalized_word.is_empty() {
                store_word(&normalized_word, word_index, *word_weight);
            }

            // For words that may be CompoundWords, also index them as their constituent parts
            if normalized_word != word {
                let (word_parts, extras) = get_discrete_words(word);
                // Only proceed if the word was broken into multiple parts
                if word_parts.contains(|c: char| c.is_whitespace()) {
                    let part_words: Vec<_> = word_parts.split_whitespace().collect();
                    // Index constituents of a compound word as a proportion of the
                    // weight of the full word.
                    let per_weight =
                        (word_weight / part_words.len().try_into().unwrap_or(std::u8::MAX)).max(1);

                    // Only index two+ character words
                    for part_word in part_words.into_iter().filter(|w| w.len() > 1) {
                        store_word(part_word, word_index, per_weight);
                    }
                }
                // Additionally store any special extra characters we are given
                if let Some(extras) = extras {
                    for extra in extras {
                        store_word(&extra, word_index, *word_weight);
                    }
                }
            }

            max_word_index = word_index;
        }
        if content.ends_with(' ') {
            content.pop();
        }
        (content, map, anchors, max_word_index + 1)
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
        if (self.file_path.is_some() || self.synthetic_content.is_some()) && self.data.is_none() {
            self.fossick_html(options).await;
        };

        let (content, word_data, anchors, word_count) = self.parse_digest();

        let data = self.data.unwrap();
        let url = if let Some(url) = &self.page_url {
            url.clone()
        } else if let Some(path) = &self.file_path {
            if let Some(root) = &self.root_path {
                build_url(path, Some(root), options)
            } else {
                build_url(path, None, options)
            }
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
            has_old_bundle_reference: data.has_old_bundle_reference,
            language: data.language,
            fragment: PageFragment {
                page_number: 0, // This page number is updated later once determined
                data: PageFragmentData {
                    url,
                    content,
                    filters: data.filters,
                    meta: data.meta,
                    word_count,
                    anchors: anchors
                        .into_iter()
                        .map(|(element, id, text, location)| PageAnchorData {
                            element,
                            id,
                            location,
                            text,
                        })
                        .collect(),
                },
            },
            word_data,
            sort: data.sort,
        })
    }
}

fn build_url(page_url: &Path, relative_to: Option<&Path>, options: &SearchOptions) -> String {
    let prefix = relative_to.unwrap_or(&options.site_source);

    let url = if let Ok(trimmed) = page_url.strip_prefix(prefix) {
        trimmed
    } else if page_url.is_relative() {
        page_url
    } else {
        options.logger.error(format!(
            "Absolute file was found that does not start with the source directory. Source: {:?}\nFile: {:?}",
            prefix,
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

fn normalize_content(content: &str) -> String {
    let content = html_escape::decode_html_entities(content);
    let content = TRIM_NEWLINES.replace_all(&content, "");
    let content = NEWLINES.replace_all(&content, " ");
    let content = EXTRANEOUS_SPACES.replace_all(&content, " ");

    content.to_string()
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

    #[test]
    fn normalizing_content() {
        let input = "\nHello  Wor\n ld? \n \n";
        let output = normalize_content(input);

        assert_eq!(&output, "Hello Wor ld?");
    }

    async fn test_fossick(s: String) -> Fossicker {
        std::env::set_var("PAGEFIND_SOURCE", "somewhere");
        let config =
            PagefindInboundConfig::with_layers(&[Layer::Env(Some("PAGEFIND_".into()))]).unwrap();
        let opts = SearchOptions::load(config).unwrap();

        let mut f = Fossicker {
            file_path: Some("test/index.html".into()),
            root_path: None,
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

        let (digest, words, anchors, word_count) = f.parse_digest();

        assert_eq!(digest, "Hello World!".to_string());
        assert_eq!(
            words,
            HashMap::from_iter([
                (
                    "hello".to_string(),
                    vec![FossickedWord {
                        position: 0,
                        weight: 1 * 25
                    }]
                ),
                (
                    "world".to_string(),
                    vec![FossickedWord {
                        position: 1,
                        weight: 1 * 25
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
                "Fox",
                "<p data-pagefind-weight='0.5'>Jumps Over</p>",
                "<p data-pagefind-weight='0.00001'>Ryan</p></div>",
                "</body></html>",
            ]
            .concat(),
        )
        .await;

        let (digest, words, anchors, word_count) = f.parse_digest();

        assert_eq!(digest, "The Quick Brown. Fox Jumps Over. Ryan.".to_string());
        assert_eq!(
            words,
            HashMap::from_iter([
                (
                    "the".to_string(),
                    vec![FossickedWord {
                        position: 0,
                        weight: 1 * 25
                    }]
                ),
                (
                    "quick".to_string(),
                    vec![FossickedWord {
                        position: 1,
                        weight: 2 * 25
                    }]
                ),
                (
                    "brown".to_string(),
                    vec![FossickedWord {
                        position: 2,
                        weight: 2 * 25
                    }]
                ),
                (
                    "fox".to_string(),
                    vec![FossickedWord {
                        position: 3,
                        weight: 1 * 25
                    }]
                ),
                (
                    "jumps".to_string(),
                    vec![FossickedWord {
                        position: 4,
                        weight: 12
                    }]
                ),
                (
                    "over".to_string(),
                    vec![FossickedWord {
                        position: 5,
                        weight: 12
                    }]
                ),
                (
                    "ryan".to_string(),
                    vec![FossickedWord {
                        position: 6,
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
                "<div data-pagefind-weight='0'><h1>Pagefind</h1></div>",
                "</body></html>",
            ]
            .concat(),
        )
        .await;

        let (digest, words, anchors, word_count) = f.parse_digest();

        assert_eq!(
            words,
            HashMap::from_iter([(
                "pagefind".to_string(),
                vec![
                    FossickedWord {
                        position: 0,
                        weight: 7 * 25
                    },
                    FossickedWord {
                        position: 1,
                        weight: 6 * 25
                    },
                    FossickedWord {
                        position: 2,
                        weight: 5 * 25
                    },
                    FossickedWord {
                        position: 3,
                        weight: 4 * 25
                    },
                    FossickedWord {
                        position: 4,
                        weight: 3 * 25
                    },
                    FossickedWord {
                        position: 5,
                        weight: 2 * 25
                    },
                    FossickedWord {
                        position: 6,
                        weight: 1 * 25
                    },
                    FossickedWord {
                        position: 7,
                        weight: 0 * 25
                    }
                ]
            )])
        );
    }

    #[tokio::test]
    async fn parse_bad_weights() {
        let mut f = test_fossick(
            [
                "<html><body>",
                "<p data-pagefind-weight='lots'>The</p>",
                "<p data-pagefind-weight='99999999'>Quick</p>",
                "<p data-pagefind-weight='-1234'>Brown</p>",
                "<p data-pagefind-weight='65.4'>Fox</p>",
                "</body></html>",
            ]
            .concat(),
        )
        .await;

        let (digest, words, anchors, word_count) = f.parse_digest();

        assert_eq!(
            words,
            HashMap::from_iter([
                (
                    "the".to_string(),
                    vec![FossickedWord {
                        position: 0,
                        weight: 25
                    }]
                ),
                (
                    "quick".to_string(),
                    vec![FossickedWord {
                        position: 1,
                        weight: 250
                    }]
                ),
                (
                    "brown".to_string(),
                    vec![FossickedWord {
                        position: 2,
                        weight: 0
                    }]
                ),
                (
                    "fox".to_string(),
                    vec![FossickedWord {
                        position: 3,
                        weight: 250
                    }]
                )
            ])
        );
    }

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn building_url() {
        std::env::set_var("PAGEFIND_SITE", "hello/world");
        let config =
            PagefindInboundConfig::with_layers(&[Layer::Env(Some("PAGEFIND_".into()))]).unwrap();
        let opts = SearchOptions::load(config).unwrap();

        let cwd = std::env::current_dir().unwrap();

        let p: PathBuf = cwd.join::<PathBuf>("hello/world/index.html".into());
        assert_eq!(&build_url(&p, None, &opts), "/");

        let p: PathBuf = cwd.join::<PathBuf>("hello/world/about/index.html".into());
        assert_eq!(&build_url(&p, None, &opts), "/about/");

        let p: PathBuf = cwd.join::<PathBuf>("hello/world/about.html".into());
        assert_eq!(&build_url(&p, None, &opts), "/about.html");

        let p: PathBuf = cwd.join::<PathBuf>("hello/world/about/index.htm".into());
        assert_eq!(&build_url(&p, None, &opts), "/about/index.htm");

        let p: PathBuf = cwd.join::<PathBuf>("hello/world/index.html".into());
        let root: PathBuf = cwd.join::<PathBuf>("hello".into());
        assert_eq!(&build_url(&p, Some(&root), &opts), "/world/");
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn building_windows_urls() {
        std::env::set_var("PAGEFIND_SITE", "C:\\hello\\world");
        let config =
            PagefindInboundConfig::with_layers(&[Layer::Env(Some("PAGEFIND_".into()))]).unwrap();
        let opts = SearchOptions::load(config).unwrap();

        let p: PathBuf = "C:\\hello\\world\\index.html".into();
        assert_eq!(&build_url(&p, None, &opts), "/");

        let p: PathBuf = "C:\\hello\\world\\about\\index.html".into();
        assert_eq!(&build_url(&p, None, &opts), "/about/");

        let p: PathBuf = "C:\\hello\\world\\about\\index.htm".into();
        assert_eq!(&build_url(&p, None, &opts), "/about/index.htm");
    }
}
