use elvis::{
    driver::{Driver, WebDriver},
    gesture::{Gesture, GestureDetector},
    prelude::*,
    style::TextStyle,
    widgets::{layouts::Center, Text},
};

#[page]
struct Hello;

impl LifeCycle<GestureDetector<Center>> for Hello {
    fn create(&self) -> GestureDetector<Center> {
        let center = Center {
            child: Text {
                text: "Hello!!!".into(),
                style: TextStyle::default(),
            }
            .into(),
        };

        GestureDetector::new(center)
            .register(Gesture::Tap, |_| WebDriver::alert("Hello from rust!"))
    }
}
