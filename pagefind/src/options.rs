use anyhow::{bail, Result};
use clap::Parser;
use std::{env, path::PathBuf};
use twelf::config;

use crate::logging::{LogLevel, Logger};

#[config]
#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct PagefindInboundConfig {
    #[clap(long, short, help = "The location of your built static website")]
    #[clap(required = false)]
    #[serde(default)] // This is actually required, but we validate that later
    pub source: String,

    #[clap(
        long,
        short,
        help = "Where to output the search files, relative to source"
    )]
    #[clap(required = false)]
    #[serde(default = "defaults::default_bundle_dir")]
    pub bundle_dir: String,

    #[clap(
        long,
        help = "The element Pagefind should treat as the root of the document. Usually you will want to use the data-pagefind-body attribute instead."
    )]
    #[clap(required = false)]
    #[serde(default = "defaults::default_root_selector")]
    pub root_selector: String,

    #[clap(
        long,
        help = "Custom selectors that Pagefind should ignore when indexing. Usually you will want to use the data-pagefind-ignore attribute instead."
    )]
    #[clap(required = false)]
    #[serde(default)]
    pub exclude_selectors: Vec<String>,

    #[clap(
        long,
        help = "The file glob Pagefind uses to find HTML files. Defaults to \"**/*.{html}\""
    )]
    #[clap(required = false)]
    #[serde(default = "defaults::default_glob")]
    pub glob: String,

    #[clap(
        long,
        help = "Ignore any detected languages and index the whole site as a single language. Expects an ISO 639-1 code."
    )]
    #[clap(required = false)]
    pub force_language: Option<String>,

    #[clap(
        long,
        help = "Serve the source directory after creating the search index"
    )]
    #[clap(required = false)]
    #[serde(default = "defaults::default_false")]
    pub serve: bool,

    #[clap(
        long,
        short,
        help = "Print verbose logging while indexing the site. Does not impact the web-facing search."
    )]
    #[clap(required = false)]
    #[serde(default = "defaults::default_false")]
    pub verbose: bool,

    #[clap(
        long,
        short,
        help = "Path to a logfile to write to. Will replace the file on each run"
    )]
    #[clap(required = false)]
    #[serde(default)]
    pub logfile: Option<String>,

    #[clap(
        long,
        short,
        help = "Keep \"index.html\" at the end of search result paths. Defaults to false, stripping \"index.html\"."
    )]
    #[clap(required = false)]
    #[serde(default = "defaults::default_false")]
    pub keep_index_url: bool,

    #[clap(long)]
    #[clap(required = false)]
    #[serde(default = "defaults::default_false")]
    pub service: bool,
}

mod defaults {
    pub fn default_bundle_dir() -> String {
        "_pagefind".into()
    }
    pub fn default_root_selector() -> String {
        "html".into()
    }
    pub fn default_glob() -> String {
        "**/*.{html}".into()
    }
    pub fn default_false() -> bool {
        false
    }
}

// The configuration object used internally
#[derive(Debug, Clone)]
pub struct SearchOptions {
    pub working_directory: PathBuf,
    pub source: PathBuf,
    pub bundle_dir: PathBuf,
    pub root_selector: String,
    pub exclude_selectors: Vec<String>,
    pub glob: String,
    pub force_language: Option<String>,
    pub version: &'static str,
    pub logger: Logger,
    pub keep_index_url: bool,
}

impl SearchOptions {
    pub fn load(config: PagefindInboundConfig) -> Result<Self> {
        if !config.service && config.source.is_empty() {
            eprintln!("Required argument source not supplied. Pagefind needs to know the root of your built static site.");
            eprintln!("Provide a --source flag, a PAGEFIND_SOURCE environment variable, or a source key in a Pagefind configuration file.");
            bail!("Missing argument: source");
        } else {
            let log_level = if config.verbose {
                LogLevel::Verbose
            } else {
                LogLevel::Standard
            };

            let log_to_terminal = !config.service;

            Ok(Self {
                working_directory: env::current_dir().unwrap(),
                source: PathBuf::from(config.source),
                bundle_dir: PathBuf::from(config.bundle_dir),
                root_selector: config.root_selector,
                exclude_selectors: config.exclude_selectors,
                glob: config.glob,
                force_language: config.force_language,
                version: env!("CARGO_PKG_VERSION"),
                logger: Logger::new(
                    log_level,
                    log_to_terminal,
                    config.logfile.map(PathBuf::from),
                ),
                keep_index_url: config.keep_index_url,
            })
        }
    }
}
