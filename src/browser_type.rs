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

// NOTE: keep `&self` here for dyn compatibility.
pub trait BrowserType {
    fn name(&self) -> &'static str;
    fn connect(&self) -> impl Future<Output = Result<Browser>>;
    fn launch(&self) -> impl Future<Output = Result<Browser>>;
}

#[derive(Debug, Clone, Copy)]
pub struct Chromium;

impl BrowserType for Chromium {
    fn name(&self) -> &'static str {
        "chromium"
    }

    async fn connect(&self) -> Result<Browser> {
        todo!()
    }

    async fn launch(&self) -> Result<Browser> {
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
