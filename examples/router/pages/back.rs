use elvis::{
    gesture::{Gesture, GestureDetector},
    prelude::*,
    style::TextStyle,
    traits::RouterTrait,
    widgets::{layouts::Center, Text},
    Router,
};

#[page]
struct Back;

impl LifeCycle<Center> for Back {
    fn create(&self) -> Center {
        Center {
            child: GestureDetector::<Text>::new(
                Text {
                    text: "Back to the Future!!!".into(),
                    style: TextStyle::default(),
                }
                .into(),
            )
            .register(Gesture::Tap, |_| Router::push("index").unwrap())
            .into(),
        }
    }
}
