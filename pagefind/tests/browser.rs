use std::sync::{Arc, Mutex};

use chromiumoxide::cdp::browser_protocol::log::EventEntryAdded;
use chromiumoxide::listeners::EventStream;
use futures::{StreamExt, TryFutureExt};

use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::page::Page;

#[derive(Debug)]
pub struct BrowserTester {
    browser: Browser,
    page: Option<Page>,
    log_events: Arc<Mutex<Vec<String>>>,
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
            log_events: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn load_page(&mut self, url: &str) -> Result<(), Box<dyn std::error::Error>> {
        let page = self.page.insert(self.browser.new_page(url).await?);

        let console_override = vec![
            "function() {",
            "const c = console; c.events = [];",
            "let l = [c.log, c.warn, c.error, c.debug].map(e => e.bind(c));",
            "let p = (m, a) => c.events.push(`${m}: ${Array.from(a).join(' ')}`)",
            "c.log = function(){ l[0].apply(c, arguments); p('LOG', arguments); }",
            "c.warn = function(){ l[1].apply(c, arguments); p('WRN', arguments); }",
            "c.error = function(){ l[2].apply(c, arguments); p('ERR', arguments); }",
            "c.debug = function(){ l[3].apply(c, arguments); p('DBG', arguments); }",
            "}",
        ]
        .join("\n");

        let _ = page.evaluate_function(console_override).await?;

        // TODO: This block isn't working
        // https://github.com/mattsse/chromiumoxide/issues/91
        let mut events = page
            .event_listener::<chromiumoxide::cdp::browser_protocol::log::EventEntryAdded>()
            .await?;

        let event_list = Arc::clone(&self.log_events);
        let _handle = tokio::task::spawn(async move {
            loop {
                let event = events.next().await;
                if let Some(event) = event {
                    event_list.lock().unwrap().push(format!("{:#?}", event));
                }
                panic!("This block was broken, but now seems to be working? Remove the console override hack 🙂 ");
            }
        });
        // END TODO

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

    pub async fn eval(&mut self, js: &str) -> Result<(), Box<dyn std::error::Error>> {
        let _ = self
            .page
            .as_mut()
            .expect("No page launched")
            .evaluate_function(js)
            .await?;
        Ok(())
    }

    pub async fn get_logs(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let res = self
            .page
            .as_mut()
            .expect("No page launched")
            .evaluate_function("() => console.events")
            .await?
            .into_value::<Vec<String>>();

        if let Ok(logs) = res {
            Ok(logs)
        } else {
            panic!("Couldn't load logs from the browser");
        }

        // TODO: This is the real method that should be working:
        // Ok(self.log_events.lock().unwrap().iter().cloned().collect())
    }
}
