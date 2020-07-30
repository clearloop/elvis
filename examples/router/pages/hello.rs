use elvis::{
    gesture::{Gesture, GestureDetector},
    prelude::*,
    style::TextStyle,
    traits::DriverTrait,
    widgets::{layouts::Center, Text},
    Driver,
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

        GestureDetector::new(center).register(Gesture::Tap, |_| Driver::alert("Hello from rust!"))
    }
}
