use chromiumoxide::cdp::browser_protocol::log::EventEntryAdded;
use chromiumoxide::listeners::EventStream;
use futures::{StreamExt, TryFutureExt};

use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::page::Page;

#[derive(Debug)]
pub struct BrowserTester {
    browser: Browser,
    page: Option<Page>,
    logs: Option<EventStream<EventEntryAdded>>,
}

impl BrowserTester {
    pub async fn new() -> Self {
        let (browser, mut handler) = Browser::launch(BrowserConfig::builder().build().unwrap())
            .await
            .unwrap();

        let _handle = tokio::task::spawn(async move {
            loop {
                let _ = handler.next().await.unwrap();
            }
        });
        Self {
            browser,
            page: None,
            logs: None,
        }
    }

    pub async fn load_page(&mut self, url: &str) -> Result<(), Box<dyn std::error::Error>> {
        let page = self.page.insert(self.browser.new_page(url).await?);

        let events = page.event_listener::<EventEntryAdded>().await?;
        self.logs = Some(events);

        Ok(())
    }

    pub async fn click(&mut self, selector: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.page
            .as_mut()
            .expect("No page launched")
            .find_element(selector)
            .await?
            .click()
            .await?;

        Ok(())
    }

    pub async fn contents(&mut self, selector: &str) -> Result<String, Box<dyn std::error::Error>> {
        let el = self
            .page
            .as_mut()
            .expect("No page launched")
            .find_element(selector)
            .await
            .expect("Selector does not exist");

        let contents = el
            .inner_html()
            .await
            .expect("Element did not have Inner HTML");

        Ok(contents.expect("Element was empty"))
    }
}
