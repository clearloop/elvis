//! Elvis Router
use crate::Error;
use elvis_core::{Error as CoreError, Router as RouterTrait};
use wasm_bindgen::JsValue;
use web_sys::window;

/// Elvis Router
pub struct Router;

impl RouterTrait for Router {
    /// Push new pathl
    fn push(path: impl Into<String>) -> Result<(), CoreError> {
        let history = window().unwrap().history();
        CoreError::check::<_, Error, JsValue>(&history)?;
        CoreError::check::<_, Error, JsValue>(&history.clone().unwrap().push_state_with_url(
            &JsValue::NULL,
            "",
            Some(&format!("{}.html", path.into())),
        ))?;
        CoreError::check::<_, Error, JsValue>(&history.unwrap().go())?;

        Ok(())
    }
}
