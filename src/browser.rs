use std::sync::Arc;

use derive_more::Debug;
use tokio::{process::Child, sync::RwLock};
use webdriverbidi::{
    model::{
        browser::CreateUserContextParameters, browsing_context::CreateParameters,
        common::EmptyParams,
    },
    session::WebDriverBiDiSession,
};

use crate::{
    browser_context::BrowserContext,
    browser_type::{BrowserType, Chromium},
    page::Page,
};

#[derive(Debug, Clone)]
pub struct Browser {
    #[debug(skip)]
    pub(crate) session: Arc<RwLock<WebDriverBiDiSession>>,
    pub(crate) _process: Option<Arc<Child>>,
}

impl Browser {
    pub fn browser_type(&self) -> impl BrowserType {
        // TODO: actual implementation
        Chromium
    }

    pub async fn close(&self) {
        self.session
            .write()
            .await
            .browser_close(EmptyParams::new())
            .await
            .unwrap();
    }

    pub fn contexts(&self) -> Vec<BrowserContext> {
        todo!()
    }

    pub fn is_connected(&self) -> bool {
        todo!()
    }

    pub async fn new_context(&self) -> BrowserContext {
        let res = self
            .session
            .write()
            .await
            .browser_create_user_context(CreateUserContextParameters {
                accept_insecure_certs: None,
                proxy: None,
                unhandled_prompt_behavior: None,
            })
            .await
            .unwrap();
        BrowserContext {
            session: self.session.clone(),
            id: res.user_context,
        }
    }

    pub async fn new_page(&self) -> Page {
        let res = self
            .session
            .write()
            .await
            .browsing_context_create(CreateParameters {
                create_type: webdriverbidi::model::browsing_context::CreateType::Tab,
                reference_context: None,
                background: None,
                user_context: None,
            })
            .await
            .unwrap();
        Page {
            session: self.session.clone(),
            id: res.context,
        }
    }

    pub async fn remove_all_listeners(&self) {
        todo!()
    }

    pub fn version(&self) -> String {
        todo!()
    }

    // TODO: on('disconnected')
}
