//! The full Pagefind indexer as run by the CLI.

use crate::options::SearchOptions;
use crate::serve;

use super::service::run_service;
use super::{PagefindInboundConfig, SearchState};
use anyhow::{bail, Result};
use std::path::PathBuf;
use std::time::Instant;
use twelf::reexports::clap::CommandFactory;
use twelf::Layer;

const CONFIGS: &[&str] = &[
    "pagefind.json",
    "pagefind.yml",
    "pagefind.yaml",
    "pagefind.toml",
];

/// Runs the full Pagefind indexing process used by the Pagefind binary.
///
/// Will log to stdout/stderr.
pub async fn run_indexer() -> Result<()> {
    let start = Instant::now();

    let matches = PagefindInboundConfig::command()
        // .ignore_errors(true)
        .get_matches();

    let mut config_layers = vec![];

    let configs: Vec<&str> = CONFIGS
        .iter()
        .filter(|c| std::path::Path::new(c).exists())
        .cloned()
        .collect();
    if configs.len() > 1 {
        let found = configs.join(", ");
        bail!("\
            Found multiple possible config files: [{found}]\n\
            Pagefind only supports loading one configuration file format, please ensure only one file exists.\
        ");
    }

    for config in configs {
        let layer_fn = if config.ends_with("json") {
            Layer::Json
        } else if config.ends_with("toml") {
            Layer::Toml
        } else if config.ends_with("yaml") || config.ends_with("yml") {
            Layer::Yaml
        } else {
            bail!("Unknown config file format {config}");
        };
        config_layers.push(layer_fn(config.into()));
    }

    config_layers.push(Layer::Env(Some("PAGEFIND_".to_string())));
    config_layers.push(Layer::Clap(matches));

    match PagefindInboundConfig::with_layers(&config_layers) {
        Ok(config) => {
            let options = match SearchOptions::load(config.clone()) {
                Ok(o) => o,
                Err(e) => return Err(e),
            };

            if config.service {
                run_service().await;
                Ok(())
            } else {
                let mut runner = SearchState::new(options.clone());
                let logger = runner.options.logger.clone();

                runner.log_start();
                // TODO: Error handling
                _ = runner
                    .fossick_many(options.site_source.clone(), options.glob)
                    .await;

                let use_old_bundle = options.config_warnings.unconfigured_bundle_output
                    && runner
                        .fossicked_pages
                        .iter()
                        .filter(|p| p.has_old_bundle_reference)
                        .next()
                        .is_some();
                if use_old_bundle {
                    logger.warn(
                            "!! Found references to a /_pagefind/ resource, running in pre-1.0 compatibility mode.",
                        );
                }

                runner.build_indexes().await?;
                _ = &runner.write_files(None).await;

                if use_old_bundle {
                    let old_bundle_location = options.site_source.join("_pagefind");
                    _ = &runner.write_files(Some(old_bundle_location)).await;
                }

                let duration = start.elapsed();

                logger.status(&format!(
                    "Finished in {}.{:03} seconds",
                    duration.as_secs(),
                    duration.subsec_millis()
                ));

                let warnings = options.config_warnings.get_strings();
                if !warnings.is_empty() {
                    logger.warn(&format!("{} configuration warning(s):", warnings.len()));

                    for warning in options.config_warnings.get_strings() {
                        logger.warn(warning);
                    }
                }

                if use_old_bundle {
                    logger.warn(&format!(
                            "\n\nWarning: Running in pre-1.0 compatibility mode.\n\
                            Pagefind 1.0 changes the default output directory from /_pagefind/ to /pagefind/\n\
                            but references to the /_pagefind/ URL were found on your site, and the output directory is unconfigured.\n\
                            To preserve your setup, the search files have been written twice, to both /_pagefind/ and /pagefind/\n\n\
                            To remove this warning, either update your script and style references to the new `/pagefind/` URL\n\
                            or run Pagefind with `--output-subdir _pagefind` to ensure pre-1.0 behaviour"
                        ));
                }

                if config.serve {
                    serve::serve_dir(PathBuf::from(options.site_source)).await;
                }
                Ok(())
            }
        }
        Err(e) => {
            let inner_err = match e {
                twelf::Error::Io(e) => {
                    format!("{}", e)
                }
                twelf::Error::Envy(e) => {
                    format!("{}", e)
                }
                twelf::Error::Json(e) => {
                    format!("{}", e)
                }
                twelf::Error::Toml(e) => {
                    format!("{}", e)
                }
                twelf::Error::Yaml(e) => {
                    format!("{}", e)
                }
                twelf::Error::Deserialize(e) => {
                    format!("{}", e)
                }
                _ => {
                    format!("Unknown Error")
                }
            };
            bail!("Error loading Pagefind config:\n{inner_err}")
        }
    }
}
