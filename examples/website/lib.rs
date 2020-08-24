use elvis::{
    prelude::*,
    style::traits::{JustifyContent, Margin},
    value::{layouts::FlexPosition, FontFamily, TextAlign, Unit, VecUnit},
    widgets::{
        layouts::{Center, Col, Row},
        Link, Text,
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
                Link::with(Text::with("Contribute"))
                    .href("https://github.com/elvisjs/elvis#help-wanted"),
                Link::with(Text::with("The Book")).href("https://elvisjs.github.io/book/"),
                Link::with(Text::with("Github")).href("https://github.com/elvisjs/elvis"),
            ])
            .justify_content(FlexPosition::SpaceAround)
            .margin(VecUnit(vec![Unit::Rem(2.0), Unit::Rem(10.0)])),
        ]))
        .margin(VecUnit(vec![Unit::Rem(0.0), Unit::Auto]))
    }
}
