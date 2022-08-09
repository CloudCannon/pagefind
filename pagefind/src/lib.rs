use fossick::Fossicker;
use futures::future::join_all;
use hashbrown::HashMap;
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

        let mut language_map: HashMap<String, Vec<fossick::FossickedData>> = HashMap::new();
        for page in pages_with_data {
            let language = page.language.clone();
            if let Some(lang_pages) = language_map.get_mut(&language) {
                lang_pages.push(page);
            } else {
                language_map.insert(language, vec![page]);
            }
        }

        let indexes: Vec<_> = language_map
            .into_iter()
            .map(|(language, pages)| async {
                let indexes = build_indexes(pages.into_iter(), language, &self.options).await;
                let index_meta = (indexes.language.clone(), indexes.meta_index.0.clone());
                indexes.write_files(&self.options).await;
                index_meta
            })
            .collect();
        let language_indexes = join_all(indexes).await;

        output::write_common(&self.options, language_indexes).await;
    }
}
