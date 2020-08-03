use elvis::{
    prelude::*,
    style::{SizedBoxStyle, TextStyle},
    value::{Color, FontFamily, TextAlign, Unit},
    widgets::{
        layouts::{Col, SizedBox},
        Text, TextField,
    },
};

#[page]
struct Index;

fn title() -> Text {
    Text {
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
            ]))
            .align(TextAlign::Center),
    }
    .into()
}

fn input() -> TextField {
    TextField::new().text(Text::new().text("hello".to_string()))
}

impl LifeCycle<SizedBox> for Index {
    fn create(&self) -> SizedBox {
        SizedBox::new()
            .child(
                Col {
                    children: vec![title().into(), input().into()],
                }
                .into(),
            )
            .style(SizedBoxStyle::new().max_width(Unit::Px(550.0)))
    }
}
