use fossick::Fossicker;
use futures::future::join_all;
pub use options::{PagefindInboundConfig, SearchOptions};
use wax::{Glob, WalkEntry};

use crate::index::build_indexes;

mod fossick;
mod fragments;
mod index;
mod options;
mod output;
pub mod serve;
mod utils;

pub struct SearchState {
    options: SearchOptions,
}

impl SearchState {
    pub fn new(options: SearchOptions) -> Self {
        Self { options }
    }

    pub async fn walk_for_files(&mut self) -> Vec<Fossicker> {
        println!("Walking source directory...");
        if let Ok(glob) = Glob::new(&self.options.glob) {
            glob.walk(&self.options.source, usize::MAX)
                .filter_map(Result::ok)
                .map(WalkEntry::into_path)
                .map(Fossicker::new)
                .collect()
        } else {
            eprintln!(
                "Error: Provided glob \"{}\" did not parse as a valid glob.",
                self.options.glob
            );
            std::process::exit(1);
        }
    }

    pub async fn run(&mut self) {
        if self.options.verbose {
            println!("Running Pagefind v{} in verbose mode", self.options.version);
        } else {
            println!("Running Pagefind v{}", self.options.version);
        }
        println!("Running from: {:?}", self.options.working_directory);
        println!("Source:       {:?}", self.options.source);
        println!("Bundle Directory:  {:?}", self.options.bundle_dir);
        let files = self.walk_for_files().await;
        println!("Building search indexes...");

        let results: Vec<_> = files
            .into_iter()
            .map(|f| f.fossick(&self.options))
            .collect();
        let all_pages = join_all(results).await;

        let used_custom_body = all_pages.iter().flatten().any(|page| page.has_custom_body);
        if used_custom_body {
            println!(
                "Found a data-pagefind-body element on the site.\n↳ Ignoring pages without this tag."
            );
        } else {
            println!(
                "Did not find a data-pagefind-body element on the site.\n↳ Indexing all <body> elements on the site."
            );
        }

        let pages_with_data = all_pages.into_iter().flatten().filter(|d| {
            if used_custom_body && !d.has_custom_body {
                return false;
            }
            !d.word_data.is_empty()
        });

        let indexes = build_indexes(pages_with_data, &self.options).await;
        indexes.write_files(&self.options).await;
    }
}
