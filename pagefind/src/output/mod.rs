use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;

use crate::index::PagefindIndexes;
use crate::SearchOptions;
use flate2::write::GzEncoder;
use flate2::Compression;
use futures::future::join_all;
use minifier::js::minify;
use tokio::fs::{create_dir_all, File};
use tokio::io::AsyncWriteExt;
use tokio::time::sleep;

const WEB_WASM: &[u8] = include_bytes!(concat!(
    "../../vendor/pagefind_web_bg.",
    env!("CARGO_PKG_VERSION"),
    ".wasm"
));
const WEB_JS: &str = include_str!(concat!(
    "../../vendor/pagefind_web.",
    env!("CARGO_PKG_VERSION"),
    ".js"
));
const WEB_UI_JS: &[u8] = include_bytes!(concat!(
    "../../vendor/pagefind_ui.",
    env!("CARGO_PKG_VERSION"),
    ".js"
));
const WEB_UI_CSS: &[u8] = include_bytes!(concat!(
    "../../vendor/pagefind_ui.",
    env!("CARGO_PKG_VERSION"),
    ".css"
));
const GUNZIP_JS: &str = include_str!("./stubs/gz.js");
const SEARCH_JS: &str = include_str!("./stubs/search.js");

impl PagefindIndexes {
    pub async fn write_files(self, options: &SearchOptions) {
        let outdir = options.source.join(&options.bundle_dir);

        let fragment_data: Vec<_> = self
            .fragments
            .iter()
            .map(|(hash, fragment)| (hash, serde_json::to_string(&fragment.data).unwrap()))
            .collect();

        let js = minify(&format!("{}\n{}\n{}", WEB_JS, GUNZIP_JS, SEARCH_JS));

        let mut files = vec![
            write(
                outdir.join("pagefind.js"),
                vec![js.as_bytes()],
                Compress::None,
            ),
            write(
                outdir.join("pagefind-ui.js"),
                vec![WEB_UI_JS],
                Compress::None,
            ),
            write(
                outdir.join("pagefind-ui.css"),
                vec![WEB_UI_CSS],
                Compress::None,
            ),
            write(outdir.join("wasm.pagefind"), vec![WEB_WASM], Compress::GZ),
            write(
                outdir.join("pagefind.pf_meta"),
                vec![&self.meta_index],
                Compress::GZ,
            ),
        ];

        files.extend(fragment_data.iter().map(|(hash, fragment)| {
            write(
                outdir.join(format!("fragment/{}.pf_fragment", hash)),
                vec![fragment.as_bytes()],
                Compress::GZ,
            )
        }));

        files.extend(self.word_indexes.iter().map(|(hash, index)| {
            write(
                outdir.join(format!("index/{}.pf_index", hash)),
                vec![index],
                Compress::GZ,
            )
        }));

        files.extend(self.filter_indexes.iter().map(|(hash, index)| {
            write(
                outdir.join(format!("filter/{}.pf_filter", hash)),
                vec![index],
                Compress::GZ,
            )
        }));

        join_all(files).await;
    }
}

enum Compress {
    GZ,
    None,
}

async fn write(filename: PathBuf, contents: Vec<&[u8]>, compression: Compress) {
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
