use std::cmp::Ordering;

use fossick::Fossicker;
use futures::future::join_all;
use hashbrown::HashMap;
pub use options::{PagefindInboundConfig, SearchOptions};
use wax::{Glob, WalkEntry};

use crate::index::build_indexes;

mod fossick;
mod fragments;
mod index;
#[macro_use]
mod logging;
mod options;
mod output;
pub mod serve;
mod utils;

pub struct SearchState {
    pub options: SearchOptions,
}

impl SearchState {
    pub fn new(options: SearchOptions) -> Self {
        Self { options }
    }

    pub async fn walk_for_files(&mut self) -> Vec<Fossicker> {
        let log = &self.options.logger;

        log.status("[Walking source directory]");
        if let Ok(glob) = Glob::new(&self.options.glob) {
            glob.walk(&self.options.source, usize::MAX)
                .filter_map(Result::ok)
                .map(WalkEntry::into_path)
                .map(Fossicker::new)
                .collect()
        } else {
            log.error(format!(
                "Error: Provided glob \"{}\" did not parse as a valid glob.",
                self.options.glob
            ));
            std::process::exit(1);
        }
    }

    pub async fn run(&mut self) {
        let log = &self.options.logger;
        log.status(&format!("Running Pagefind v{}", self.options.version));
        log.v_info("Running in verbose mode");

        log.info(format!(
            "Running from: {:?}",
            self.options.working_directory
        ));
        log.info(format!("Source:       {:?}", self.options.source));
        log.info(format!("Bundle Directory:  {:?}", self.options.bundle_dir));

        let files = self.walk_for_files().await;
        let log = &self.options.logger;

        log.info(format!(
            "Found {} file{} matching {}",
            files.len(),
            plural!(files.len()),
            self.options.glob
        ));
        log.status("[Parsing files]");

        let results: Vec<_> = files
            .into_iter()
            .map(|f| f.fossick(&self.options))
            .collect();
        let all_pages = join_all(results).await;

        let used_custom_body = all_pages.iter().flatten().any(|page| page.has_custom_body);
        if used_custom_body {
            log.info("Found a data-pagefind-body element on the site.\n↳ Ignoring pages without this tag.");
        } else {
            log.info(
                "Did not find a data-pagefind-body element on the site.\n↳ Indexing all <body> elements on the site."
            );
        }

        if self.options.root_selector == "html" {
            let pages_without_html = all_pages
                .iter()
                .flatten()
                .filter(|p| !p.has_html_element)
                .map(|p| format!("  * {:?} has no <html> element", p.fragment.data.url))
                .collect::<Vec<_>>();
            if !pages_without_html.is_empty() {
                log.warn(format!(
                    "{} page{} found without an <html> element. \n\
                    Pages without an outer <html> element will not be processed by default. \n\
                    If adding this element is not possible, use the root selector config to target a different root element.",
                    pages_without_html.len(),
                    plural!(pages_without_html.len())
                ));
                log.v_warn(pages_without_html.join("\n"));
            }
        }

        log.status("[Reading languages]");

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

        log.info(format!(
            "Discovered {} language{}: {}",
            language_map.len(),
            plural!(language_map.len()),
            language_map.keys().cloned().collect::<Vec<_>>().join(", ")
        ));
        log.v_info(
            language_map
                .iter()
                .map(|(k, v)| format!("  * {}: {} page{}", k, v.len(), plural!(v.len())))
                .collect::<Vec<_>>()
                .join("\n"),
        );

        let primary_language = language_map
            .iter()
            .filter(|(k, _)| k.as_str() != "unknown")
            .max_by(|(lang_a, pages_a), (lang_b, pages_b)| {
                let size = pages_a.len().cmp(&pages_b.len());
                if matches!(size, Ordering::Equal) {
                    return lang_b.cmp(lang_a);
                }
                size
            })
            .map(|(k, _)| k.clone())
            .unwrap_or_else(|| "unknown".into());

        if let Some(mut unknown_pages) = language_map.remove("unknown") {
            if language_map.len() > 1 {
                log.warn(format!(
                    "{} page{} found without an html lang attribute. \n\
                    Merging these pages with the {} language, as that is the main language on this site. \n\
                    Run Pagefind with --verbose for more information.",
                    unknown_pages.len(),
                    plural!(unknown_pages.len()),
                    primary_language
                ));

                log.v_warn(
                    unknown_pages
                        .iter()
                        .map(|p| {
                            format!("  * {:?} has no html lang attribute", p.fragment.data.url)
                        })
                        .collect::<Vec<_>>()
                        .join("\n"),
                );

                if let Some(primary) = language_map.get_mut(&primary_language) {
                    primary.append(&mut unknown_pages);
                } else {
                    language_map.insert(primary_language, unknown_pages);
                }
            } else {
                language_map.insert(primary_language, unknown_pages);
            }
        }

        log.status("[Building search indexes]");

        let indexes: Vec<_> = language_map
            .into_iter()
            .map(|(language, pages)| async {
                build_indexes(pages.into_iter(), language, &self.options).await
            })
            .collect();
        let indexes = join_all(indexes).await;

        let stats = indexes.iter().fold((0, 0, 0), |mut stats, index| {
            log.v_info(format!(
                "Language {}: \n  Indexed {} page{}\n  Indexed {} word{}\n  Indexed {} filter{}\n",
                index.language,
                index.fragments.len(),
                plural!(index.fragments.len()),
                index.word_count,
                plural!(index.word_count),
                index.filter_indexes.len(),
                plural!(index.filter_indexes.len())
            ));
            stats.0 += index.fragments.len();
            stats.1 += index.word_count;
            stats.2 += index.filter_indexes.len();
            stats
        });

        log.info(format!(
            "Total: \n  Indexed {} language{}\n  Indexed {} page{}\n  Indexed {} word{}\n  Indexed {} filter{}",
            indexes.len(),
            plural!(indexes.len()),
            stats.0,
            plural!(stats.0),
            stats.1,
            plural!(stats.1),
            stats.2,
            plural!(stats.2)
        ));

        let index_entries: Vec<_> = indexes
            .into_iter()
            .map(|indexes| async { indexes.write_files(&self.options).await })
            .collect();
        let index_entries = join_all(index_entries).await;

        output::write_common(&self.options, index_entries).await;
    }
}
