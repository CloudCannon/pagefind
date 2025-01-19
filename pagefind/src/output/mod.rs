use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;

use crate::index::PagefindIndexes;
use crate::{SearchOptions, PAGEFIND_VERSION};
use flate2::write::GzEncoder; // TODO: Replace flate2 with async-compression since we
use flate2::Compression; //   // require that crate for the input compression anyway.
use futures::future::join_all;
use hashbrown::HashMap;
use include_dir::{include_dir, Dir};
use minifier::js::minify;
use tokio::fs::{create_dir_all, File};
use tokio::io::AsyncWriteExt;
use tokio::time::sleep;

mod entry;

const GENERIC_WEB_WASM: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/vendor/wasm/pagefind_web_bg.unknown.",
    env!("CARGO_PKG_VERSION"),
    ".wasm.gz"
));
const WEB_WASM_FILES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/vendor/wasm");

const WEB_JS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/vendor/pagefind_web.",
    env!("CARGO_PKG_VERSION"),
    ".js"
));
const WEB_UI_JS: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/vendor/pagefind_ui.",
    env!("CARGO_PKG_VERSION"),
    ".js"
));
const WEB_UI_CSS: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/vendor/pagefind_ui.",
    env!("CARGO_PKG_VERSION"),
    ".css"
));
const WEB_MODULAR_UI_JS: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/vendor/pagefind_modular_ui.",
    env!("CARGO_PKG_VERSION"),
    ".js"
));
const WEB_MODULAR_UI_CSS: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/vendor/pagefind_modular_ui.",
    env!("CARGO_PKG_VERSION"),
    ".css"
));
const SEARCH_JS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/vendor/pagefind_public_search_api.",
    env!("CARGO_PKG_VERSION"),
    ".js"
));
const HIGHLIGHT_JS: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/vendor/pagefind_highlight.",
    env!("CARGO_PKG_VERSION"),
    ".js"
));

pub struct LanguageMeta {
    pub page_count: usize,
    pub language: String,
    pub hash: String,
    pub wasm: Option<String>,
}

pub async fn write_common_to_disk(
    language_indexes: Vec<LanguageMeta>,
    output_playground: bool,
    outdir: &PathBuf,
) {
    write_common(language_indexes, output_playground, outdir, false).await;
}

pub async fn write_common_to_memory(
    language_indexes: Vec<LanguageMeta>,
    output_playground: bool,
    outdir: &PathBuf,
) -> Vec<SyntheticFile> {
    write_common(language_indexes, output_playground, outdir, true)
        .await
        .unwrap()
}

async fn write_common(
    language_indexes: Vec<LanguageMeta>,
    output_playground: bool,
    outdir: &PathBuf,
    synthetic: bool,
) -> Option<Vec<SyntheticFile>> {
    let js_version = format!("const pagefind_version = \"{PAGEFIND_VERSION}\";");
    let mut js = vec![];
    minify(&format!("{js_version}\n{WEB_JS}\n{SEARCH_JS}"))
        .write(&mut js)
        .expect("Minifying Pagefind JS failed");

    let entry_meta = entry::PagefindEntryMeta {
        version: PAGEFIND_VERSION,
        languages: HashMap::from_iter(language_indexes.into_iter().map(|i| {
            (
                i.language,
                entry::PagefindEntryLanguage {
                    hash: i.hash,
                    wasm: i.wasm,
                    page_count: i.page_count,
                },
            )
        })),
    };
    let encoded_entry_meta = serde_json::to_string(&entry_meta).unwrap();

    let write_behavior = if synthetic {
        WriteBehavior::Synthetic
    } else {
        WriteBehavior::Disk
    };

    let mut files = vec![
        write(
            outdir.join("pagefind.js"),
            vec![&js],
            Compress::None,
            write_behavior,
        ),
        write(
            outdir.join("pagefind-highlight.js"),
            vec![HIGHLIGHT_JS],
            Compress::None,
            write_behavior,
        ),
        write(
            outdir.join("pagefind-ui.js"),
            vec![WEB_UI_JS],
            Compress::None,
            write_behavior,
        ),
        write(
            outdir.join("pagefind-ui.css"),
            vec![WEB_UI_CSS],
            Compress::None,
            write_behavior,
        ),
        write(
            outdir.join("pagefind-modular-ui.js"),
            vec![WEB_MODULAR_UI_JS],
            Compress::None,
            write_behavior,
        ),
        write(
            outdir.join("pagefind-modular-ui.css"),
            vec![WEB_MODULAR_UI_CSS],
            Compress::None,
            write_behavior,
        ),
        write(
            outdir.join("wasm.unknown.pagefind"),
            vec![GENERIC_WEB_WASM],
            Compress::None,
            write_behavior,
        ),
        write(
            outdir.join("pagefind-entry.json"),
            vec![encoded_entry_meta.as_bytes()],
            Compress::None,
            write_behavior,
        ),
    ];

    if output_playground {
        files.extend([
            write(
                outdir.join("playground/index.html"),
                vec![crate::playground::PLAYGROUND_HTML.as_bytes()],
                Compress::None,
                write_behavior,
            ),
            write(
                outdir.join("playground/pagefind-playground.js"),
                vec![crate::playground::PLAYGROUND_JS.as_bytes()],
                Compress::None,
                write_behavior,
            ),
            write(
                outdir.join("playground/pagefind-playground.css"),
                vec![crate::playground::PLAYGROUND_CSS.as_bytes()],
                Compress::None,
                write_behavior,
            ),
        ]);
    }

    let output_files = join_all(files).await;

    if synthetic {
        Some(output_files.into_iter().flatten().collect())
    } else {
        None
    }
}

impl PagefindIndexes {
    fn lang_wasm_path(&self) -> Option<String> {
        let base_language = self.language.split('-').next().unwrap();
        let wasm_path = format!(
            "pagefind_web_bg.{}.{}.wasm.gz",
            base_language,
            env!("CARGO_PKG_VERSION")
        );

        if WEB_WASM_FILES.contains(&wasm_path) {
            Some(wasm_path)
        } else {
            None
        }
    }

    pub fn get_lang_meta(&self, options: &SearchOptions) -> LanguageMeta {
        let mut wasm_file = None;

        if self.language != "unknown" {
            if self.lang_wasm_path().is_some() {
                wasm_file = Some(self.language.to_string());
            } else {
                options.logger.v_warn(format!(
                    "Note: Pagefind doesn't support stemming for the language {}. \n\
                    Search will still work, but will not match across root words.",
                    self.language
                ));
            }
        }

        LanguageMeta {
            page_count: self.fragments.len(),
            language: self.language.clone(),
            hash: self.meta_index.0.clone(),
            wasm: wasm_file,
        }
    }

    pub async fn write_files_to_disk(&self, options: &SearchOptions, outdir: &PathBuf) {
        self.write_files(options, outdir, false).await;
    }

    pub async fn write_files_to_memory(
        &self,
        options: &SearchOptions,
        outdir: &PathBuf,
    ) -> Vec<SyntheticFile> {
        self.write_files(options, outdir, true).await.unwrap()
    }

    async fn write_files(
        &self,
        options: &SearchOptions,
        outdir: &PathBuf,
        synthetic: bool,
    ) -> Option<Vec<SyntheticFile>> {
        let immutable_write_behaviour = if synthetic {
            WriteBehavior::Synthetic
        } else {
            WriteBehavior::Immutable
        };

        let mut files = vec![write(
            outdir.join(format!("pagefind.{}.pf_meta", &self.meta_index.0)),
            vec![&self.meta_index.1],
            Compress::GZ,
            immutable_write_behaviour,
        )];

        if self.language != "unknown" {
            if let Some(wasm_path) = self.lang_wasm_path() {
                files.push(write(
                    outdir.join(format!("wasm.{}.pagefind", self.language)),
                    vec![WEB_WASM_FILES
                        .get_file(wasm_path)
                        .expect("WASM should exist")
                        .contents()],
                    Compress::None,
                    if synthetic {
                        WriteBehavior::Synthetic
                    } else {
                        WriteBehavior::Disk
                    },
                ));
            } else {
                options.logger.v_warn(format!(
                    "Note: Pagefind doesn't support stemming for the language {}. \n\
                    Search will still work, but will not match across root words.",
                    self.language
                ));
            }
        }

        files.extend(self.fragments.iter().map(|(hash, fragment)| {
            write(
                outdir.join(format!("fragment/{}.pf_fragment", hash)),
                vec![fragment.as_bytes()],
                Compress::GZ,
                immutable_write_behaviour,
            )
        }));

        files.extend(self.word_indexes.iter().map(|(hash, index)| {
            write(
                outdir.join(format!("index/{}.pf_index", hash)),
                vec![index],
                Compress::GZ,
                immutable_write_behaviour,
            )
        }));

        files.extend(self.filter_indexes.iter().map(|(hash, index)| {
            write(
                outdir.join(format!("filter/{}.pf_filter", hash)),
                vec![index],
                Compress::GZ,
                immutable_write_behaviour,
            )
        }));

        let output_files = join_all(files).await;
        if synthetic {
            Some(output_files.into_iter().flatten().collect())
        } else {
            None
        }
    }
}

#[derive(Copy, Clone)]
enum Compress {
    GZ,
    None,
}

#[derive(Copy, Clone)]
enum WriteBehavior {
    Synthetic,
    Immutable,
    Disk,
}

#[derive(Clone)]
pub struct SyntheticFile {
    pub filename: PathBuf,
    pub contents: Vec<u8>,
}

async fn write(
    filename: PathBuf,
    content_chunks: Vec<&[u8]>,
    compression: Compress,
    write_behavior: WriteBehavior,
) -> Option<SyntheticFile> {
    let mut file = None;

    match write_behavior {
        WriteBehavior::Synthetic => {}
        // For "immutable" (hashed) files, don't re-write them as the contents _should_ be unchanged.
        WriteBehavior::Immutable if filename.exists() => return None,
        WriteBehavior::Immutable | WriteBehavior::Disk => {
            if let Some(parent) = filename.parent() {
                create_dir_all(parent).await.unwrap();
            }

            let mut output_file = File::create(&filename).await;
            while output_file.is_err() {
                sleep(Duration::from_millis(100)).await;
                output_file = File::create(&filename).await;
            }
            file = output_file.ok();
        }
    };

    match compression {
        Compress::GZ => {
            let mut gz = GzEncoder::new(Vec::new(), Compression::best());
            for chunk in content_chunks {
                gz.write_all(b"pagefind_dcd").unwrap();
                gz.write_all(chunk).unwrap();
            }
            if let Ok(contents) = gz.finish() {
                if let Some(mut file) = file {
                    file.write_all(&contents).await.unwrap();
                } else {
                    return Some(SyntheticFile { filename, contents });
                }
            }
            None
        }
        Compress::None => {
            if let Some(mut file) = file {
                for chunk in content_chunks {
                    file.write_all(chunk).await.unwrap();
                }
                None
            } else {
                return Some(SyntheticFile {
                    filename,
                    contents: content_chunks.into_iter().flatten().cloned().collect(),
                });
            }
        }
    }
}
