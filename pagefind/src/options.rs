use anyhow::{bail, Result};
use clap::Parser;
use std::{env, path::PathBuf};
use twelf::config;

#[config]
#[derive(Parser, Debug)]
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
        short,
        help = "Print debug logging while indexing the site. Does not impact the web-facing search."
    )]
    #[clap(required = false)]
    #[serde(default = "defaults::default_verbosity")]
    pub verbose: bool,
}

mod defaults {
    pub fn default_bundle_dir() -> String {
        "_pagefind".into()
    }
    pub fn default_verbosity() -> bool {
        false
    }
}

// The configuration object used internally
#[derive(Debug)]
pub struct SearchOptions {
    pub working_directory: PathBuf,
    pub source: PathBuf,
    pub bundle_dir: PathBuf,
    pub verbose: bool,
    pub version: &'static str,
}

impl SearchOptions {
    pub fn load(config: PagefindInboundConfig) -> Result<Self> {
        if config.source.is_empty() {
            eprintln!("Required argument source not supplied. Pagefind needs to know the root of your built static site.");
            eprintln!("Provide a --source flag, a PAGEFIND_SOURCE environment variable, or a source key in a Pagefind configuration file.");
            bail!("Missing argument: source");
        } else {
            Ok(Self {
                working_directory: env::current_dir().unwrap(),
                source: PathBuf::from(config.source),
                bundle_dir: PathBuf::from(config.bundle_dir),
                verbose: config.verbose,
                version: env!("CARGO_PKG_VERSION"),
            })
        }
    }
}
