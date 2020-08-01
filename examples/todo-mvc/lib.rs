use elvis::{
    prelude::*,
    style::{FlexStyle, TextStyle},
    value::{layouts::Alignments, Colors, FontFamily, Unit},
    widgets::{layouts::Flex, Text},
};

#[page]
struct Index;

impl LifeCycle<Flex> for Index {
    fn create(&self) -> Flex {
        Flex {
            child: Text {
                text: "todos".into(),
                style: TextStyle::new()
                    .color(Colors::ORGB(0.15, 175, 47, 47))
                    .size(Unit::Px(100.0))
                    .weight(Unit::None(100.0))
                    .family(vec![
                        FontFamily::Mix(
                            Box::new(FontFamily::Helvetica),
                            Box::new(FontFamily::Neue),
                        ),
                        FontFamily::Helvetica,
                        FontFamily::Neue,
                        FontFamily::Arial,
                    ]),
            }
            .into(),
            style: FlexStyle::new().align(Alignments::TopCenter),
        }
    }
}
