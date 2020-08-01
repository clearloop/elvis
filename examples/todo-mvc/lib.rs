use elvis::{
    prelude::*,
    style::{FlexStyle, TextStyle},
    value::{layouts::Alignments, Colors},
    widgets::{layouts::Flex, Text},
};

#[page]
struct Index;

impl LifeCycle<Flex> for Index {
    fn create(&self) -> Flex {
        Flex {
            child: Text {
                text: "todos".into(),
                style: TextStyle::new().color(Colors::ORGB(0.15, 175, 47, 47)),
            }
            .into(),
            style: FlexStyle::new().align(Alignments::TopCenter),
        }
    }
}
