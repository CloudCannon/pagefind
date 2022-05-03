use crate::TestWorld;
use cucumber::{given, then, when};
use tokio::time::{sleep, Duration};
use warp::Filter;

#[when(regex = "^I serve the (?:\"|')(.*)(?:\"|') directory$")]
async fn serve_dir(world: &mut TestWorld, dir: String) {
    let port = world.ensure_port();
    let dir = world.tmp_file_path(&dir);
    let route = warp::path::end().and(warp::fs::dir(dir));
    let _handle = tokio::task::spawn(async move {
        warp::serve(route).run(([127, 0, 0, 1], port)).await;
    });
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

#[then(regex = "^The selector (?:\"|')(.*)(?:\"|') should contain (?:\"|')(.*)(?:\"|')$")]
async fn selector_contains(world: &mut TestWorld, selector: String, contents: String) {
    let browser = world.ensure_browser().await;
    let found_contents = browser
        .contents(&selector)
        .await
        .expect("Selector does not exist");
    assert_eq!(found_contents, contents);
}
