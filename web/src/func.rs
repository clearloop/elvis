use elvis::{Error, FnBox};
use js_sys::Function;
use std::ops::{Deref, DerefMut};
use wasm_bindgen::prelude::JsValue;

pub struct Func(pub Function);
deref!(Func, Function);

impl FnBox for Func {
    fn call(&mut self) -> Result<(), Error> {
        let r = self.call0(&JsValue::NULL);
        if r.is_err() {
            return Err(Error::FunctionError(
                "call javascript function failed".to_string(),
            ));
        }
        Ok(())
    }
}
