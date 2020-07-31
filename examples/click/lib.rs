use elvis::{
    gesture::{Gesture, GestureDetector},
    prelude::*,
    style::TextStyle,
    traits::DriverTrait,
    widgets::{layouts::Center, Text},
    Driver,
};

#[page]
struct Index;

impl LifeCycle<Center> for Index {
    fn create(&self) -> Center {
        Center {
            child: GestureDetector::<Text>::new(Text {
                text: "Pink is the Pig!!!".into(),
                style: TextStyle::default(),
            })
            .register(Gesture::Tap, |_| Driver::alert("Hello from rust!"))
            .into(),
        }
    }
}
