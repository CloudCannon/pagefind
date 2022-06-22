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
mod utils;

pub struct SearchState {
    options: SearchOptions,
    files: Vec<Fossicker>,
}

impl SearchState {
    pub fn new(options: SearchOptions) -> Self {
        Self {
            options,
            files: vec![],
        }
    }

    pub async fn walk_for_files(&mut self) {
        println!("Walking source directory...");
        let glob = Glob::new("**/*.{html}").unwrap();
        self.files = glob
            .walk(&self.options.source, usize::MAX)
            .filter_map(Result::ok)
            .map(WalkEntry::into_path)
            .map(Fossicker::new)
            .collect()
    }

    pub async fn run(&mut self) {
        println!("Running Pagefind v{}", self.options.version);
        println!("Running from: {:?}", self.options.working_directory);
        println!("Source:       {:?}", self.options.source);
        println!("Bundle Directory:  {:?}", self.options.bundle_dir);
        self.walk_for_files().await;
        println!("Building search indexes...");

        let results: Vec<_> = self
            .files
            .iter_mut()
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
