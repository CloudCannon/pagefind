use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;

use crate::index::PagefindIndexes;
use crate::SearchOptions;
use flate2::write::GzEncoder;
use flate2::Compression;
use futures::future::join_all;
use hashbrown::HashMap;
use include_dir::{include_dir, Dir};
use minifier::js::minify;
use tokio::fs::{create_dir_all, File};
use tokio::io::AsyncWriteExt;
use tokio::time::sleep;

mod entry;

const PAGEFIND_VERSION: &str = env!("CARGO_PKG_VERSION");

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
const GUNZIP_JS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/output/stubs/gz.js"
));
const SEARCH_JS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/output/stubs/search.js"
));

pub async fn write_common(options: &SearchOptions, language_indexes: Vec<(String, String)>) {
    let outdir = options.source.join(&options.bundle_dir);

    let js_version = format!("const pagefind_version = \"{PAGEFIND_VERSION}\";");
    let js = minify(&format!("{js_version}\n{WEB_JS}\n{GUNZIP_JS}\n{SEARCH_JS}"));

    let entry_meta = entry::PagefindEntryMeta {
        version: PAGEFIND_VERSION,
        languages: HashMap::from_iter(language_indexes),
    };
    let encoded_entry_meta = serde_json::to_string(&entry_meta).unwrap();

    let files = vec![
        write(
            outdir.join("pagefind.js"),
            vec![js.as_bytes()],
            Compress::None,
            WriteBehavior::None,
        ),
        write(
            outdir.join("pagefind-ui.js"),
            vec![WEB_UI_JS],
            Compress::None,
            WriteBehavior::None,
        ),
        write(
            outdir.join("pagefind-ui.css"),
            vec![WEB_UI_CSS],
            Compress::None,
            WriteBehavior::None,
        ),
        write(
            outdir.join("wasm.unknown.pagefind"),
            vec![GENERIC_WEB_WASM],
            Compress::None,
            WriteBehavior::None,
        ),
        write(
            outdir.join("pagefind-entry.json"),
            vec![encoded_entry_meta.as_bytes()],
            Compress::None,
            WriteBehavior::None,
        ),
    ];

    join_all(files).await;
}

impl PagefindIndexes {
    pub async fn write_files(self, options: &SearchOptions) {
        let outdir = options.source.join(&options.bundle_dir);

        let mut files = vec![write(
            outdir.join(format!("pagefind.{}.pf_meta", &self.meta_index.0)),
            vec![&self.meta_index.1],
            Compress::GZ,
            WriteBehavior::Immutable,
        )];

        if self.language != "unknown" {
            if let Some(wasm) = WEB_WASM_FILES.get_file(format!(
                "pagefind_web_bg.{}.{}.wasm.gz",
                self.language,
                env!("CARGO_PKG_VERSION")
            )) {
                files.push(write(
                    outdir.join(format!("wasm.{}.pagefind", self.language)),
                    vec![wasm.contents()],
                    Compress::None,
                    WriteBehavior::None,
                ));
            } else {
                eprintln!("TODO: Error: No wasm for {}", self.language);
            }
        }

        files.extend(self.fragments.iter().map(|(hash, fragment)| {
            write(
                outdir.join(format!("fragment/{}.pf_fragment", hash)),
                vec![fragment.as_bytes()],
                Compress::GZ,
                WriteBehavior::Immutable,
            )
        }));

        files.extend(self.word_indexes.iter().map(|(hash, index)| {
            write(
                outdir.join(format!("index/{}.pf_index", hash)),
                vec![index],
                Compress::GZ,
                WriteBehavior::Immutable,
            )
        }));

        files.extend(self.filter_indexes.iter().map(|(hash, index)| {
            write(
                outdir.join(format!("filter/{}.pf_filter", hash)),
                vec![index],
                Compress::GZ,
                WriteBehavior::Immutable,
            )
        }));

        join_all(files).await;
    }
}

enum Compress {
    GZ,
    None,
}

enum WriteBehavior {
    Immutable,
    None,
}

async fn write(
    filename: PathBuf,
    contents: Vec<&[u8]>,
    compression: Compress,
    write_behavior: WriteBehavior,
) {
    // For "immutable" (hashed) files, don't re-write them as the contents _should_ be unchanged.
    if matches!(write_behavior, WriteBehavior::Immutable) && filename.exists() {
        return;
    }

    if let Some(parent) = filename.parent() {
        create_dir_all(parent).await.unwrap();
    }

    let mut file = File::create(&filename).await;
    while file.is_err() {
        sleep(Duration::from_millis(100)).await;
        file = File::create(&filename).await;
    }
    let mut file = file.unwrap();

    match compression {
        Compress::GZ => {
            let mut gz = GzEncoder::new(Vec::new(), Compression::best());
            for chunk in contents {
                gz.write_all(b"pagefind_dcd").unwrap();
                gz.write_all(chunk).unwrap();
            }
            if let Ok(bytes) = gz.finish() {
                file.write_all(&bytes).await.unwrap();
            }
        }
        Compress::None => {
            for chunk in contents {
                file.write_all(chunk).await.unwrap();
            }
        }
    }
}
