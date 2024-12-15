use pagefind::runner::run_indexer;

#[tokio::main]
async fn main() {
    match run_indexer().await {
        Ok(_) => { /* success */ }
        Err(msg) => {
            eprintln!("{msg}");
            std::process::exit(1);
        }
    }
}
