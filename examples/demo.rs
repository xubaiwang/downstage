use std::time::Duration;

use downstage::browser_type::{BrowserType, Chromium};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let browser = Chromium.launch().await?;
    let page = browser.new_page().await?;
    page.goto("https://example.com").await?;

    tokio::time::sleep(Duration::from_secs(5)).await;

    browser.close().await?;

    Ok(())
}
