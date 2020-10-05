use elvis::{
    gesture::{Gesture, GestureDetector},
    prelude::*,
    traits::DriverTrait,
    widgets::{layouts::Center, Text},
    Driver, State,
};

#[page]
struct Index;

impl LifeCycle for Index {
    fn create(&self) -> Node {
        // let text = Text::new().text("Pink is the Pig!!!");
        let mut state = State::new(Text::new().text("Pink is the Pig!!!"));
        state.set(b"hello", b"world");
        Center::with(
            GestureDetector::<State>::new(state).register(Gesture::Tap, |m| {
                Driver::alert("Hello from rust!");
                Driver::log(&format!("{:?}", m.get(&b"hello".to_vec())));
            }),
        )
        .into()
    }
}
