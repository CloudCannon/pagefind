use std::{env, path::PathBuf};

use clap::ArgMatches;

pub struct SearchOptions {
    pub working_directory: PathBuf,
    pub source: PathBuf,
    pub dest: PathBuf,
    pub bundle_dir: PathBuf,
    pub verbose: bool,
    pub version: &'static str,
}

impl Default for SearchOptions {
    fn default() -> Self {
        Self {
            working_directory: env::current_dir().unwrap(),
            source: PathBuf::from("public"),
            dest: PathBuf::from("public"),
            bundle_dir: PathBuf::from("_pagefind"),
            verbose: false,
            version: env!("CARGO_PKG_VERSION"),
        }
    }
}

impl From<&ArgMatches<'_>> for SearchOptions {
    fn from(cli: &ArgMatches) -> Self {
        let defaults = SearchOptions::default();
        let source = cli
            .value_of("source")
            .map(PathBuf::from)
            .unwrap_or(defaults.source);
        SearchOptions {
            source: source.clone(),
            dest: cli.value_of("dest").map(PathBuf::from).unwrap_or(source),
            bundle_dir: cli
                .value_of("bundle_dir")
                .map(PathBuf::from)
                .unwrap_or(defaults.bundle_dir),
            ..defaults
        }
    }
}
