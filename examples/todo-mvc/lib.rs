use elvis::{
    prelude::*,
    style::TextStyle,
    value::Colors,
    widgets::{layouts::Center, Text},
};

#[page]
struct Index;

impl LifeCycle<Center> for Index {
    fn create(&self) -> Center {
        Center {
            child: Text {
                text: "todos".into(),
                style: TextStyle {
                    bold: true,
                    color: Colors::PinkAccent,
                    italic: false,
                    stretch: Default::default(),
                    size: Default::default(),
                    weight: Default::default(),
                    height: Default::default(),
                },
            }
            .into(),
        }
    }
}
