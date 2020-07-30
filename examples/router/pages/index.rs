use elvis::{
    gesture::{Gesture, GestureDetector},
    prelude::*,
    style::TextStyle,
    traits::RouterTrait,
    widgets::{layouts::Center, Text},
    Router,
};

#[page]
struct Index;

impl LifeCycle<Center> for Index {
    fn create(&self) -> Center {
        Center {
            child: GestureDetector::<Text>::new(
                Text {
                    text: "Let's roll up for the magical mystery tour!!!".into(),
                    style: TextStyle::default(),
                }
                .into(),
            )
            .register(Gesture::Tap, |_| Router::push("back").unwrap())
            .into(),
        }
    }
}
