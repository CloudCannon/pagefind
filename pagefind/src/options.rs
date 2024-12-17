//! Configuration that can be supplied to the `api` module when using Pagefind as a service.

use anyhow::{bail, Result};
use clap::Parser;
use rust_patch::Patch;
use serde::{Deserialize, Serialize};
use std::{env, path::PathBuf};
use twelf::config;
use typed_builder::TypedBuilder;

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
pub(crate) struct PagefindInboundConfig {
    #[clap(long, help = "DEPRECATED: Use the `site` option instead")]
    #[clap(required = false, hide = true)]
    #[serde(default)] // This is actually required, but we validate that later
    pub(crate) source: String,

    #[clap(long, short, help = "The location of your built static website")]
    #[clap(required = false)]
    #[serde(default)] // This is actually required, but we validate that later
    pub(crate) site: String,

    #[clap(
        long,
        short,
        help = "DEPRECATED: Use `output_subdir` or `output_path` instead"
    )]
    #[clap(required = false, hide = true)]
    pub(crate) bundle_dir: Option<String>,

    #[clap(
        long,
        help = "Where to output the search bundle, relative to the processed site"
    )]
    #[clap(required = false)]
    pub(crate) output_subdir: Option<String>,

    #[clap(
        long,
        help = "Where to output the search bundle, relative to the working directory of the command"
    )]
    #[clap(required = false)]
    pub(crate) output_path: Option<String>,

    #[clap(
        long,
        help = "The element Pagefind should treat as the root of the document. Usually you will want to use the data-pagefind-body attribute instead."
    )]
    #[clap(required = false)]
    #[serde(default = "defaults::default_root_selector")]
    pub(crate) root_selector: String,

    #[clap(
        long,
        help = "Custom selectors that Pagefind should ignore when indexing. Usually you will want to use the data-pagefind-ignore attribute instead."
    )]
    #[clap(required = false)]
    #[serde(default)]
    pub(crate) exclude_selectors: Vec<String>,

    #[clap(
        long,
        help = "The file glob Pagefind uses to find HTML files. Defaults to \"**/*.{html}\""
    )]
    #[clap(required = false)]
    #[serde(default = "defaults::default_glob")]
    pub(crate) glob: String,

    #[clap(
        long,
        help = "Ignore any detected languages and index the whole site as a single language. Expects an ISO 639-1 code."
    )]
    #[clap(required = false)]
    pub(crate) force_language: Option<String>,

    #[clap(
        long,
        help = "Serve the source directory after creating the search index"
    )]
    #[clap(required = false)]
    #[serde(default = "defaults::default_false")]
    pub(crate) serve: bool,

    #[clap(
        long,
        short,
        help = "Print verbose logging while indexing the site. Does not impact the web-facing search."
    )]
    #[clap(required = false)]
    #[serde(default = "defaults::default_false")]
    pub(crate) verbose: bool,

    #[clap(
        long,
        short,
        help = "Only log errors and warnings while indexing the site. Does not impact the web-facing search."
    )]
    #[clap(required = false)]
    #[serde(default = "defaults::default_false")]
    pub(crate) quiet: bool,

    #[clap(
        long,
        short,
        help = "Only log errors while indexing the site. Does not impact the web-facing search."
    )]
    #[clap(required = false)]
    #[serde(default = "defaults::default_false")]
    pub(crate) silent: bool,

    #[clap(
        long,
        short,
        help = "Path to a logfile to write to. Will replace the file on each run"
    )]
    #[clap(required = false)]
    #[serde(default)]
    pub(crate) logfile: Option<String>,

    #[clap(
        long,
        short,
        help = "Keep \"index.html\" at the end of search result paths. Defaults to false, stripping \"index.html\"."
    )]
    #[clap(required = false)]
    #[serde(default = "defaults::default_false")]
    pub(crate) keep_index_url: bool,

    #[clap(long)]
    #[clap(required = false, hide = true)]
    #[serde(default = "defaults::default_false")]
    pub(crate) service: bool,
}

#[derive(Debug, Deserialize, Serialize, Patch, TypedBuilder)]
#[patch = "PagefindInboundConfig"]
#[builder(
    doc,
    field_defaults(default, setter(strip_option)),
    builder_method(
        vis = "pub",
        doc = "Create a builder for building `PagefindServiceConfig` for the api."
    ),
    builder_type(vis = "pub"),
    build_method(vis = "pub")
)]
/// Fields that can be set via the Pagefind service.
/// In other words, the subset of the Pagefind configuration that makes sense to set globally,
/// excluding fields that are irrelevant or set when each individual method is called.
///
/// Must be constructed through the `PagefindServiceConfigBuilder` interface.
pub struct PagefindServiceConfig {
    /// The element Pagefind should treat as the root of the document.
    pub(crate) root_selector: Option<String>,
    /// Custom selectors that Pagefind should ignore when indexing.
    pub(crate) exclude_selectors: Option<Vec<String>>,
    #[patch(as_option)]
    /// Ignore any detected languages and index the whole site as a single language. Expects an ISO 639-1 code.
    pub(crate) force_language: Option<String>,
    /// Print verbose logging while indexing the site. Does not impact the web-facing search.
    pub(crate) verbose: Option<bool>,
    #[patch(as_option)]
    /// Path to a logfile to write to. Will replace the file on each run
    pub(crate) logfile: Option<String>,
    /// Keep \"index.html\" at the end of search result paths. Defaults to false, stripping \"index.html\".
    pub(crate) keep_index_url: Option<bool>,
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
pub(crate) struct SearchOptions {
    pub(crate) working_directory: PathBuf,
    pub(crate) site_source: PathBuf,
    pub(crate) bundle_output: PathBuf,
    pub(crate) root_selector: String,
    pub(crate) exclude_selectors: Vec<String>,
    pub(crate) glob: String,
    pub(crate) force_language: Option<String>,
    pub(crate) version: &'static str,
    pub(crate) logger: Logger,
    pub(crate) keep_index_url: bool,
    pub(crate) running_as_service: bool,
    pub(crate) config_warnings: ConfigWarnings,
}

#[derive(Debug, Clone)]
pub(crate) struct ConfigWarnings {
    pub(crate) unconfigured_bundle_output: bool,
    pub(crate) using_deprecated_source: bool,
    pub(crate) using_deprecated_bundle_dir: bool,
}

impl SearchOptions {
    pub(crate) fn load(config: PagefindInboundConfig) -> Result<Self> {
        if !config.service && config.site.is_empty() && config.source.is_empty() {
            eprintln!("Required argument site not supplied. Pagefind needs to know the root of your built static site.");
            eprintln!("Provide a --site flag, a PAGEFIND_SITE environment variable, or a site key in a Pagefind configuration file.");
            bail!("Missing argument: site");
        } else {
            let log_level = if config.verbose {
                LogLevel::Verbose
            } else if config.quiet {
                LogLevel::Quiet
            } else if config.silent {
                LogLevel::Silent
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

            let bundle_output = if let Some(subdir) = config.output_path {
                working_directory.join(subdir)
            } else {
                let subdir = config
                    .output_subdir
                    .or(config.bundle_dir)
                    .unwrap_or_else(|| defaults::default_bundle_dir());
                site_source.join(subdir)
            };

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
    pub(crate) fn get_strings(&self) -> Vec<String> {
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
