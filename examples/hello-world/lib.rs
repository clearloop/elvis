use elvis::{
    prelude::*,
    widgets::{layouts::Center, Text},
};

#[page]
struct Index;

impl LifeCycle for Index {
    fn create(&self) -> Node {
        Center::new()
            .child(Text::new().text("Pink is the Pig!!!"))
            .into()
    }
}
