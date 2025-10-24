use std::sync::Arc;

use derive_more::Debug;
use tokio::{process::Child, sync::RwLock};
use webdriverbidi::{
    events::EventType,
    model::{
        browser::{CreateUserContextParameters, GetUserContextsResult},
        browsing_context::CreateParameters,
        common::EmptyParams,
    },
    session::WebDriverBiDiSession,
};

use crate::{
    browser_context::BrowserContext, browser_type::BrowserType, error::Result, page::Page,
};

#[derive(Debug, Clone)]
pub struct Browser<T: BrowserType> {
    #[debug(skip)]
    pub(crate) session: Arc<RwLock<WebDriverBiDiSession>>,
    pub(crate) _process: Option<Arc<Child>>,
    pub(crate) browser_type: T,
}

impl<T: BrowserType> Browser<T> {
    pub fn browser_type(&self) -> impl BrowserType {
        self.browser_type.clone()
    }

    pub async fn close(&self) -> Result<()> {
        self.session
            .write()
            .await
            .browser_close(EmptyParams::new())
            .await?;
        Ok(())
    }

    pub async fn contexts(&self) -> Result<Vec<BrowserContext<T>>> {
        let GetUserContextsResult { user_contexts } = self
            .session
            .write()
            .await
            .browser_get_user_contexts(EmptyParams::new())
            .await?;
        Ok(user_contexts
            .into_iter()
            .map(|info| BrowserContext {
                session: self.session.clone(),
                id: info.user_context,
                browser: self.clone(),
            })
            .collect())
    }

    pub fn is_connected(&self) -> bool {
        unimplemented!()
    }

    pub async fn new_context(&self) -> Result<BrowserContext<T>> {
        let res = self
            .session
            .write()
            .await
            .browser_create_user_context(CreateUserContextParameters {
                accept_insecure_certs: None,
                proxy: None,
                unhandled_prompt_behavior: None,
            })
            .await?;
        Ok(BrowserContext {
            session: self.session.clone(),
            id: res.user_context,
            browser: self.clone(),
        })
    }

    pub async fn new_page(&self) -> Result<Page> {
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
            .await?;
        Ok(Page {
            session: self.session.clone(),
            id: res.context,
        })
    }

    pub async fn remove_all_listeners(&self) {
        unimplemented!()
    }

    pub fn version(&self) -> String {
        unimplemented!()
    }

    // TODO: on('disconnected')
}
