use std::time::Duration;

use downstage::{BrowserType, Chromium};

#[tokio::main]
async fn main() {
    let browser = Chromium.launch().await;
    let page = browser.new_page().await;
    page.goto("https://example.com").await;

    tokio::time::sleep(Duration::from_secs(10)).await;

    browser.close().await;
}
