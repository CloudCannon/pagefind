use crate::TestWorld;
use actix_files as fs;
use actix_web::{App, HttpServer};
use cucumber::gherkin::Step;
use cucumber::{then, when};
use tokio::time::{sleep, Duration};

#[when(regex = "^I serve the (?:\"|')(.*)(?:\"|') directory$")]
async fn serve_dir(world: &mut TestWorld, dir: String) {
    let mut attempts = 0;
    let mut running = false;
    while !running && attempts < 5 {
        let port = world.ensure_port();
        let dir = world.tmp_file_path(&dir);
        match HttpServer::new(move || {
            App::new().service(fs::Files::new("/", &dir).index_file("index.html"))
        })
        .bind(("127.0.0.1", port))
        {
            Ok(bound) => {
                let server = bound.run();
                let handle = server.handle();
                world.handles.push(handle);
                world
                    .threads
                    .push(tokio::task::spawn(async { server.await }));
                running = true;
            }
            Err(_) => {
                world.purge_port();
                attempts += 1;
            }
        }
    }

    assert!(running);
    // Wait a beat to make sure the server is ready to roll
    sleep(Duration::from_millis(100)).await;
}

#[when(regex = "^I load (?:\"|')(.*)(?:\"|')$")]
async fn load_page(world: &mut TestWorld, path: String) {
    let url = format!("http://localhost:{}{}", world.ensure_port(), path);
    let browser = world.ensure_browser().await;
    browser.load_page(&url).await.expect("Loading URL failed");
}

#[when(regex = "^I click (?:\"|')(.*)(?:\"|')$")]
async fn click_selector(world: &mut TestWorld, selector: String) {
    let browser = world.ensure_browser().await;
    browser
        .click(&selector)
        .await
        .expect("Selector did not exist");
}

#[when(regex = "^I evaluate:$")]
async fn eval_js(world: &mut TestWorld, step: &Step) {
    match &step.docstring {
        Some(contents) => {
            let browser = world.ensure_browser().await;
            browser.eval(contents).await.expect("Javascript crashed");
        }
        None => panic!("`{}` step expected a docstring", step.value),
    }
}

#[then(regex = "^The selector (?:\"|')(.*)(?:\"|') should exist$")]
async fn selector_exists(world: &mut TestWorld, selector: String) {
    let browser = world.ensure_browser().await;
    browser
        .selector_exists(&selector)
        .await
        .expect("Selector loaded");
}

#[then(regex = "^The selector (?:\"|')(.*)(?:\"|') should contain (?:\"|')(.*)(?:\"|')$")]
async fn selector_contains(world: &mut TestWorld, selector: String, contents: String) {
    let browser = world.ensure_browser().await;
    let found_contents = browser
        .contents(&selector)
        .await
        .expect("Selector does not exist");
    assert_eq!(found_contents, contents);
}

#[then(regex = "^There should be no logs$")]
async fn no_logs(world: &mut TestWorld) {
    let browser = world.ensure_browser().await;
    let logs = browser.get_logs().await.expect("Page is loaded");
    if !logs.is_empty() {
        panic!(
            "No logs were expected, but logs were found:\n\n{}",
            logs.join("\n")
        );
    }
}
