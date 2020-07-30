//! Elvis Router
use crate::Error;
use elvis_core::{Error as CoreError, Router as RouterTrait};
use wasm_bindgen::JsValue;
use web_sys::window;

/// Elvis Router
pub struct Router;

impl RouterTrait for Router {
    /// Back to last page
    fn back() -> Result<(), CoreError> {
        let history = window().unwrap().history();
        CoreError::check::<_, Error, JsValue>(&history)?;
        CoreError::check::<_, Error, JsValue>(&history.unwrap().back())?;
        Ok(())
    }

    /// Push new path
    fn push(path: &str) -> Result<(), CoreError> {
        let history = window().unwrap().history();
        CoreError::check::<_, Error, JsValue>(&history)?;
        CoreError::check::<_, Error, JsValue>(&history.unwrap().push_state_with_url(
            &JsValue::NULL,
            "",
            Some(path),
        ))?;

        Ok(())
    }

    // /// Replace current route
    // pub fn replace(path: &str, title: &str) -> Result<(), JsValue> {
    //     let history = window().unwrap().history()?;
    //     history.replace_state_with_url(&JsValue::NULL, title, Some(path))?;
    //     Ok(())
    // }
    //
    // /// Register page to name
    // pub fn register(&mut self, path: String, page: Page) {
    //     self.pages.insert(path, page);
    // }
}
