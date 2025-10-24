use std::sync::Arc;

use derive_more::Debug;
use tokio::sync::RwLock;
use webdriverbidi::{
    model::browser::{RemoveUserContextParameters, UserContext},
    session::WebDriverBiDiSession,
};

use crate::{browser::Browser, page::Page};

#[derive(Debug)]
pub struct BrowserContext {
    #[debug(skip)]
    pub(crate) session: Arc<RwLock<WebDriverBiDiSession>>,
    pub(crate) id: UserContext,
}

impl BrowserContext {
    pub async fn add_cookies(&self) {
        todo!()
    }

    pub fn browser(&self) -> Option<Browser> {
        todo!()
    }

    pub async fn clear_cookies(&self) {
        todo!()
    }

    pub async fn clear_permissions(&self) {
        todo!()
    }

    pub async fn close(&mut self) {
        self.session
            .write()
            .await
            .browser_remove_user_context(RemoveUserContextParameters {
                user_context: self.id.clone(),
            })
            .await
            .unwrap();
    }

    pub fn cookies(&self) -> Vec<()> {
        todo!()
    }

    pub fn expose_binding(&self) -> Vec<()> {
        todo!()
    }

    pub fn expose_function(&self) {
        todo!()
    }

    pub fn grant_permissions(&self) {
        todo!()
    }

    pub fn new_page(&self) -> Page {
        todo!()
    }

    pub fn pages(&self) -> Vec<Page> {
        todo!()
    }

    pub fn remove_all_listeners(&self) {
        todo!()
    }

    pub fn route(&self) {
        todo!()
    }

    pub fn route_from_har(&self) {
        todo!()
    }

    pub fn route_web_socket(&self) {
        todo!()
    }

    pub fn service_workers(&self) {
        todo!()
    }

    pub fn set_default_navigation_timeout(&self) {
        todo!()
    }

    pub fn set_default_timeout(&self) {
        todo!()
    }

    pub fn set_extra_http_headers(&self) {
        todo!()
    }

    pub fn set_geolocations(&self) {
        todo!()
    }

    pub fn set_offline(&self) {
        todo!()
    }

    pub fn storage_state(&self) {
        todo!()
    }

    pub fn unroute(&self) {
        todo!()
    }

    pub fn unroute_all(&self) {
        todo!()
    }

    pub fn wait_for_event(&self) {
        todo!()
    }

    pub fn clock(&self) {
        todo!()
    }

    pub fn request(&self) {
        todo!()
    }

    pub fn tracing(&self) {
        todo!()
    }

    // TODO: on('close')
    // TODO: on('console')
    // TODO: on('dialog')
    // TODO: on('page')
    // TODO: on('request')
    // TODO: on('requestfailed')
    // TODO: on('requestfinished')
    // TODO: on('response')
    // TODO: on('serviceworker')
    // TODO: on('weberro')
    // TODO: on('close')
}
