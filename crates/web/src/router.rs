//! Elvis Router
use crate::Page;
use std::collections::HashMap;
use wasm_bindgen::JsValue;
use web_sys::window;

/// Elvis Router
pub struct Router {
    /// Elvis pages
    pages: HashMap<String, Page>,
}

impl Router {
    /// Back to last page
    pub fn back() -> Result<(), JsValue> {
        window().unwrap().history()?.back()?;
        Ok(())
    }

    /// Push new path
    pub fn push(path: &str, title: &str) -> Result<(), JsValue> {
        let history = window().unwrap().history()?;
        history.push_state_with_url(&JsValue::NULL, title, Some(path))?;
        Ok(())
    }

    /// Replace current route
    pub fn replace(path: &str, title: &str) -> Result<(), JsValue> {
        let history = window().unwrap().history()?;
        history.replace_state_with_url(&JsValue::NULL, title, Some(path))?;
        Ok(())
    }

    /// Register page to name
    pub fn register(&mut self, path: String, page: Page) {
        self.pages.insert(path, page);
    }
}
