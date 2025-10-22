use downstage::{BrowserType, Chromium};

#[tokio::main]
async fn main() {
    let browser = Chromium.launch().await;
    let page = browser.new_page().await;
    page.goto("https://example.com").await;
    browser.close().await;
}
