use pagefind::service::run_service;
use pagefind::{PagefindInboundConfig, SearchOptions, SearchState};
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

#[tokio::main]
async fn main() {
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
        eprintln!(
            "Found multiple possible config files: [{}]",
            configs.join(", ")
        );
        eprintln!("Pagefind only supports loading one configuration file format, please ensure only one file exists.");
        std::process::exit(1);
    }

    for config in configs {
        let layer_fn = if config.ends_with("json") {
            Layer::Json
        } else if config.ends_with("toml") {
            Layer::Toml
        } else if config.ends_with("yaml") || config.ends_with("yml") {
            Layer::Yaml
        } else {
            eprintln!("Unknown config file format {}", config);
            std::process::exit(1);
        };
        config_layers.push(layer_fn(config.into()));
    }

    config_layers.push(Layer::Env(Some("PAGEFIND_".to_string())));
    config_layers.push(Layer::Clap(matches));

    match PagefindInboundConfig::with_layers(&config_layers) {
        Ok(config) => {
            if let Ok(options) = SearchOptions::load(config.clone()) {
                if config.service {
                    run_service().await;
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

                    runner.build_indexes().await;
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
                        pagefind::serve::serve_dir(PathBuf::from(options.site_source)).await;
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error loading Pagefind config:");
            match e {
                twelf::Error::Io(e) => {
                    eprintln!("{}", e);
                }
                twelf::Error::Envy(e) => {
                    eprintln!("{}", e);
                }
                twelf::Error::Json(e) => {
                    eprintln!("{}", e);
                }
                twelf::Error::Toml(e) => {
                    eprintln!("{}", e);
                }
                twelf::Error::Yaml(e) => {
                    eprintln!("{}", e);
                }
                twelf::Error::Deserialize(e) => {
                    eprintln!("{}", e);
                }
                _ => {
                    eprintln!("Unknown Error");
                }
            }
            std::process::exit(1);
        }
    }
}
