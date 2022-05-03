use futures::StreamExt;

use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::page::Page;

#[derive(Debug)]
pub struct BrowserTester {
    browser: Browser,
    page: Option<Page>,
}

impl BrowserTester {
    pub async fn new(webdriver_url: &str) -> Self {
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
        }
    }

    pub async fn load_page(&mut self, url: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.page = Some(self.browser.new_page(url).await?);
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
        let text = self
            .page
            .as_mut()
            .expect("No page launched")
            .find_element(selector)
            .await?
            .inner_text()
            .await?;

        Ok(text.expect("Element contained inner text"))
    }
}
