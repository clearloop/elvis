use elvis::{
    layouts::Center,
    prelude::*,
    widgets::{Text, TextStyle},
};

#[elvis]
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
