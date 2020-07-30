use elvis::{
    driver::{Driver, WebDriver},
    gesture::{Gesture, GestureDetector},
    prelude::*,
    style::TextStyle,
    widgets::{layouts::Center, Text},
};

#[page]
struct Index;

impl LifeCycle<Center> for Index {
    fn create(&self) -> Center {
        Center {
            child: GestureDetector::<Text>::new(
                Text {
                    text: "Pink is the Pig!!!".into(),
                    style: TextStyle::default(),
                }
                .into(),
            )
            .register(Gesture::Tap, |_| WebDriver::alert("Hello from rust!"))
            .into(),
        }
    }
}
