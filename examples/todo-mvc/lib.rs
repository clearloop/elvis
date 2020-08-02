use elvis::{
    prelude::*,
    style::{FlexStyle, TextStyle},
    value::{layouts::Alignment, Color, FontFamily, Unit},
    widgets::{
        layouts::{Col, Flex},
        Text, TextField,
    },
};

#[page]
struct Index;

fn title() -> Flex {
    Flex {
        child: Text {
            text: "todos".into(),
            style: TextStyle::new()
                .color(Color::ORGB(0.15, 175, 47, 47))
                .size(Unit::Px(100.0))
                .weight(Unit::None(100.0))
                .family(FontFamily::Derive(vec![
                    FontFamily::Mix(Box::new(FontFamily::Helvetica), Box::new(FontFamily::Neue)),
                    FontFamily::Helvetica,
                    FontFamily::Neue,
                    FontFamily::Arial,
                ])),
        }
        .into(),
        style: FlexStyle::new().align(Alignment::TopCenter),
    }
}

fn input() -> TextField {
    TextField::new().text(Text::new().text("hello".to_string()))
}

impl LifeCycle<Col> for Index {
    fn create(&self) -> Col {
        Col {
            children: vec![title().into(), input().into()],
        }
    }
}
