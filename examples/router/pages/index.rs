use elvis::{
    gesture::{Gesture, GestureDetector},
    prelude::*,
    traits::RouterTrait,
    widgets::{layouts::Center, Text},
    Router,
};

#[page]
struct Index;

impl LifeCycle for Index {
    fn create(&self) -> Node {
        Center::with(
            GestureDetector::<Text>::new(
                Text::new().text("Let's roll up for the magical mystery tour!!!"),
            )
            .register(Gesture::Tap, |_| Router::push("back").unwrap()),
        )
        .into()
    }
}
