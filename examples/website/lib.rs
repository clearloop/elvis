use elvis::{
    prelude::*,
    style::traits::{JustifyContent, Margin, MarginTop},
    value::{layouts::FlexPosition, FontFamily, TextAlign, Unit, VecUnit},
    widgets::{
        layouts::{Center, Col, Row},
        Text,
    },
};

#[page]
struct Index;

pub fn font() -> FontFamily {
    FontFamily::Derive(vec![FontFamily::Mix(
        Box::new(FontFamily::Helvetica),
        Box::new(FontFamily::Neue),
    )])
}

impl LifeCycle for Index {
    fn create(&self) -> Node {
        Center::with(Col::with(vec![
            Text::with("Elvis . JS")
                .size(Unit::Rem(16.0))
                .family(font())
                .align(TextAlign::Center)
                .margin(VecUnit(vec![Unit::Rem(2.0)])),
            Row::with(vec![
                Text::with("Contribute"),
                Text::with("The Book"),
                Text::with("Github"),
            ])
            .justify_content(FlexPosition::SpaceAround)
            .margin_top(Unit::Rem(5.0)),
        ]))
        .margin(VecUnit(vec![Unit::Rem(0.0), Unit::Auto]))
    }
}
