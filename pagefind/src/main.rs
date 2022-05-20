use clap::{App, Arg};
use pagefind::{SearchOptions, SearchState};
use std::time::Instant;

#[tokio::main]
async fn main() {
    let start = Instant::now();

    let matches = App::new("Untitled Search Project")
        .version(option_env!("RELEASE_VERSION").unwrap_or("Development"))
        .author("CloudCannon")
        .about("Pending Description")
        .arg(
            Arg::with_name("source")
                .short("s")
                .long("source")
                .default_value("public")
                .help("The location of your build static website"),
        )
        .arg(
            Arg::with_name("bundle_dir")
                .short("b")
                .long("bundle_dir")
                .default_value("_pagefind")
                .help("Where to output the search files, relative to source"),
        )
        .get_matches();

    // Run
    let options = SearchOptions::from(&matches);
    let mut runner = SearchState::new(options);

    runner.run().await;

    let duration = start.elapsed();
    println!(
        "Finished in {}.{} seconds",
        duration.as_secs(),
        duration.subsec_millis()
    );
}
