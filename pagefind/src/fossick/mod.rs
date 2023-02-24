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

use crate::fragments::{PageFragment, PageFragmentData};
use crate::SearchOptions;
use parser::DomParser;

use self::parser::DomParserResult;

lazy_static! {
    // TODO: i18n?
    static ref SPECIAL_CHARS: Regex = Regex::new("[^\\w]").unwrap();
}

mod parser;

#[derive(Debug)]
pub struct FossickedData {
    pub file_path: PathBuf,
    pub fragment: PageFragment,
    pub word_data: HashMap<String, Vec<u32>>,
    pub sort: HashMap<String, String>,
    pub has_custom_body: bool,
    pub has_html_element: bool,
    pub language: String,
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

    async fn read_file(&mut self, options: &SearchOptions) -> Result<(), Error> {
        let file = File::open(&self.file_path).await?;

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
                        self.file_path.to_str().unwrap_or("[unknown file]")
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
                        self.file_path.to_str().unwrap_or("[unknown file]")
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

    fn parse_digest(&mut self) -> (String, HashMap<String, Vec<u32>>) {
        let mut map: HashMap<String, Vec<u32>> = HashMap::new();
        // TODO: push this error handling up a level and return an Err from parse_digest
        if self.data.as_ref().is_none() {
            return ("".into(), map); // empty page result, will be dropped from search
        }
        let data = self.data.as_ref().unwrap();
        let stemmer = get_stemmer(&data.language);

        #[cfg(feature = "extended")]
        let mut content = String::with_capacity(data.digest.len());

        #[cfg(not(feature = "extended"))]
        let content = data.digest.replace('\u{200B}', ""); // TODO: Use separate parse_digest methods based on features

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
            content.push_str(&data.digest.replace('\u{200B}', ""));
            // Currently hesistant to run segmentation during indexing
            // that we can't also run during search, since we don't
            // ship a segmenter to the browser. This logic is easier
            // to replicate in the JavaScript that parses a search query.
            data.digest.split_whitespace().collect::<Vec<_>>()
        };

        #[cfg(not(feature = "extended"))]
        let segments = data.digest.split_whitespace();

        for (word_index, word) in segments.into_iter().enumerate() {
            let mut normalized_word = SPECIAL_CHARS
                .replace_all(word, "")
                .into_owned()
                .to_lowercase();

            #[cfg(feature = "extended")]
            if should_segment {
                content.push_str(&word.replace('\u{200B}', ""));
                content.push('\u{200B}');
            }

            if !normalized_word.is_empty() {
                if let Some(stemmer) = &stemmer {
                    normalized_word = stemmer.stem(&normalized_word).into_owned();
                }

                if let Some(repeat) = map.get_mut(&normalized_word) {
                    repeat.push(word_index.try_into().unwrap());
                } else {
                    map.insert(normalized_word, vec![word_index.try_into().unwrap()]);
                }
            }
        }

        (content, map)
    }

    pub async fn fossick(mut self, options: &SearchOptions) -> Result<FossickedData, ()> {
        while self.read_file(options).await.is_err() {
            sleep(Duration::from_millis(1)).await;
        }

        let (content, word_data) = self.parse_digest();

        if self.data.is_none() {
            return Err(());
        }

        let data = self.data.unwrap();
        let url = build_url(&self.file_path, options);

        Ok(FossickedData {
            file_path: self.file_path,
            has_custom_body: data.has_custom_body,
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

    format!(
        "/{}",
        final_url
    )
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
