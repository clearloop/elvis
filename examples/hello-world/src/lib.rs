use elvis::{
    prelude::elvis,
    widgets::{layouts::Center, Text, TextStyle},
    LifeCycle,
};
use elvis_web::Widget;
use wasm_bindgen::prelude::*;

#[elvis]
struct Index;

impl LifeCycle<Center> for Index {
    fn create(&self) -> Center {
        Center {
            child: Text {
                text: "Pink is the Pig!!!".into(),
                style: TextStyle::default(),
            }
            .into(),
        }
    }
}
