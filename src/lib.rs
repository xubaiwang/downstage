/// BrowserType provides methods to launch a specific browser instance or connect to an existing one.
pub trait BrowserType {
    fn name(&self) -> &'static str;
    fn connect(&self) -> impl Future<Output = Browser>;
    fn launch(&self) -> impl Future<Output = Browser>;
}

#[derive(Debug, Clone, Copy)]
pub struct Chromium;

impl BrowserType for Chromium {
    fn name(&self) -> &'static str {
        "chromium"
    }

    async fn connect(&self) -> Browser {
        todo!()
    }

    async fn launch(&self) -> Browser {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct Browser;

impl Browser {
    pub fn browser_type(&self) -> impl BrowserType {
        // TODO: actual implementation
        Chromium
    }

    pub async fn close(&self) {
        todo!()
    }

    pub fn contexts(&self) -> Vec<BrowserContext> {
        todo!()
    }

    pub fn is_connected(&self) -> bool {
        todo!()
    }

    pub async fn new_page(&self) -> Page {
        todo!()
    }

    pub async fn remove_all_listeners(&self) {
        todo!()
    }

    pub fn version(&self) -> String {
        todo!()
    }

    // TODO: on('disconnected')
}

pub struct BrowserContext;

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

    pub async fn close(&self) {
        todo!()
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

pub struct Page;

impl Page {
    pub fn add_init_script(&self) {
        todo!()
    }

    pub fn add_locator_handler(&self) {
        todo!()
    }

    pub fn add_script_tag(&self) {
        todo!()
    }

    pub fn add_style_tag(&self) {
        todo!()
    }

    pub fn bring_to_front(&self) {
        todo!()
    }

    pub fn close(&self) {
        todo!()
    }

    pub fn console_messages(&self) {
        todo!()
    }

    pub fn content(&self) {
        todo!()
    }

    pub fn context(&self) {
        todo!()
    }

    pub fn drag_and_drop(&self) {
        todo!()
    }

    pub fn emulate_media(&self) {
        todo!()
    }

    pub fn evaluate(&self) {
        todo!()
    }

    pub fn evaluate_handle(&self) {
        todo!()
    }

    pub fn expose_binding(&self) {
        todo!()
    }

    pub fn expose_function(&self) {
        todo!()
    }

    pub fn frame(&self) {
        todo!()
    }

    pub fn frame_locator(&self) {
        todo!()
    }

    pub fn get_by_alt_text(&self) {
        todo!()
    }

    pub fn get_by_label(&self) {
        todo!()
    }

    pub fn get_by_placeholder(&self) {
        todo!()
    }

    pub fn get_by_role(&self) {
        todo!()
    }

    pub fn get_by_test_id(&self) {
        todo!()
    }

    pub fn get_by_text(&self) {
        todo!()
    }

    pub fn get_by_title(&self) {
        todo!()
    }

    pub fn go_back(&self) {
        todo!()
    }

    pub fn go_forward(&self) {
        todo!()
    }

    pub async fn goto(&self, _url: &str) {
        todo!()
    }

    pub fn is_closed(&self) {
        todo!()
    }

    pub fn locator(&self) {
        todo!()
    }

    pub fn main_frame(&self) {
        todo!()
    }

    pub fn opener(&self) {
        todo!()
    }

    pub fn page_errors(&self) {
        todo!()
    }

    pub fn pause(&self) {
        todo!()
    }

    pub fn pdf(&self) {
        todo!()
    }

    pub fn reload(&self) {
        todo!()
    }

    pub fn remove_all_listeners(&self) {
        todo!()
    }
    pub fn remove_locator_handler(&self) {
        todo!()
    }
    pub fn request_gc(&self) {
        todo!()
    }
    pub fn requests(&self) {
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
    pub fn screenshot(&self) {
        todo!()
    }
    pub fn set_content(&self) {
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
    pub fn set_viewport_size(&self) {
        todo!()
    }
    pub fn title(&self) {
        todo!()
    }
    pub fn unroute(&self) {
        todo!()
    }
    pub fn unroute_all(&self) {
        todo!()
    }
    pub fn url(&self) {
        todo!()
    }
    pub fn video(&self) {
        todo!()
    }
    pub fn viewport_size(&self) {
        todo!()
    }
    pub fn wait_for_event(&self) {
        todo!()
    }
    pub fn wait_for_function(&self) {
        todo!()
    }
    pub fn wait_for_load_state(&self) {
        todo!()
    }
    pub fn wait_for_request(&self) {
        todo!()
    }
    pub fn wait_for_response(&self) {
        todo!()
    }
    pub fn wait_for_url(&self) {
        todo!()
    }
    pub fn workers(&self) {
        todo!()
    }

    pub fn clock(&self) {
        todo!()
    }
    pub fn coverage(&self) {
        todo!()
    }
    pub fn keyboard(&self) {
        todo!()
    }
    pub fn mouse(&self) {
        todo!()
    }
    pub fn request(&self) {
        todo!()
    }
    pub fn touchscreen(&self) {
        todo!()
    }

    // TODO: on('close')
    // TODO: on('console')
    // TODO: on('crash')
    // TODO: on('dialog')
    // TODO: on('domcontentloaded')
    // TODO: on('download')
    // TODO: on('filechooser')
    // TODO: on('frameattached')
    // TODO: on('framedetached')
    // TODO: on('framenavigated')
    // TODO: on('load')
    // TODO: on('pageerror')
    // TODO: on('popup')
    // TODO: on('request')
    // TODO: on('requestfailed')
    // TODO: on('requestfinished')
    // TODO: on('response')
    // TODO: on('websocket')
    // TODO: on('worker')
}
