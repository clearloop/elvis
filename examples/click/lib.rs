use elvis::{
    gesture::{Gesture, GestureDetector},
    prelude::*,
    traits::DriverTrait,
    widgets::{layouts::Center, Text},
    Driver,
};

#[page]
struct Index;

impl LifeCycle for Index {
    fn create(&self) -> Node {
        Center::new()
            .child(
                GestureDetector::<Text>::new(Text::new().text("Pink is the Pig!!!"))
                    .register(Gesture::Tap, |_| Driver::alert("Hello from rust!")),
            )
            .into()
    }
}
