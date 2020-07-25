use elvis::{
    driver::{Driver, WebDriver},
    gesture::{Gesture, GestureDetector},
    prelude::*,
    style::TextStyle,
    widgets::{layouts::Center, Text},
};

#[page]
struct Index;

impl LifeCycle<GestureDetector<Center>> for Index {
    fn create(&self) -> GestureDetector<Center> {
        let center = Center {
            child: Text {
                text: "Pink is the Pig!!!".into(),
                style: TextStyle::default(),
            }
            .into(),
        };

        let mut detector = GestureDetector::new(center);
        detector.register(Gesture::Tap, |_| WebDriver::alert("Hello from rust!"));
        detector
    }
}
