use elvis::{Error, FnBox};
use js_sys::{Function, Number};
use std::ops::{Deref, DerefMut};
use wasm_bindgen::prelude::*;
pub struct Func(pub Function);
deref!(Func, Function);

impl FnBox<Number> for Func {
    fn call(&mut self, p: &Number) -> Result<(), Error> {
        let r = self.call1(&JsValue::NULL, &p);
        if r.is_err() {
            return Err(Error::FunctionError(
                "call javascript function failed".to_string(),
            ));
        }
        Ok(())
    }
}
