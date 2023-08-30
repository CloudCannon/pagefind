use anyhow::{bail, Result};
use clap::Parser;
use rust_patch::Patch;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, path::PathBuf};
use twelf::config;

use crate::logging::{LogLevel, Logger};

//
// If editing this configuration struct,
// also make sure to edit the patch config below,
// (if it makes sense for that option to be set via the service),
// and also update any wrapper packages to include definitions for that option.
//
// No options should be added that are required.
//

#[config]
#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct PagefindInboundConfig {
    #[clap(long, help = "DEPRECATED: Use the `site` option instead")]
    #[clap(required = false, hide = true)]
    #[serde(default)] // This is actually required, but we validate that later
    pub source: String,

    #[clap(long, short, help = "The location of your built static website")]
    #[clap(required = false)]
    #[serde(default)] // This is actually required, but we validate that later
    pub site: String,

    #[clap(long, short, help = "DEPRECATED: Use . . .")]
    #[clap(required = false, hide = true)]
    pub bundle_dir: Option<String>,

    #[clap(
        long,
        short,
        help = "Where to output the search files, relative to the processed site"
    )]
    #[clap(required = false)]
    pub output_subdir: Option<String>,

    #[clap(
        long,
        help = "Where to output the search files, relative to the working directory of the command"
    )]
    #[clap(required = false)]
    pub output_path: Option<String>,

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

#[derive(Debug, Deserialize, Serialize, Patch)]
#[patch = "PagefindInboundConfig"]
/// Fields that can be set via the Pagefind service.
/// In other words, the subset of the above fields that make sense to set globally,
/// excluding those that are set when each individual method is called.
pub struct PagefindServiceConfig {
    pub root_selector: Option<String>,
    pub exclude_selectors: Option<Vec<String>>,
    #[patch(as_option)]
    pub force_language: Option<String>,
    pub verbose: Option<bool>,
    #[patch(as_option)]
    pub logfile: Option<String>,
    pub keep_index_url: Option<bool>,
}

mod defaults {
    pub fn default_bundle_dir() -> String {
        "pagefind".into()
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
    pub site_source: PathBuf,
    pub bundle_output: PathBuf,
    pub root_selector: String,
    pub exclude_selectors: Vec<String>,
    pub glob: String,
    pub force_language: Option<String>,
    pub version: &'static str,
    pub logger: Logger,
    pub keep_index_url: bool,
    pub running_as_service: bool,
    pub config_warnings: ConfigWarnings,
}

#[derive(Debug, Clone)]
pub struct ConfigWarnings {
    pub unconfigured_bundle_output: bool,
    pub using_deprecated_source: bool,
    pub using_deprecated_bundle_dir: bool,
}

impl SearchOptions {
    pub fn load(config: PagefindInboundConfig) -> Result<Self> {
        if !config.service && config.site.is_empty() && config.source.is_empty() {
            eprintln!("Required argument site not supplied. Pagefind needs to know the root of your built static site.");
            eprintln!("Provide a --site flag, a PAGEFIND_SITE environment variable, or a site key in a Pagefind configuration file.");
            bail!("Missing argument: site");
        } else {
            let log_level = if config.verbose {
                LogLevel::Verbose
            } else {
                LogLevel::Standard
            };

            let log_to_terminal = !config.service;
            let working_directory = env::current_dir().unwrap();

            let site_source = if !config.site.is_empty() {
                working_directory.join(PathBuf::from(config.site))
            } else {
                working_directory.join(PathBuf::from(config.source.clone()))
            };

            // For backwards compat pre-1.0, we output files for older defaults
            // when the path hasn't been set
            let configured_bundle_output = config
                .output_path
                .as_ref()
                .or(config.output_subdir.as_ref())
                .or(config.bundle_dir.as_ref())
                .is_some();

            let warnings = ConfigWarnings {
                unconfigured_bundle_output: !configured_bundle_output,
                using_deprecated_source: !config.source.is_empty(),
                using_deprecated_bundle_dir: config.bundle_dir.is_some(),
            };

            let bundle_output = config
                .output_path
                .map(|o| working_directory.join(o))
                .or(config.output_subdir.map(|o| site_source.join(o)))
                .or(config.bundle_dir.map(|o| site_source.join(o)))
                .unwrap_or_else(|| site_source.join(PathBuf::from("_pagefind")));

            Ok(Self {
                working_directory,
                site_source,
                bundle_output,
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
                running_as_service: config.service,
                config_warnings: warnings,
            })
        }
    }
}

impl ConfigWarnings {
    pub fn get_strings(&self) -> Vec<String> {
        let mut strings = vec![];
        if self.using_deprecated_bundle_dir {
            strings.push(
                "\n\
                 The `bundle-dir` option is deprecated as of Pagefind 1.0. \
                 Use either `output-subdir` or `output-path` instead:\n\n\
                 cli:    --output-subdir\n\
                 config: output_subdir\n\
                 env:    PAGEFIND_OUTPUT_SUBDIR\n\
                 └─ \"Where to output the search files, relative to the processed site\"\n\n\
                 cli:    --output-path\n\
                 config: output_path\n\
                 env:    PAGEFIND_OUTPUT_PATH\n\
                 └─ \"Where to output the search files, relative to the working directory of the command\"\n"
                    .into(),
            );
        }

        if self.using_deprecated_source {
            strings.push(
                "\n\
                 The `source` option is deprecated as of Pagefind 1.0. \
                 The `source` option has been renamed to `site`:\n\n\
                 cli:    --site\n\
                 config: site\n\
                 env:    PAGEFIND_SITE\n\
                 └─ \"The location of your built static website\"\n"
                    .into(),
            );
        }

        strings
    }
}
