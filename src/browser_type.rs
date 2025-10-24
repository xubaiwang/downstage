use std::{process::Stdio, sync::Arc};

use regex::Regex;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Child,
    sync::RwLock,
};
use webdriverbidi::{session::WebDriverBiDiSession, webdriver::capabilities::CapabilitiesRequest};

use crate::{
    browser::Browser,
    error::{Error, Result},
};

pub trait BrowserType: std::fmt::Debug + Clone {
    fn name() -> &'static str;
    fn connect() -> impl Future<Output = Result<Browser<Self>>>;
    fn launch() -> impl Future<Output = Result<Browser<Self>>>;
}

#[derive(Debug, Clone, Copy)]
pub struct Chromium;

impl BrowserType for Chromium {
    fn name() -> &'static str {
        "chromium"
    }

    async fn connect() -> Result<Browser<Self>> {
        todo!()
    }

    async fn launch() -> Result<Browser<Self>> {
        let mut child = tokio::process::Command::new("chromedriver")
            .stdout(Stdio::piped())
            .kill_on_drop(true)
            .spawn()?;

        let port = wait_for_chromedriver(&mut child).await?;
        let mut session =
            WebDriverBiDiSession::new("localhost".into(), port, CapabilitiesRequest::default());
        session.start().await?;
        Ok(Browser {
            session: Arc::new(RwLock::new(session)),
            _process: Some(Arc::new(child)),
            browser_type: Self,
        })
    }
}

/// Wait for chromedriver to actually startup and establish.
///
/// Returns the port.
async fn wait_for_chromedriver(child: &mut Child) -> Result<u16> {
    let stdout = child
        .stdout
        .take()
        .expect("Child process should have stdout");

    let mut reader = BufReader::new(stdout);
    let mut line = String::new();

    let re = Regex::new(r"ChromeDriver was started successfully on port (\d+).")
        .expect("regex should compile");

    loop {
        line.clear();
        let bytes_read = reader.read_line(&mut line).await?;
        if bytes_read == 0 {
            return Err(Error::Spawn);
        }

        if let Some(capture) = re.captures(&line) {
            let port = capture
                .get(1)
                .expect("capture should have one group")
                .as_str()
                .parse()
                .expect("regex should capture number");
            return Ok(port);
        }
    }
}
