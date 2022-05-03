use std::cell::RefCell;
use std::str::FromStr;

use cucumber::gherkin::Step;
use cucumber::{given, then, when};
use json_dotpath::DotPaths;
use kuchiki::iter::{Descendants, Elements, Select};
use kuchiki::traits::TendrilSink;
use kuchiki::{Attributes, ElementData, NodeDataRef, NodeRef};
use regex::Regex;
use serde_json::Value;

use crate::BrowserTester;
use crate::TestWorld;

#[when(regex = "^I load (?:\"|')(.*)(?:\"|')$")]
async fn load_page(world: &mut TestWorld, url: String) {
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
        .expect("Selector did not exist");
    assert_eq!(found_contents, contents);
}
