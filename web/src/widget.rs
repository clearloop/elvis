// use elvis::Tree;
use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};
use wasm_bindgen::prelude::*;

/// mock web element
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Widget {
    tag: &'static str,
    pre: Option<Weak<RefCell<Widget>>>,
    attrs: HashMap<&'static str, String>,
    state: HashMap<&'static str, String>,
    children: Vec<Rc<RefCell<Widget>>>,
}
