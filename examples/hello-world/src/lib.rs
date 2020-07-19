use elvis::{
    prelude::*,
    widgets::{layouts::Center, Text, TextStyle},
};

#[page]
struct Index;

impl LifeCycle<Center> for Index {
    fn create(&self) -> Center {
        Center {
            child: Text {
                text: "Pink is the Pig!!!".into(),
                style: TextStyle::default(),
            }
            .into(),
        }
    }
}
