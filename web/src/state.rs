use crate::{Func, StyleSheet, Widget};
use elvis::{Serde, State as ElvisState};
use js_sys::{Function, Map, Number, Object};
use std::{cell::RefCell, collections::HashSet, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast};

/// state manager
#[wasm_bindgen]
pub struct State {
    es: Rc<RefCell<ElvisState<Widget, Number>>>,
    this: JsValue,
}

#[wasm_bindgen]
impl State {
    #[wasm_bindgen(constructor)]
    pub fn new(context: JsValue, widget: Widget, trigger: Function) -> State {
        State {
            es: Rc::new(RefCell::new(ElvisState::new(
                widget,
                Box::new(Func(trigger)),
            ))),
            this: context,
        }
    }

    #[wasm_bindgen(js_name = "migrateState")]
    pub fn migrate_state(&mut self, obj: Object, init: bool) {
        let map = obj.dyn_into::<Map>().unwrap_or(Map::new());
        let mut bm = self.es.borrow_mut();
        map.for_each(&mut |k, v| {
            bm.set(
                &k.as_string().unwrap_or("".to_string()),
                &v.as_string().unwrap_or("".to_string()),
            );
        });

        if init.eq(&false) {
            self.es.borrow_mut().process(&Number::from(1)).unwrap_or(());
        }
    }

    pub fn calling(&mut self) -> Result<(), JsValue> {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let html = document.query_selector("html")?.unwrap();

        // set style
        let style = document.create_element("style")?;
        let mut stylesheet = StyleSheet::new();
        stylesheet.0 += &self.style();
        style.set_inner_html(&stylesheet.0);
        html.append_child(&style)?;

        // set body
        let body = document.query_selector("body")?.unwrap();
        body.set_inner_html(&self.ser());
        Ok(())
    }
}

impl State {
    pub fn get_this<'c>(&self) -> &JsValue {
        &self.this
    }

    pub fn ser(&self) -> String {
        self.es.borrow().widget.ser()
    }

    pub fn style(&mut self) -> String {
        self.es.borrow_mut().process(&Number::from(0)).unwrap_or(());
        StyleSheet::batch(&mut self.es.borrow_mut().widget, &mut HashSet::new())
            .trim()
            .to_string()
    }
}
