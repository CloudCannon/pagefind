use std::{cmp::Ordering, path::PathBuf};

pub use fossick::{FossickedData, Fossicker};
use futures::future::join_all;
use hashbrown::HashMap;
use index::PagefindIndexes;
use logging::Logger;
pub use options::{PagefindInboundConfig, SearchOptions};
use output::SyntheticFile;
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
pub mod service;
mod utils;

pub struct SearchState {
    pub options: SearchOptions,
    pub fossicked_pages: Vec<FossickedData>,
    pub built_indexes: Vec<PagefindIndexes>,
}

impl SearchState {
    pub fn new(options: SearchOptions) -> Self {
        Self {
            options,
            fossicked_pages: vec![],
            built_indexes: vec![],
        }
    }

    pub async fn walk_for_files(&mut self, dir: PathBuf, glob: String) -> Vec<Fossicker> {
        let log = &self.options.logger;

        log.status("[Walking source directory]");
        if let Ok(glob) = Glob::new(&glob) {
            glob.walk(&dir)
                .filter_map(Result::ok)
                .map(WalkEntry::into_path)
                .map(|file_path| Fossicker::new_relative_to(file_path, dir.clone()))
                .collect()
        } else {
            log.error(format!(
                "Error: Provided glob \"{}\" did not parse as a valid glob.",
                self.options.glob
            ));
            // TODO: Bubble this error back to the Node API if applicable
            std::process::exit(1);
        }
    }

    pub async fn fossick_many(&mut self, dir: PathBuf, glob: String) -> Result<usize, ()> {
        let files = self.walk_for_files(dir.clone(), glob).await;
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

        let existing_page_count = self.fossicked_pages.len();
        self.fossicked_pages
            .extend(join_all(results).await.into_iter().flatten());

        Ok(self.fossicked_pages.len() - existing_page_count)
    }

    pub async fn fossick_one(&mut self, file: Fossicker) -> Result<FossickedData, ()> {
        let result = file.fossick(&self.options).await;
        self.options
            .logger
            .info(format!("Indexing file into: {:#?}", self.fossicked_pages));
        if let Ok(result) = result.clone() {
            let existing = self
                .fossicked_pages
                .iter()
                .position(|page| page.url == result.url);
            if let Some(existing) = existing {
                *self.fossicked_pages.get_mut(existing).unwrap() = result;
            } else {
                self.fossicked_pages.push(result);
            }
        }
        self.options
            .logger
            .info(format!("Now: {:#?}", self.fossicked_pages));
        result
    }

    pub async fn build_indexes(&mut self) {
        let log = &self.options.logger;

        let used_custom_body = self.fossicked_pages.iter().any(|page| page.has_custom_body);
        if used_custom_body {
            log.info("Found a data-pagefind-body element on the site.\n↳ Ignoring pages without this tag.");
        } else {
            log.info(
                "Did not find a data-pagefind-body element on the site.\n↳ Indexing all <body> elements on the site."
            );
        }

        if self.options.root_selector == "html" {
            let pages_without_html = self
                .fossicked_pages
                .iter()
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

        let pages_with_data = self.fossicked_pages.iter().filter(|d| {
            if used_custom_body && !d.has_custom_body && !d.force_inclusion {
                return false;
            }
            !d.word_data.is_empty()
        });

        let mut language_map: HashMap<String, Vec<fossick::FossickedData>> = HashMap::new();
        for page in pages_with_data {
            let language = page.language.clone();
            if let Some(lang_pages) = language_map.get_mut(&language) {
                lang_pages.push(page.clone());
            } else {
                language_map.insert(language, vec![page.clone()]);
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
            if !language_map.is_empty() {
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
            .map(|(language, pages)| async { build_indexes(pages, language, &self.options).await })
            .collect();
        self.built_indexes = join_all(indexes).await;

        let stats = self.built_indexes.iter().fold((0, 0, 0, 0), |mut stats, index| {
            log.v_info(format!(
                "Language {}: \n  Indexed {} page{}\n  Indexed {} word{}\n  Indexed {} filter{}\n  Indexed {} sort{}\n",
                index.language,
                index.fragments.len(),
                plural!(index.fragments.len()),
                index.word_count,
                plural!(index.word_count),
                index.filter_indexes.len(),
                plural!(index.filter_indexes.len()),
                index.sorts.len(),
                plural!(index.sorts.len())
            ));

            #[cfg(not(feature = "extended"))]
            match index.language.split('-').next() {
                Some("zh") => log.warn("⚠ Indexing Chinese in non-extended mode. \n\
                                        In this mode, Pagefind will not segment words that are not whitespace separated. \n\
                                        Running the extended Pagefind binary will include this segmentation. \n\
                                        Either download the pagefind_extended binary, or run via npx pagefind."),
                Some("ja") => log.warn("⚠ Indexing Japanese in non-extended mode. \n\
                                        In this mode, Pagefind will not segment words that are not whitespace separated. \n\
                                        Running the extended Pagefind binary will include this segmentation. \n\
                                        Either download the pagefind_extended binary, or run via npx pagefind."),
                _ => {}
            };

            stats.0 += index.fragments.len();
            stats.1 += index.word_count;
            stats.2 += index.filter_indexes.len();
            stats.3 += index.sorts.len();
            stats
        });

        log.info(format!(
            "Total: \n  Indexed {} language{}\n  Indexed {} page{}\n  Indexed {} word{}\n  Indexed {} filter{}\n  Indexed {} sort{}",
            self.built_indexes.len(),
            plural!(self.built_indexes.len()),
            stats.0,
            plural!(stats.0),
            stats.1,
            plural!(stats.1),
            stats.2,
            plural!(stats.2),
            stats.3,
            plural!(stats.3)
        ));

        if stats.1 == 0 && !self.options.running_as_service {
            log.error(
                "Error: Pagefind wasn't able to build an index. \n\
                Most likely, the directory passed to Pagefind was empty \
                or did not contain any html files.",
            );
            std::process::exit(1);
        }
    }

    pub async fn write_files(&self, custom_outdir: Option<PathBuf>) -> PathBuf {
        let outdir = custom_outdir.unwrap_or(self.options.bundle_output.clone());

        let index_entries: Vec<_> = self
            .built_indexes
            .iter()
            .map(|indexes| indexes.get_lang_meta(&self.options))
            .collect();

        join_all(
            self.built_indexes
                .iter()
                .map(|indexes| async { indexes.write_files_to_disk(&self.options, &outdir).await }),
        )
        .await;

        output::write_common_to_disk(&self.options, index_entries, &outdir).await;

        outdir
    }

    pub async fn get_files(&self) -> Vec<SyntheticFile> {
        let outdir = &self.options.bundle_output;

        let index_entries: Vec<_> = self
            .built_indexes
            .iter()
            .map(|indexes| indexes.get_lang_meta(&self.options))
            .collect();

        let mut files: Vec<_> =
            join_all(self.built_indexes.iter().map(|indexes| async {
                indexes.write_files_to_memory(&self.options, outdir).await
            }))
            .await
            .into_iter()
            .flatten()
            .collect();

        files.extend(
            output::write_common_to_memory(&self.options, index_entries, outdir)
                .await
                .into_iter(),
        );

        // SyntheticFiles should only return the relative path to the file
        // _within_ the bundle directory — placing them in a final location
        // is left to the API consumer.
        for file in files.iter_mut() {
            if let Ok(relative_path) = file.filename.strip_prefix(outdir) {
                file.filename = relative_path.to_path_buf();
            }
        }

        files
    }

    pub fn log_start(&self) {
        let log = &self.options.logger;

        #[cfg(not(feature = "extended"))]
        log.status(&format!("Running Pagefind v{}", self.options.version));
        #[cfg(feature = "extended")]
        log.status(&format!(
            "Running Pagefind v{} (Extended)",
            self.options.version
        ));
        log.v_info("Running in verbose mode");

        log.info(format!(
            "Running from: {:?}",
            self.options.working_directory
        ));
        log.info(format!(
            "Source:       {:?}",
            self.options
                .site_source
                .strip_prefix(&self.options.working_directory)
                .unwrap_or(&self.options.site_source)
        ));
        log.info(format!(
            "Output:       {:?}",
            self.options
                .bundle_output
                .strip_prefix(&self.options.working_directory)
                .unwrap_or(&self.options.bundle_output)
        ));
    }
}
